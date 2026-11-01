// spec: revues/architecture.md §18.4, §22 ; carnet architecture-securite.md §9.16
//! Python bindings of time_switcher, phase 0 (T10, V0-10): Python reads the state.
//!
//! The module `ts_py` loads the toy world of `ts_toy` with a **public test seed**, advances
//! it, and reads **copies** of its state: positions of the residents and state hash. It holds
//! no game rule and writes no state besides advancing the world. Conditions of archi-sec
//! (§9.16):
//!
//! - PY1: `#![deny(unsafe_code)]`, and no local `allow`: the `pyo3` macros need none.
//! - PY2: every read returns a numpy array that owns its data (integers, by increasing
//!   `StableId`); nothing points into the ECS memory, and no Python object keeps a reference
//!   to the `World`.
//! - PY3: the world key only comes from `ts_toy::public_test_key(seed)`.
//! - PY4: bounded inputs; an out-of-range count of ticks, date, seed or thread count raises
//!   `ValueError`, never a panic or an overflow; errors name no local path.
//! - PY6: the number of threads only goes to `SimConfig`, which has no effect on the result.
//! - PY7: a tool crate, outside `default-members`; it reads and writes no file.

#![deny(unsafe_code)]

use std::num::NonZeroU16;

use numpy::ToPyArray;
use numpy::ndarray::Array2;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyInt};
use ts_kernel::{Sim, SimConfig, SimTime, StableIndex, StopCondition, Threads};
use ts_toy::town::TownSpec;
use ts_toy::world::{Commuter, Place, Whereabouts, rules};
use ts_toy::{day_start, hex, public_test_key};

/// Latest date the world may reach, in simulated days: the bound of `ts-sim` (Y4). A bound
/// of the tool, not a game value.
const MAX_DAYS: i64 = 3660;
/// Largest thread count accepted. A bound of the tool.
const MAX_THREADS: u16 = 64;
/// Columns of [`ToyWorld::positions`], in order.
const POSITION_COLUMNS: [&str; 5] = ["id", "place", "parcel", "x", "y"];
/// Names of the `place` codes, by code.
const PLACES: [&str; 4] = ["home", "to_work", "work", "to_home"];

/// Checks that `value` is a Python `int` (not a `bool`) and converts it to an `i128`. A value
/// beyond `i128` is out of range for every caller.
fn int_arg(value: &Bound<'_, PyAny>, what: &str) -> PyResult<i128> {
    if value.is_instance_of::<PyBool>() || !value.is_instance_of::<PyInt>() {
        return Err(PyTypeError::new_err(format!("{what} must be an int")));
    }
    value
        .extract::<i128>()
        .map_err(|_| PyValueError::new_err(format!("{what} is out of range")))
}

/// `value` converted to `T`, or `ValueError` when it does not fit.
fn in_range<T: TryFrom<i128>>(value: i128, what: &str) -> PyResult<T> {
    T::try_from(value).map_err(|_| PyValueError::new_err(format!("{what} is out of range")))
}

/// Thread configuration for `n` threads, 1 to [`MAX_THREADS`].
fn threads_of(n: i128) -> PyResult<Threads> {
    let n: u16 = in_range(n, "threads")?;
    match NonZeroU16::new(n) {
        Some(n) if n.get() == 1 => Ok(Threads::Single),
        Some(n) if n.get() <= MAX_THREADS => Ok(Threads::Fixed(n)),
        _ => Err(PyValueError::new_err(format!(
            "threads must be between 1 and {MAX_THREADS}"
        ))),
    }
}

/// Code of a place, as listed in [`PLACES`].
const fn place_code(place: Place) -> i64 {
    place as i64
}

/// The toy world of `ts_toy` (`TownSpec::PHASE0`: 32 x 32 parcels, 1,000 residents), built
/// from a public test seed. A test world, never a game world (D-DEV-02).
#[pyclass(module = "ts_py")]
struct ToyWorld {
    sim: Sim,
}

impl ToyWorld {
    /// Runs the world to `target` with the GIL released. `target` is already checked.
    fn run_to(&mut self, py: Python<'_>, target: SimTime) -> (u64, u64) {
        let sim = &mut self.sim;
        let report = py.detach(|| sim.run_until(StopCondition::At(target)));
        (report.ticks, report.skipped_ticks)
    }

    /// `target` if it lies between the current date and day [`MAX_DAYS`], else `ValueError`.
    fn checked_target(&self, target: i128) -> PyResult<SimTime> {
        let latest = day_start(MAX_DAYS).as_millis();
        let now = self.sim.now().as_millis();
        if target < i128::from(now) {
            return Err(PyValueError::new_err(
                "the date is before the current date of the world",
            ));
        }
        if target > i128::from(latest) {
            return Err(PyValueError::new_err(format!(
                "the date is after day {MAX_DAYS}, the latest date of the world"
            )));
        }
        Ok(SimTime::from_millis(in_range(target, "date")?))
    }
}

