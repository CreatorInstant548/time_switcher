# spec: carnet architecture-securite.md §9.16 VZ4, VZ5, VZ7 (AS-48)
"""Tests of the optional converter positions_to_rrd.py (group `viz`).

Not part of `cargo xtask py` nor of the CI (`testpaths` is `tests`). Run by hand:

    uv sync --locked --group viz        # in python/, with the flags of docs/dev/visualisation.md
    uv run --no-sync pytest viz -p no:cacheprovider

The input is built here with pyarrow, in the layout of crates/ts_viz (the Rust side has its own
tests); nothing is written outside the temporary directory.
"""

import ast
import os
import socket
import sys
from pathlib import Path

import pytest

pa = pytest.importorskip("pyarrow")
pq = pytest.importorskip("pyarrow.parquet")

sys.path.insert(0, str(Path(__file__).resolve().parent))
import positions_to_rrd as conv  # noqa: E402

# Same guard as the script (VZ7): `rerun` is never imported while its analytics are not opted
# out in the configuration of the user. Nothing here changes that configuration.
if not conv.analytics_disabled():
    pytest.skip(
        "the Rerun analytics are not opted out (see docs/dev/visualisation.md)",
        allow_module_level=True,
    )
rr = pytest.importorskip("rerun")
from rerun.chunk import RrdReader  # noqa: E402

DAYS = 3
RESIDENTS = 6
META = {
    "format": "1",
    "recording_id": "b1fd6e5958536daffa18fbd269cb5cd4",
    "days": str(DAYS),
    "grid_width": "4",
    "grid_height": "4",
}


