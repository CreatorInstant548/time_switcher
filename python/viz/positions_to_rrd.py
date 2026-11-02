# spec: carnet architecture-securite.md §9.16 VZ4, VZ5, VZ7 (AS-48)
"""Converts `positions.parquet` (crate ts_viz) into a Rerun recording (`.rrd`).

OPTIONAL TOOL, group `viz` of python/pyproject.toml: installed neither by `cargo xtask py`
nor by the CI. How to use it: docs/dev/visualisation.md.

    uv run --no-sync python -I viz/positions_to_rrd.py <positions.parquet> --out <positions.rrd>

What it does, and nothing else (VZ7):
- reads the Parquet file with pyarrow (a dependency of rerun-sdk, nothing more is installed);
- writes the recording with `rr.save(<file under TS_CACHE>)`: never `spawn`, `connect_grpc`,
  `serve` or `show`, and it never starts the Rerun viewer or its CLI. Open the .rrd in the
  viewer you installed apart;
- requires, before `rerun` is imported, that the Rerun analytics are opted out in the
  configuration file of the user (`analytics_enabled` is false), and stops otherwise (see
  `ensure_analytics_disabled`);
- removes from its environment every variable that could redirect Rerun (`RERUN*`,
  `FORCE_RERUN_ANALYTICS`, `OTEL_*`, `TELEMETRY_*`) and sets `TELEMETRY_ENABLED=false`.

Input and output are strictly under TS_CACHE (`C:/Dev/ts-cache` on Windows, else
`~/.cache/time_switcher`), as for `ts-sim` and `ts-viz` (Y3); the output must not exist yet.
The input is untrusted data: size, footer, size of each column chunk, schema, row counts,
dictionary and values are checked before they are decoded or used (VZ5: at most 3660 days, and
the residents of the toy town).
"""

from __future__ import annotations

import argparse
import json
import os
import secrets
import sys
import uuid
from pathlib import Path

# Bounds, the same as the Rust side (ts-viz, Y4 and VZ5). Bounds of the tool, not game values.
MAX_DAYS = 3660
MAX_RESIDENTS = 1000
MAX_ROWS = MAX_DAYS * MAX_RESIDENTS
MAX_FILE_BYTES = 512 * 1024 * 1024
FORMAT = "1"
META_PREFIX = "ts_viz."
INT_COLUMNS = ("day", "tick", "id", "parcel", "x", "y")
PLACES = ("home", "to_work", "work", "to_home")
# One colour per place (RGB), in the order of PLACES.
PLACE_COLORS = ((110, 110, 110), (240, 170, 30), (50, 130, 220), (50, 190, 120))
APP_ID = "time_switcher_positions"
# Variables that could redirect or enable Rerun (the CLI to run, its endpoints, its logging).
SCRUBBED_PREFIXES = ("RERUN", "FORCE_RERUN", "OTEL_", "TELEMETRY_")
DEFAULT_CACHE_WINDOWS = "C:/Dev/ts-cache"
DEFAULT_CACHE_UNDER_HOME = ".cache/time_switcher"


class VizError(Exception):
    """A refused input or output. The message never carries a local path."""


def scrub_environment(environ: dict[str, str] | None = None) -> list[str]:
    """Removes the variables of `SCRUBBED_PREFIXES`; returns their names."""
    env = os.environ if environ is None else environ
    names = [n for n in env if n.upper().startswith(SCRUBBED_PREFIXES)]
    for name in names:
        del env[name]
    return names


def harden_environment(environ: dict[str, str] | None = None) -> list[str]:
    """Scrubs the environment, then sets the SDK's own telemetry switch explicitly.

    `TELEMETRY_ENABLED` is the variable the SDK reads for its OpenTelemetry tracing (active only
    when it is `true` and an OTLP endpoint is set, see `rerun/_tracing.py`): after the scrub it
    is set to `false`, so that the opt-out of the caller is not silently lost. The analytics of
    `re_analytics` have no such variable in 0.38.1 (only `FORCE_RERUN_ANALYTICS`, which forces
    them on in debug builds): they depend on the configuration file, see
    `ensure_analytics_disabled`.
    """
    env = os.environ if environ is None else environ
    names = scrub_environment(env)
    env["TELEMETRY_ENABLED"] = "false"
    return names


