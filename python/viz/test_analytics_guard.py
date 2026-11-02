# spec: carnet architecture-securite.md §9.16 VZ7 (AS-48)
"""Tests of the opt-out guard of positions_to_rrd.py: the Rerun analytics must be opted out
BEFORE `rerun` is imported. This file never imports `rerun`, and never touches the real
configuration of the user: it works on a temporary `analytics.json`.

Run by hand: `uv run --no-sync pytest viz -p no:cacheprovider` (docs/dev/visualisation.md).
"""

import json
import subprocess
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import positions_to_rrd as conv  # noqa: E402

# What `rerun analytics config` shows for a user who opted out (0.38.1).
CONFIG = {
    "analytics_enabled": True,
    "analytics_id": "00000000000000000000000000000000",
    "metadata": {},
    "config_file_path": "x",
    "data_dir_path": "y",
}


def write(path: Path, **changes) -> bytes:
    path.write_text(json.dumps({**CONFIG, **changes}, separators=(",", ":")), encoding="utf-8")
    return path.read_bytes()


def test_an_opted_out_configuration_is_left_untouched(tmp_path):
    path = tmp_path / "analytics.json"
    before = write(path, analytics_enabled=False)
    assert conv.analytics_disabled(path)
    conv.ensure_analytics_disabled(path)
    assert path.read_bytes() == before
    assert list(tmp_path.iterdir()) == [path]


def test_an_enabled_configuration_gets_only_its_flag_changed(tmp_path):
    path = tmp_path / "analytics.json"
    write(path)
    assert not conv.analytics_disabled(path)
    conv.ensure_analytics_disabled(path)
    assert json.loads(path.read_text(encoding="utf-8")) == {**CONFIG, "analytics_enabled": False}
    assert conv.analytics_disabled(path)
    assert list(tmp_path.iterdir()) == [path], "no temporary file stays"


@pytest.mark.parametrize(
    "text",
    [None, "", "not json", "[]", '{"analytics_id":"x"}', '{"analytics_enabled":"no"}', "x" * 70_000],
    ids=["missing", "empty", "not-json", "list", "no-flag", "flag-not-bool", "too-large"],
)
def test_without_a_usable_configuration_the_script_stops(tmp_path, text):
    path = tmp_path / "analytics.json"
    if text is not None:
        path.write_text(text, encoding="utf-8")
    assert not conv.analytics_disabled(path)
    with pytest.raises(conv.VizError) as caught:
        conv.ensure_analytics_disabled(path)
    assert str(tmp_path) not in str(caught.value)
    # Nothing was created or rewritten.
    assert sorted(p.name for p in tmp_path.iterdir()) == ([] if text is None else ["analytics.json"])
    if text is not None:
        assert path.read_text(encoding="utf-8") == text


def test_the_check_runs_before_rerun_is_imported(tmp_path, monkeypatch):
    """`main` stops on a configuration that is not opted out, with `rerun` never imported."""
    cache = tmp_path / "cache" / "ts_viz_guard"
    cache.mkdir(parents=True)
    source = cache / "a.parquet"
    source.write_bytes(b"x")
    config = tmp_path / "analytics.json"
    write(config, analytics_enabled="maybe")
    code = (
        "import sys\n"
        f"sys.path.insert(0, {str(Path(conv.__file__).parent)!r})\n"
        "import positions_to_rrd as conv\n"
        f"conv.analytics_config_path = lambda: __import__('pathlib').Path({str(config)!r})\n"
        f"status = conv.main([{str(source)!r}, '--out', {str(cache / 'a.rrd')!r}])\n"
        "print(status, 'rerun' in sys.modules, 'rerun_bindings' in sys.modules)\n"
    )
    monkeypatch.setenv("TS_CACHE", str(cache))
    out = subprocess.run(
        [sys.executable, "-I", "-c", code], capture_output=True, text=True, timeout=300, check=True,
        env={**__import__("os").environ, "TS_CACHE": str(cache)},
    )  # fmt: skip
    assert out.stdout.split() == ["1", "False", "False"]
    assert not (cache / "a.rrd").exists() and not (cache / "a.rrd.part").exists()