def write_parquet(path: Path, **overrides) -> None:
    """A positions.parquet like the one of ts-viz: DAYS days of RESIDENTS residents."""
    rows = [(d, i) for d in range(DAYS) for i in range(RESIDENTS)]
    columns = {
        "day": [d for d, _ in rows],
        "tick": [d * 1000 + 576 for d, _ in rows],
        "id": [i for _, i in rows],
        "parcel": [(d + i) % 16 for d, i in rows],
        "x": [((d + i) % 16) % 4 for d, i in rows],
        "y": [((d + i) % 16) // 4 for d, i in rows],
        "place": [conv.PLACES[(d + i) % 4] for d, i in rows],
    }
    columns.update(overrides)
    fields = [pa.field(n, pa.int64(), nullable=False) for n in conv.INT_COLUMNS]
    fields.append(pa.field("place", pa.string(), nullable=False))
    meta = {f"ts_viz.{k}": v for k, v in META.items()}
    table = pa.table(columns, schema=pa.schema(fields, metadata=meta))
    pq.write_table(table, path, compression="none")


@pytest.fixture
def cache(tmp_path, monkeypatch):
    root = tmp_path / "cache" / "ts_viz_test_cache"
    root.mkdir(parents=True)
    monkeypatch.setenv("TS_CACHE", str(root))
    return root.resolve()


def canonical(rrd: Path) -> dict:
    """Decoded content by entity: rows by day, without the row identifiers."""
    out: dict[str, list] = {}
    for chunk in RrdReader(rrd).stream().to_chunks():
        for row in chunk.to_record_batch().to_pylist():
            row.pop("rerun.controls.RowId", None)
            out.setdefault(chunk.entity_path, []).append(repr(sorted(row.items())))
    return {path: sorted(rows) for path, rows in out.items()}


def convert(cache: Path, name: str) -> Path:
    source = cache / f"{name}.parquet"
    write_parquet(source)
    out = cache / f"{name}.rrd"
    positions = conv.read_positions(source)
    assert conv.convert(positions, out) == DAYS
    return out


def test_two_conversions_decode_to_the_same_data_without_wall_clock(cache):
    first, second = convert(cache, "a"), convert(cache, "b")
    data = canonical(first)
    assert data == canonical(second)
    assert set(data) == {
        "/town", "/town/bounds", "/town/residents",
        "/counts/home", "/counts/to_work", "/counts/work", "/counts/to_home",
    }  # fmt: skip
    # Two simulated time lines only: no `log_time` (wall clock) is injected.
    assert all("log_time" not in row for rows in data.values() for row in rows)
    assert len(data["/counts/home"]) == DAYS
    # The recording id of the file is the one of the recording.
    store = RrdReader(first).recordings()[0]
    assert str(store.recording_id).replace("-", "") == META["recording_id"]


def test_the_recording_holds_no_local_path_user_or_host(cache):
    data = convert(cache, "a").read_bytes()
    forbidden = ["TS_CACHE", "HOME", str(cache), cache.name, socket.gethostname()]
    for name in ("USERNAME", "USER", "COMPUTERNAME", "LOGNAME"):
        if len(os.environ.get(name, "")) >= 3:
            forbidden.append(os.environ[name])
    for needle in forbidden:
        assert needle.encode() not in data, "a local string is in the recording"


def test_refused_inputs_and_outputs(cache, tmp_path):
    good = cache / "good.parquet"
    write_parquet(good)
    # Outside TS_CACHE, `..`, TS_CACHE itself, existing output, not a .rrd.
    outside = tmp_path / "outside.parquet"
    write_parquet(outside)
    for path in (outside, cache / ".." / "good.parquet", cache):
        with pytest.raises(conv.VizError):
            conv.under_cache(str(path), conv.cache_root(), "input")
    assert conv.main([str(outside), "--out", str(cache / "x.rrd")]) == 1
    assert conv.main([str(good), "--out", str(tmp_path / "x.rrd")]) == 1
    assert conv.main([str(good), "--out", str(cache / "x.txt")]) == 1
    (cache / "x.rrd").write_bytes(b"")
    assert conv.main([str(good), "--out", str(cache / "x.rrd")]) == 1
    assert (cache / "x.rrd").read_bytes() == b""


@pytest.mark.parametrize(
    "overrides",
    [
        {"x": [4] * (DAYS * RESIDENTS)},  # outside the grid
        {"day": [DAYS] * (DAYS * RESIDENTS)},  # beyond the days of the metadata
        {"place": ["office"] * (DAYS * RESIDENTS)},  # unknown place
    ],
)
def test_values_outside_the_bounds_are_refused(cache, overrides):
    source = cache / "bad.parquet"
    write_parquet(source, **overrides)
    with pytest.raises(conv.VizError):
        conv.read_positions(source)


def test_wrong_schema_and_not_parquet_are_refused(cache):
    junk = cache / "junk.parquet"
    junk.write_bytes(b"not parquet")
    with pytest.raises(conv.VizError):
        conv.read_positions(junk)
    wrong = cache / "wrong.parquet"
    pq.write_table(pa.table({"day": [0]}), wrong)
    with pytest.raises(conv.VizError):
        conv.read_positions(wrong)


def test_environment_is_scrubbed_before_rerun_is_imported():
    env = {"RERUN": "off", "RERUN_CLI_PATH": "x", "FORCE_RERUN_ANALYTICS": "1", "OTEL_X": "1",
           "TELEMETRY_ENABLED": "true", "PATH": "p"}  # fmt: skip
    expected = sorted(set(env) - {"PATH"})
    assert sorted(conv.harden_environment(env)) == expected
    # The SDK's own switch is set explicitly, not just erased.
    assert env == {"PATH": "p", "TELEMETRY_ENABLED": "false"}


def test_the_script_never_spawns_connects_serves_or_shows():
    source = Path(conv.__file__).read_text(encoding="utf-8")
    forbidden = {"spawn", "connect", "connect_grpc", "connect_grpc_blueprint", "serve",
                 "serve_grpc", "serve_web", "show", "notebook_show", "web_viewer", "run",
                 "Popen", "call", "check_call", "check_output", "system"}  # fmt: skip
    called = {
        node.func.attr
        for node in ast.walk(ast.parse(source))
        if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)
    }
    assert not called & forbidden
    # `rr.init` is told not to spawn the viewer, and `rr.save` is the only sink.
    assert "spawn=False" in source.replace(" ", "")
    assert "rr.save(" in source
    # No program is run: not the Rerun CLI either (its opt-out command records an event).
    assert "subprocess" not in source