#[pymethods]
impl ToyWorld {
    /// Builds the toy world at the epoch with the public test seed `seed` (an unsigned 64-bit
    /// integer) on `threads` compute threads (1 to 64). The thread count has no effect on the
    /// result. The thread pool belongs to the process: a second world with another count
    /// raises `ValueError`.
    #[new]
    #[pyo3(signature = (seed, threads = None))]
    fn new(seed: &Bound<'_, PyAny>, threads: Option<&Bound<'_, PyAny>>) -> PyResult<Self> {
        let seed: u64 = in_range(int_arg(seed, "seed")?, "seed")?;
        let threads = match threads {
            Some(t) => threads_of(int_arg(t, "threads")?)?,
            None => Threads::Single,
        };
        let config = SimConfig {
            threads,
            ..SimConfig::default()
        };
        let sim = Sim::build(config, &rules(TownSpec::PHASE0), &public_test_key(seed))
            .map_err(|e| PyValueError::new_err(format!("cannot build the toy world: {e}")))?;
        Ok(Self { sim })
    }

    /// Current date of the world, in milliseconds since the epoch.
    #[getter]
    fn now(&self) -> i64 {
        self.sim.now().as_millis()
    }

    /// Duration of a tick, in milliseconds.
    #[getter]
    fn tick_millis(&self) -> i64 {
        self.sim.tick_millis()
    }

    /// Advances the world by `ticks` ticks of simulated time, run or skipped. Returns
    /// `(ticks run, ticks skipped)`.
    fn advance_ticks(&mut self, py: Python<'_>, ticks: &Bound<'_, PyAny>) -> PyResult<(u64, u64)> {
        let ticks = int_arg(ticks, "ticks")?;
        if ticks < 0 {
            return Err(PyValueError::new_err("ticks must not be negative"));
        }
        let now = i128::from(self.sim.now().as_millis());
        let target = ticks
            .checked_mul(i128::from(self.sim.tick_millis()))
            .and_then(|ms| ms.checked_add(now))
            .ok_or_else(|| PyValueError::new_err("ticks is out of range"))?;
        let target = self.checked_target(target)?;
        Ok(self.run_to(py, target))
    }

    /// Advances the world to the first tick start at or after `millis`, a date in
    /// milliseconds since the epoch, between the current date and day 3660. Returns
    /// `(ticks run, ticks skipped)`.
    fn advance_to(&mut self, py: Python<'_>, millis: &Bound<'_, PyAny>) -> PyResult<(u64, u64)> {
        let target = self.checked_target(int_arg(millis, "date")?)?;
        Ok(self.run_to(py, target))
    }

    /// Global state hash, in lower-case hexadecimal (the format of `data/golden/`).
    fn state_hash(&self) -> String {
        hex(self.sim.state_hash().global.as_bytes())
    }

    /// Fingerprint of the active rules, in lower-case hexadecimal.
    fn rules_fingerprint(&self) -> String {
        hex(self.sim.rules_fingerprint().as_bytes())
    }

    /// Positions of the residents: a new `int64` array of shape `(residents, 5)`, one row per
    /// resident by increasing `StableId`, with the columns of `POSITION_COLUMNS`: identifier,
    /// place code (`PLACES`), parcel, and the parcel's column `x` and row `y` in the grid.
    /// On the way, the parcel is the one the resident left. The array owns its data: a copy,
    /// never a view of the world.
    fn positions<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let world = self.sim.world();
        let width = i64::from(TownSpec::PHASE0.width);
        let mut rows = Vec::new();
        for (id, e) in world.resource::<StableIndex>().iter() {
            let (Some(c), Some(w)) = (world.get::<Commuter>(e), world.get::<Whereabouts>(e)) else {
                continue;
            };
            let parcel = i64::from(match w.place {
                Place::Home | Place::ToWork => c.home,
                Place::Work | Place::ToHome => c.work,
            });
            let id: i64 = in_range(i128::from(id.index()), "id")?;
            rows.extend([
                id,
                place_code(w.place),
                parcel,
                parcel % width,
                parcel / width,
            ]);
        }
        let count = rows.len() / POSITION_COLUMNS.len();
        let table = Array2::from_shape_vec((count, POSITION_COLUMNS.len()), rows)
            .map_err(|_| PyValueError::new_err("inconsistent position table"))?;
        // `to_pyarray` allocates a numpy array and copies into it: the array owns its data.
        Ok(table.to_pyarray(py).into_any())
    }
}

/// The `ts_py` module: the [`ToyWorld`] class and its constants.
#[pymodule]
fn ts_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ToyWorld>()?;
    m.add("MILLIS_PER_DAY", SimTime::MILLIS_PER_DAY)?;
    m.add("MAX_DAYS", MAX_DAYS)?;
    m.add("POSITION_COLUMNS", POSITION_COLUMNS)?;
    m.add("PLACES", PLACES)?;
    Ok(())
}