def _roaming_appdata() -> Path:
    """The Roaming AppData folder: the Windows known folder, not the `APPDATA` variable."""
    import ctypes
    from ctypes import wintypes

    class Guid(ctypes.Structure):
        _fields_ = [
            ("a", wintypes.DWORD),
            ("b", wintypes.WORD),
            ("c", wintypes.WORD),
            ("d", ctypes.c_ubyte * 8),
        ]

    tail = (ctypes.c_ubyte * 8)(0xA0, 0x3A, 0xE3, 0xEF, 0x65, 0x72, 0x9F, 0x3D)
    guid = Guid(0x3EB685DB, 0x65F9, 0x4CF6, tail)  # FOLDERID_RoamingAppData
    pointer = ctypes.c_void_p()
    shell32 = ctypes.windll.shell32  # type: ignore[attr-defined]
    shell32.SHGetKnownFolderPath.argtypes = [
        ctypes.c_void_p,
        wintypes.DWORD,
        wintypes.HANDLE,
        ctypes.POINTER(ctypes.c_void_p),
    ]
    if shell32.SHGetKnownFolderPath(ctypes.byref(guid), 0, None, ctypes.byref(pointer)) != 0:
        raise VizError("cannot locate the Rerun analytics configuration")
    try:
        return Path(ctypes.wstring_at(pointer))
    finally:
        ctypes.windll.ole32.CoTaskMemFree(pointer)  # type: ignore[attr-defined]


def analytics_config_path() -> Path:
    """Where Rerun keeps `analytics.json` (layout of the `directories` crate, project `rerun`).

    Verified on Windows only (`rerun analytics config` prints this path there). The macOS and
    Linux paths follow the documented layout of that crate and have not been run.
    """
    if sys.platform.startswith("win"):
        return _roaming_appdata() / "rerun" / "config" / "analytics.json"
    if sys.platform == "darwin":
        return Path.home() / "Library" / "Application Support" / "rerun" / "analytics.json"
    base = os.environ.get("XDG_CONFIG_HOME") or str(Path.home() / ".config")
    return Path(base) / "rerun" / "analytics.json"


MAX_CONFIG_BYTES = 64 * 1024


def _read_analytics_config(path: Path) -> dict | None:
    """The configuration as a dict, or `None` if the file does not exist."""
    try:
        if path.stat().st_size > MAX_CONFIG_BYTES:
            raise VizError("the Rerun analytics configuration is unreadable (too large)")
        config = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        return None
    except (OSError, ValueError):
        raise VizError("the Rerun analytics configuration is unreadable") from None
    if not isinstance(config, dict):
        raise VizError("the Rerun analytics configuration is unreadable")
    return config


def analytics_disabled(path: Path | None = None) -> bool:
    """True only if the configuration exists and says `analytics_enabled: false`."""
    try:
        config = _read_analytics_config(path or analytics_config_path())
    except VizError:
        return False
    return config is not None and config.get("analytics_enabled") is False