def test_a_failed_conversion_leaves_no_partial_recording(cache, monkeypatch):
    source = cache / "a.parquet"
    write_parquet(source)

    def boom(*_args, **_kwargs):
        raise RuntimeError("boom")

    monkeypatch.setattr(rr, "send_columns", boom)
    assert conv.main([str(source), "--out", str(cache / "a.rrd")]) == 1
    assert not (cache / "a.rrd").exists()


def test_a_compressed_file_is_refused(cache):
    source = cache / "zstd.parquet"
    write_parquet(source)
    table = pq.read_table(source)
    pq.write_table(table, source, compression="zstd")
    with pytest.raises(conv.VizError):
        conv.read_positions(source)


def test_a_successful_run_leaves_only_the_recording(cache):
    source = cache / "a.parquet"
    write_parquet(source)
    assert conv.main([str(source), "--out", str(cache / "a.rrd")]) == 0
    assert sorted(p.name for p in cache.iterdir()) == ["a.parquet", "a.rrd"]
    assert canonical(cache / "a.rrd")


READ_ONLY = """
import sys
from pathlib import Path
import pyarrow as pa
sys.path.insert(0, sys.argv[1])
import positions_to_rrd as conv

try:
    conv.read_positions(Path(sys.argv[2]))
    print("accepted")
except conv.VizError as error:
    print("refused", error)
print(pa.default_memory_pool().max_memory())
"""


def write_dictionary_bomb(path: Path) -> None:
    """The file of the review of PR 97: 3 row groups of 1,000 rows, one 100 KiB string."""
    fields = [pa.field(n, pa.int64(), nullable=False) for n in conv.INT_COLUMNS]
    fields.append(pa.field("place", pa.string(), nullable=False))
    meta = {f"ts_viz.{k}": v for k, v in META.items()}
    schema = pa.schema(fields, metadata=meta)
    writer = pq.ParquetWriter(path, schema, compression="none", use_dictionary=True)
    for g in range(3):
        cols = {n: [g] * 1000 if n == "day" else [0] * 1000 for n in conv.INT_COLUMNS}
        cols["place"] = ["h" * 102_400] * 1000
        writer.write_table(pa.table(cols, schema=schema))
    writer.close()


def test_a_dictionary_bomb_is_refused_before_it_is_decoded(tmp_path):
    # About 312 KB on disk; decoded, 3,000 rows of 100 KiB are 420 MB. It is refused from the
    # footer (size of the column chunk), and the Arrow pool of the reader stays tiny.
    import subprocess

    bomb = tmp_path / "bomb.parquet"
    write_dictionary_bomb(bomb)
    assert bomb.stat().st_size < 400_000
    args = [sys.executable, "-I", "-c", READ_ONLY, str(Path(conv.__file__).parent), str(bomb)]
    out = subprocess.run(args, capture_output=True, text=True, timeout=300, check=True)
    verdict, peak = out.stdout.splitlines()[-2:]
    assert verdict.startswith("refused"), verdict
    assert str(tmp_path) not in out.stdout
    assert int(peak) < 5 * 1024 * 1024


def test_a_dictionary_with_unknown_values_is_refused(cache):
    # Small enough for the footer bounds: the check of the dictionary values refuses it.
    source = cache / "dict.parquet"
    write_parquet(source, place=["office"] * (DAYS * RESIDENTS))
    with pytest.raises(conv.VizError, match="unknown place"):
        conv.read_positions(source)


@pytest.mark.parametrize("error", [MemoryError(), pa.ArrowInvalid("corrupt"), OSError("disk")])
def test_read_errors_become_path_free_viz_errors(cache, monkeypatch, error):
    source = cache / "a.parquet"
    write_parquet(source)

    def boom(*_args, **_kwargs):
        raise error

    monkeypatch.setattr(pq.ParquetFile, "read", boom)
    with pytest.raises(conv.VizError) as caught:
        conv.read_positions(source)
    assert str(cache) not in str(caught.value)
    assert conv.main([str(source), "--out", str(cache / "b.rrd")]) == 1
