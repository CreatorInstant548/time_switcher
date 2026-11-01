# spec: carnet architecture-securite.md §9.16 VZ4, VZ5, VZ7 (AS-48)
"""Converts `positions.parquet` (crate ts_viz) into a Rerun recording (`.rrd`).

OPTIONAL TOOL, group `viz` of python/pyproject.toml: installed neither by `cargo xtask py`
nor by the CI. How to use it: docs/dev/visualisation.md.

    uv run --no-sync python viz/positions_to_rrd.py <positions.parquet> --out <positions.rrd>

What it does, and nothing else (VZ7):
- reads the Parquet file with pyarrow (a dependency of rerun-sdk, nothing more is installed);
- writes the recording with `rr.save(<file under TS_CACHE>)`: never `spawn`, `connect_grpc`,
  `serve` or `show`, and it never starts the Rerun viewer. Open the .rrd in the viewer you
  installed apart;
- cuts the telemetry of Rerun first, by the documented way (`rerun analytics disable`), and
  removes from its environment every variable that could redirect Rerun (`RERUN*`,
  `FORCE_RERUN_ANALYTICS`, `OTEL_*`, `TELEMETRY_*`) before `rerun` is imported.

Input and output are strictly under TS_CACHE (`C:/Dev/ts-cache` on Windows, else
`~/.cache/time_switcher`), as for `ts-sim` and `ts-viz` (Y3); the output must not exist yet.
The input is untrusted data: size, schema, row counts and values are checked before use
(VZ5: at most 3660 days, and the residents of the toy town).
"""

from __future__ import annotations

import argparse
import os
import subprocess
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


def disable_analytics() -> None:
    """`rerun analytics disable`, the documented opt-out (rerun-io/rerun, crates/utils/re_analytics).

    It only writes the opt-out into the analytics configuration of the user; it opens no viewer
    and sends nothing. Fails closed: the script stops if it cannot cut the telemetry.
    """
    done = subprocess.run(  # noqa: S603 - fixed arguments, scrubbed environment
        [sys.executable, "-I", "-m", "rerun_cli", "analytics", "disable"],
        stdin=subprocess.DEVNULL,
        capture_output=True,
        text=True,
        timeout=120,
        check=False,
    )
    if done.returncode != 0:
        raise VizError("cannot disable the Rerun analytics (rerun analytics disable failed)")


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
        rows = parquet.metadata.num_rows
        raw = parquet.metadata.metadata or {}
    except (pa.ArrowException, OSError, ValueError):
        raise VizError("input refused: not a readable Parquet file") from None
    expected = [(name, pa.int64()) for name in INT_COLUMNS] + [("place", pa.string())]
    if [(f.name, f.type) for f in schema] != expected or any(f.nullable for f in schema):
        raise VizError("input refused: the columns are not those of ts-viz")
    if not 0 < rows <= MAX_ROWS:
        raise VizError(f"input refused: rows must be between 1 and {MAX_ROWS}")
    meta = {
        k.decode()[len(META_PREFIX):]: v.decode()
        for k, v in raw.items()
        if k.startswith(META_PREFIX.encode())
    }
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
    table = parquet.read()
    columns = {name: table.column(name).to_numpy() for name in INT_COLUMNS}
    place_names = table.column("place").to_pylist()
    try:
        place = np.array([PLACES.index(p) for p in place_names], dtype=np.int64)
    except ValueError:
        raise VizError("input refused: unknown place") from None
    day = columns["day"]
    if not (
        day.min() >= 0
        and day.max() < days
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


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="positions.parquet to Rerun .rrd (optional tool)")
    parser.add_argument("parquet", help="positions.parquet written by ts-viz, under TS_CACHE")
    parser.add_argument("--out", required=True, help="the .rrd to write, under TS_CACHE, new")
    args = parser.parse_args(argv)
    # Before anything imports rerun (pyarrow does not read these variables).
    scrub_environment()
    try:
        cache = cache_root()
        source = under_cache(args.parquet, cache, "input")
        out = under_cache(args.out, cache, "output")
        if out.suffix != ".rrd" or out.exists():
            raise VizError("output refused: must be a new .rrd file")
        if not out.parent.is_dir():
            raise VizError("output refused: the directory does not exist")
        positions = read_positions(source)
        disable_analytics()
        days = convert(positions, out)
    except VizError as error:
        print(f"positions_to_rrd: {error}", file=sys.stderr)
        return 1
    print(f"positions_to_rrd: {days} days, recording {positions.meta['recording_id']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