def ensure_analytics_disabled(path: Path | None = None) -> None:
    """Requires the Rerun analytics to be opted out BEFORE any Rerun code is imported (VZ7).

    The result is checked, not an exit code: the configuration is read and must say
    `analytics_enabled: false`. No Rerun program is run: the CLI `rerun analytics disable`
    records a `cli_command_invoked` usage event before it applies the opt-out.
    - already opted out: nothing is written;
    - the file exists and says `true`: only that key is changed, atomically (temporary file,
      then replace), and the file is read again and must say `false`;
    - no usable file: the script stops. The documented command `rerun analytics disable`, run
      once by hand, creates it (and queues one usage event: a residual of that command).
    """
    path = path or analytics_config_path()
    config = _read_analytics_config(path)
    if config is not None and config.get("analytics_enabled") is False:
        return
    if config is None or not isinstance(config.get("analytics_enabled"), bool):
        raise VizError(
            "the Rerun analytics are not opted out: run `rerun analytics disable` once "
            "(docs/dev/visualisation.md), then run this script again"
        )
    config["analytics_enabled"] = False
    temp = path.with_name(f"{path.name}.{secrets.token_hex(4)}.tmp")
    try:
        fd = os.open(temp, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            handle.write(json.dumps(config, separators=(",", ":")))
        os.replace(temp, path)
    except OSError:
        raise VizError("cannot write the Rerun analytics opt-out") from None
    finally:
        try:
            temp.unlink(missing_ok=True)
        except OSError:
            pass
    again = _read_analytics_config(path)
    if again is None or again.get("analytics_enabled") is not False:
        raise VizError("the Rerun analytics opt-out was not applied")


def cache_root() -> Path:
    """Real directory TS_CACHE (not a link, at least two levels below the root of the disk)."""
    value = os.environ.get("TS_CACHE")
    if value:
        cache = Path(value)
    elif sys.platform.startswith("win"):
        cache = Path(DEFAULT_CACHE_WINDOWS)
    else:
        cache = Path.home() / DEFAULT_CACHE_UNDER_HOME
    if cache.is_symlink() or not cache.is_dir():
        raise VizError("TS_CACHE refused: missing, a link, or not a directory")
    cache = cache.resolve(strict=True)
    if len(cache.parts) < 3:
        raise VizError("TS_CACHE refused: too close to the root of the disk")
    return cache


def under_cache(path: str, cache: Path, what: str) -> Path:
    """`path` made absolute, strictly under `cache`, with no `..` and no link on the way."""
    given = Path(path)
    if ".." in given.parts:
        raise VizError(f"{what} refused: `..` component")
    absolute = Path(os.path.abspath(given))
    try:
        relative = absolute.relative_to(cache)
    except ValueError:
        raise VizError(f"{what} is not strictly under TS_CACHE") from None
    if not relative.parts:
        raise VizError(f"{what} is not strictly under TS_CACHE")
    walk = cache
    for part in relative.parts:
        walk = walk / part
        if walk.is_symlink() or _is_reparse_point(walk):
            raise VizError(f"{what} refused: a link on the way")
    if absolute.resolve() != absolute and absolute.exists():
        raise VizError(f"{what} refused: not a canonical path")
    return absolute


def _is_reparse_point(path: Path) -> bool:
    """A junction or any other reparse point (Windows)."""
    try:
        attributes = path.lstat().st_file_attributes  # type: ignore[attr-defined]
    except (AttributeError, OSError):
        return False
    return bool(attributes & 0x400)


class Positions:
    """A validated `positions.parquet`: integer columns as numpy arrays, places as indexes."""

    def __init__(self, meta: dict[str, str], columns: dict[str, object], place: object) -> None:
        self.meta = meta
        self.columns = columns
        self.place = place


# Room for the headers and the dictionary page of a column chunk, and the most a row can take
# (8 bytes of an integer, or 4 of length and 7 of the longest place name, plus the index).
CHUNK_SLACK_BYTES = 4096
CHUNK_BYTES_PER_ROW = 16
MAX_DICTIONARY = 64


def _footer_in_bounds(footer: object) -> bool:
    """ts-viz writes no compression, one row group per day, and small column chunks.

    Anything else is not its file. The size of every chunk is bounded BEFORE it is decoded, so
    neither compression nor a dictionary can expand a file far beyond its size on the disk.
    """
    if footer.num_row_groups > MAX_DAYS:
        return False
    for g in range(footer.num_row_groups):
        group = footer.row_group(g)
        if group.num_rows > MAX_RESIDENTS:
            return False
        limit = CHUNK_SLACK_BYTES + CHUNK_BYTES_PER_ROW * group.num_rows
        for c in range(footer.num_columns):
            chunk = group.column(c)
            if chunk.compression != "UNCOMPRESSED" or chunk.total_uncompressed_size > limit:
                return False
    return True


def _place_codes(column: object) -> object:
    """Code in PLACES of every row of the dictionary-encoded `place` column (int64 array).

    The dictionary of each chunk is checked first (size, then values), and only then are the
    indices mapped: a row never becomes a string.
    """
    import numpy as np
    import pyarrow as pa
    import pyarrow.compute as pc

    value_set = pa.array(PLACES)
    parts = []
    for chunk in column.chunks:
        if not pa.types.is_dictionary(chunk.type) or chunk.null_count:
            raise VizError("input refused: unknown place")
        if len(chunk.dictionary) > MAX_DICTIONARY:
            raise VizError("input refused: unknown place")
        mapping = pc.index_in(chunk.dictionary, value_set=value_set)
        if mapping.null_count:
            raise VizError("input refused: unknown place")
        parts.append(pc.take(mapping, chunk.indices).to_numpy())
    return np.concatenate(parts).astype(np.int64)


def read_positions(path: Path) -> Positions:
    """Reads and validates the file. Raises `VizError` for anything not written by ts-viz."""
    import numpy as np
    import pyarrow as pa
    import pyarrow.parquet as pq

    if not path.is_file() or path.stat().st_size > MAX_FILE_BYTES:
        raise VizError("input refused: not a file, or larger than 512 MiB")
    try:
        parquet = pq.ParquetFile(path)
        schema = parquet.schema_arrow
        footer = parquet.metadata
        rows = footer.num_rows
        raw = footer.metadata or {}
    except (pa.ArrowException, OSError, ValueError):
        raise VizError("input refused: not a readable Parquet file") from None
    expected = [(name, pa.int64()) for name in INT_COLUMNS] + [("place", pa.string())]
    if [(f.name, f.type) for f in schema] != expected or any(f.nullable for f in schema):
        raise VizError("input refused: the columns are not those of ts-viz")
    if not 0 < rows <= MAX_ROWS:
        raise VizError(f"input refused: rows must be between 1 and {MAX_ROWS}")
    if not _footer_in_bounds(footer):
        raise VizError("input refused: row groups or column chunks out of bounds, or compressed")
    try:
        meta = {
            k.decode()[len(META_PREFIX):]: v.decode()
            for k, v in raw.items()
            if k.startswith(META_PREFIX.encode())
        }
    except UnicodeDecodeError:
        raise VizError("input refused: malformed metadata") from None
    if meta.get("format") != FORMAT:
        raise VizError("input refused: unknown format of positions.parquet")
    try:
        uuid.UUID(hex=meta["recording_id"])
        days = int(meta["days"])
        width, height = int(meta["grid_width"]), int(meta["grid_height"])
    except (KeyError, ValueError):
        raise VizError("input refused: missing or malformed metadata") from None
    if not (1 <= days <= MAX_DAYS and 1 <= width <= 1024 and 1 <= height <= 1024):
        raise VizError("input refused: metadata out of bounds")
    try:
        # `place` stays dictionary-encoded: its few values are checked before any row is
        # expanded into a string (a dictionary bomb repeats one long string for every row).
        data = pq.ParquetFile(path, read_dictionary=["place"])
        table = data.read()
        columns = {name: table.column(name).to_numpy() for name in INT_COLUMNS}
        place = _place_codes(table.column("place"))
    except VizError:
        raise
    except (pa.ArrowException, MemoryError, OSError, ValueError, OverflowError):
        raise VizError("input refused: the data cannot be read within bounds") from None
    day = columns["day"]
    if not (
        day.min() >= 0
        and day.max() < days
        and columns["tick"].min() >= 0
        and columns["id"].min() >= 0
        and np.unique(day, return_counts=True)[1].max() <= MAX_RESIDENTS
        and (columns["x"] >= 0).all()
        and (columns["x"] < width).all()
        and (columns["y"] >= 0).all()
        and (columns["y"] < height).all()
        and (np.diff(day) >= 0).all()
    ):
        raise VizError("input refused: values out of bounds")
    return Positions(meta, columns, place)


def convert(positions: Positions, out: Path) -> int:
    """Writes the recording `out`; returns the number of days logged.

    Time lines: `day` and `tick`, both sequences of the simulated world (VZ4). The recording
    id comes from the file (derived from seed and scenario), the application id is fixed, and
    no wall-clock property is sent (`send_properties=False`).
    """
    import numpy as np
    import rerun as rr

    meta = positions.meta
    cols = positions.columns
    rr.init(
        APP_ID,
        recording_id=uuid.UUID(hex=meta["recording_id"]),
        spawn=False,
        send_properties=False,
        strict=True,
    )
    rr.save(out)
    # No wall-clock timeline: `log_time` would hold the time of the conversion (VZ4).
    rr.set_log_time_enabled(False)
    width, height = int(meta["grid_width"]), int(meta["grid_height"])
    rr.log(
        "town",
        rr.AnnotationContext(
            [(i, name, PLACE_COLORS[i]) for i, name in enumerate(PLACES)],
        ),
        static=True,
    )
    rr.log("town/bounds", rr.Boxes2D(mins=[[0, 0]], sizes=[[width, height]]), static=True)
    days = np.unique(cols["day"])
    # Rows are by increasing day (checked): split once, at the first row of each day.
    starts = np.searchsorted(cols["day"], days)
    ends = np.append(starts[1:], len(cols["day"]))
    xy = np.column_stack([cols["x"], cols["y"]]).astype(np.float32) + 0.5
    for day, start, end in zip(days, starts, ends, strict=True):
        rr.set_time("day", sequence=int(day))
        rr.set_time("tick", sequence=int(cols["tick"][start]))
        rr.log(
            "town/residents",
            rr.Points2D(xy[start:end], radii=0.2, class_ids=positions.place[start:end]),
        )
    # Residents per place and day: one call per place, on the same two time lines.
    rr.reset_time()
    indexes = [
        rr.TimeColumn("day", sequence=days),
        rr.TimeColumn("tick", sequence=cols["tick"][starts]),
    ]
    for i, name in enumerate(PLACES):
        counts = np.add.reduceat((positions.place == i).astype(np.float64), starts)
        rr.send_columns(f"counts/{name}", indexes=indexes, columns=rr.Scalars.columns(scalars=counts))
    rr.disconnect()
    return int(len(days))


def _remove(path: Path) -> None:
    """Best effort: the recording stream may still hold the file (Windows)."""
    import rerun as rr

    try:
        rr.disconnect()
    except Exception:  # noqa: BLE001 - cleaning up after a failure
        pass
    try:
        path.unlink(missing_ok=True)
    except OSError:
        pass


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="positions.parquet to Rerun .rrd (optional tool)")
    parser.add_argument("parquet", help="positions.parquet written by ts-viz, under TS_CACHE")
    parser.add_argument("--out", required=True, help="the .rrd to write, under TS_CACHE, new")
    args = parser.parse_args(argv)
    # Before anything imports rerun (pyarrow does not read these variables).
    harden_environment()
    try:
        cache = cache_root()
        source = under_cache(args.parquet, cache, "input")
        out = under_cache(args.out, cache, "output")
        if out.suffix != ".rrd" or out.exists():
            raise VizError("output refused: must be a new .rrd file")
        if not out.parent.is_dir():
            raise VizError("output refused: the directory does not exist")
        ensure_analytics_disabled()
        positions = read_positions(source)
        # The recording is written to a file this script creates (`O_EXCL`), then renamed: a
        # file created by someone else in between is never truncated by `rr.save`.
        part = out.with_name(f"{out.name}.part")
        try:
            os.close(os.open(part, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600))
        except OSError:
            raise VizError("output refused: cannot create a new file") from None
        try:
            days = convert(positions, part)
            if out.exists():
                raise VizError("output refused: must be a new .rrd file")
            os.replace(part, out)
        except BaseException as error:
            # No partial recording stays.
            _remove(part)
            if isinstance(error, Exception) and not isinstance(error, VizError):
                # The name of the error only: a traceback would show local paths.
                raise VizError(f"conversion failed ({type(error).__name__})") from None
            raise
    except VizError as error:
        print(f"positions_to_rrd: {error}", file=sys.stderr)
        return 1
    print(f"positions_to_rrd: {days} days, recording {positions.meta['recording_id']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
