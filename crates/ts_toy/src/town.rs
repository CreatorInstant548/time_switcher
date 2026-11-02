// spec: revues/architecture.md §9.2, §6.4 ; revues/generation.md P26
//! Toy generator: a grid of parcels, then households on the home parcels, all drawn from
//! keyed streams. It runs in parallel chunks of rows, merged **by chunk index**, never in
//! the order the threads finish (architecture §9.2 rule 3): 1 and N threads give the same
//! town.

use bevy_tasks::ComputeTaskPool;
use ts_kernel::rng::{Stream, StreamKeys, draw};
use ts_kernel::{StableId, StateHash, StateWriter};

/// Draw purposes of the generator.
const KIND: u32 = 1;
const MEMBERS: u32 = 2;
const WORKPLACE: u32 = 3;

/// Size of the toy town. **Test parameters**, not game values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TownSpec {
    /// Parcels per row.
    pub width: u32,
    /// Rows of parcels.
    pub height: u32,
    /// Residents kept, in parcel order: the entities of the toy world.
    pub people: u32,
    /// Rows per parallel chunk.
    pub rows_per_chunk: u32,
}

impl TownSpec {
    /// The phase 0 town: 32 x 32 parcels, 1,000 residents (test parameters).
    pub const PHASE0: Self = Self {
        width: 32,
        height: 32,
        people: 1000,
        rows_per_chunk: 4,
    };
}

/// Use of a parcel. The discriminants are part of the state hash (see the `StateHash` impl of
/// [`Town`]): explicit and frozen, they change only with the golden values (issue #68, m3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParcelKind {
    /// Households live there.
    Home = 0,
    /// Residents work there.
    Work = 1,
}

/// A resident: home parcel and work parcel, by index in the grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resident {
    /// Home parcel.
    pub home: u32,
    /// Work parcel.
    pub work: u32,
}

/// The generated town.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Town {
    /// Parcels per row.
    pub width: u32,
    /// Use of each parcel, row by row.
    pub parcels: Vec<ParcelKind>,
    /// Residents, in home parcel order, at most [`TownSpec::people`].
    pub residents: Vec<Resident>,
}

impl Town {
    /// Manhattan distance between two parcels, in parcels.
    pub const fn cells(&self, a: u32, b: u32) -> u32 {
        let w = self.width;
        (a % w).abs_diff(b % w) + (a / w).abs_diff(b / w)
    }
}

impl StateHash for Town {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u32(self.width);
        w.count(self.parcels.len());
        for p in &self.parcels {
            w.u8(*p as u8);
        }
        w.count(self.residents.len());
        for r in &self.residents {
            w.u32(r.home);
            w.u32(r.work);
        }
    }
}

/// How the chunks run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Execution {
    /// One after the other, on the calling thread.
    Inline,
    /// On the kernel's compute pool (`ComputeTaskPool`), inline if it is not set up yet.
    Pool,
}

/// Map identity of a parcel (map namespace, P28).
fn parcel_id(index: u32) -> StableId {
    let Some(id) = StableId::map(u64::from(index)) else {
        unreachable!("a u32 index is below 2^63");
    };
    id
}

/// Runs `f` on each chunk `[first, end)` of parcel indices and concatenates the results in
/// chunk order, whatever the order in which the threads finish.
fn in_chunks<T, F>(spec: &TownSpec, exec: Execution, f: F) -> Vec<T>
where
    T: Send + 'static,
    F: Fn(u32, u32) -> Vec<T> + Sync,
{
    let parcels = spec.width * spec.height;
    let step = spec.width * spec.rows_per_chunk.max(1);
    let ranges: Vec<(u32, u32)> = (0..parcels)
        .step_by(step.max(1) as usize)
        .map(|a| (a, (a + step).min(parcels)))
        .collect();
    let f = &f;
    let mut parts: Vec<(usize, Vec<T>)> = match (exec, ComputeTaskPool::try_get()) {
        (Execution::Pool, Some(pool)) => pool.scope(|s| {
            for (i, &(a, b)) in ranges.iter().enumerate() {
                s.spawn(async move { (i, f(a, b)) });
            }
        }),
        _ => ranges
            .iter()
            .enumerate()
            .map(|(i, &(a, b))| (i, f(a, b)))
            .collect(),
    };
    parts.sort_unstable_by_key(|p| p.0);
    parts.into_iter().flat_map(|p| p.1).collect()
}

// spec: revues/architecture.md §9.2, §6.4
/// Generates the toy town of `spec`. A pure function of `keys`: a third of the parcels on
/// average are workplaces (`CityGen`), each home parcel holds 1 to 4 residents who each
/// work on a drawn workplace (`PopGen`). Same town for [`Execution::Inline`] and
/// [`Execution::Pool`], with any number of threads.
pub fn generate(spec: &TownSpec, keys: &StreamKeys, exec: Execution) -> Town {
    let city = keys.get(Stream::CityGen);
    let pop = keys.get(Stream::PopGen);
    let parcels = in_chunks(spec, exec, |a, b| {
        (a..b)
            .map(|i| match draw(city, parcel_id(i), 0, KIND) % 3 {
                0 => ParcelKind::Work,
                _ => ParcelKind::Home,
            })
            .collect()
    });
    let works: Vec<u32> = (0..)
        .zip(&parcels)
        .filter(|&(_, k)| *k == ParcelKind::Work)
        .map(|(i, _)| i)
        .collect();
    let mut residents = if works.is_empty() {
        Vec::new()
    } else {
        let (parcels, works) = (&parcels, &works);
        in_chunks(spec, exec, move |a, b| {
            let mut out = Vec::new();
            for home in (a..b).filter(|&i| parcels[i as usize] == ParcelKind::Home) {
                let id = parcel_id(home);
                let members = 1 + draw(pop, id, 0, MEMBERS) % 4;
                for m in 1..=members {
                    let pick = draw(pop, id, m, WORKPLACE) % works.len() as u64;
                    out.push(Resident {
                        home,
                        work: works[pick as usize],
                    });
                }
            }
            out
        })
    };
    residents.truncate(spec.people as usize);
    Town {
        width: spec.width,
        parcels,
        residents,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The discriminants of [`ParcelKind`] are hashed: they are frozen (issue #68, m3).
    #[test]
    fn parcel_kind_discriminants_are_frozen() {
        assert_eq!(ParcelKind::Home as u8, 0);
        assert_eq!(ParcelKind::Work as u8, 1);
    }
}
