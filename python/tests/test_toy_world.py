# spec: carnet architecture-securite.md §9.16 PY2 à PY4, PY6
"""Python reads the state of the toy world (T10, V0-10).

Run by `cargo xtask py`, after `maturin develop`. The golden values come from
data/golden/ts_toy.json, checked on the Rust side by `cargo xtask determinism`.
"""

import json
import subprocess
import sys
from pathlib import Path

import numpy as np
import pytest

import ts_py

GOLDEN = Path(__file__).resolve().parents[2] / "data" / "golden" / "ts_toy.json"
SEED = 1
DAYS = 14
RESIDENTS = 1000
WIDTH = 32


def golden(key: str) -> str:
    return json.loads(GOLDEN.read_text(encoding="utf-8"))["results"][key]


def run_short(threads: int = 1) -> ts_py.ToyWorld:
    world = ts_py.ToyWorld(SEED, threads)
    world.advance_to(DAYS * ts_py.MILLIS_PER_DAY)
    return world


def test_load_advance_14_days_and_read_positions():
    world = ts_py.ToyWorld(SEED)
    assert world.now == 0
    start = world.positions()
    ran, skipped = world.advance_to(DAYS * ts_py.MILLIS_PER_DAY)
    assert world.now == DAYS * ts_py.MILLIS_PER_DAY
    assert ran == int(golden("short.ticks"))
    assert skipped == int(golden("short.skipped"))

    table = world.positions()
    assert ts_py.POSITION_COLUMNS == ["id", "place", "parcel", "x", "y"]
    assert table.dtype == np.int64
    assert table.shape == (RESIDENTS, len(ts_py.POSITION_COLUMNS))
    ids, place, parcel, x, y = table.T
    # One row per resident, by increasing StableId.
    assert np.all(np.diff(ids) > 0)
    assert np.array_equal(ids, start[:, 0])
    # At midnight, every resident is home, as on day 0.
    assert np.all(place == ts_py.PLACES.index("home"))
    assert np.array_equal(parcel, start[:, 2])
    assert np.array_equal(x, parcel % WIDTH)
    assert np.array_equal(y, parcel // WIDTH)
    assert np.all((0 <= parcel) & (parcel < WIDTH * WIDTH))


def test_golden_state_hash_and_rules():
    world = run_short()
    assert world.state_hash() == golden("short.world")
    assert world.rules_fingerprint() == golden("short.rules")


def test_two_runs_are_identical():
    first, second = run_short(), run_short()
    assert first.state_hash() == second.state_hash()
    assert np.array_equal(first.positions(), second.positions())


def test_cutting_the_run_does_not_change_it():
    world = ts_py.ToyWorld(SEED)
    for day in range(1, DAYS + 1):
        # A date inside a tick stops at the next tick start: here, the last tick of the day.
        world.advance_to(day * ts_py.MILLIS_PER_DAY - world.tick_millis - 1)
        assert world.now == day * ts_py.MILLIS_PER_DAY - world.tick_millis
        world.advance_ticks(1)
        assert world.now == day * ts_py.MILLIS_PER_DAY
    assert world.state_hash() == golden("short.world")


def test_positions_are_owned_copies():
    world = ts_py.ToyWorld(SEED)
    world.advance_to(8 * 3_600_000)
    table = world.positions()
    # Owned by numpy, not a view: no base object, nothing points into the world.
    assert table.flags.owndata and table.base is None
    assert table.flags.c_contiguous
    before = table.copy()
    table[:] = -1
    assert np.array_equal(world.positions(), before)
    # A copy taken before an advance does not follow the world.
    world.advance_to(9 * 3_600_000)
    assert np.array_equal(table, np.full_like(before, -1))
    assert not np.array_equal(world.positions(), before)


@pytest.mark.parametrize("threads", [1, 4])
def test_golden_hash_on_1_and_4_threads(threads):
    # The thread pool belongs to the process: each count runs in its own interpreter.
    code = (
        "import ts_py, sys\n"
        f"w = ts_py.ToyWorld({SEED}, {threads})\n"
        f"w.advance_to({DAYS} * ts_py.MILLIS_PER_DAY)\n"
        "sys.stdout.write(w.state_hash())\n"
    )
    out = subprocess.run(
        [sys.executable, "-I", "-c", code],
        capture_output=True,
        text=True,
        check=True,
        timeout=600,
    )
    assert out.stdout == golden("short.world")


def test_bounded_inputs_raise_value_error():
    world = ts_py.ToyWorld(SEED)
    day = ts_py.MILLIS_PER_DAY
    for bad in [-1, 2**64, 10**40]:
        with pytest.raises(ValueError):
            ts_py.ToyWorld(bad)
    for bad in [0, -1, 65, 2**70]:
        with pytest.raises(ValueError):
            ts_py.ToyWorld(SEED, bad)
    for bad in [-1, 2**62, 2**63, 2**64, 10**40, -(10**40)]:
        with pytest.raises(ValueError):
            world.advance_ticks(bad)
    for bad in [-1, (ts_py.MAX_DAYS * day) + 1, 2**63, -(2**63) - 1, 10**40]:
        with pytest.raises(ValueError):
            world.advance_to(bad)
    world.advance_to(day)
    with pytest.raises(ValueError):
        world.advance_to(day - 1)
    # A refused call leaves the world unchanged.
    assert world.now == day
    for bad in [1.5, "3", None, True, np.float64(2.0), np.True_]:
        with pytest.raises(TypeError):
            world.advance_ticks(bad)
        with pytest.raises(TypeError):
            ts_py.ToyWorld(bad)
    # numpy integers are integers (`__index__`), under the same bounds.
    world.advance_ticks(np.int64(1))
    world.advance_to(np.uint64(2 * day))
    assert world.now == 2 * day
    with pytest.raises(ValueError):
        world.advance_ticks(np.int64(-1))
    assert ts_py.ToyWorld(np.uint64(SEED)).state_hash() == ts_py.ToyWorld(SEED).state_hash()


def test_errors_name_no_local_path():
    world = ts_py.ToyWorld(SEED)
    messages = []
    for call in [
        lambda: world.advance_to(-1),
        lambda: world.advance_ticks(2**64),
        lambda: ts_py.ToyWorld(-1),
        lambda: ts_py.ToyWorld(SEED, 0),
    ]:
        with pytest.raises(ValueError) as error:
            call()
        messages.append(str(error.value))
    for message in messages:
        assert "\\" not in message and "/" not in message, message
        assert str(Path.home()) not in message


def test_another_thread_count_in_the_same_process_is_refused():
    ts_py.ToyWorld(SEED, 1)
    with pytest.raises(ValueError, match="thread"):
        ts_py.ToyWorld(SEED, 4)
