//! Parallel iteration that falls back to a plain loop on small sets (issue #67).

use bevy_ecs::query::{IterQueryData, QueryFilter, QueryItem};
use bevy_ecs::system::Query;

// spec: revues/architecture.md §9.2
/// Matched entities from which [`par_for_each_mut`] uses the thread pool. A speed setting,
/// never a game value.
// PROVISIONAL(design #88): to calibrate on the phase 1 systems; the result does not depend on it.
pub const PAR_ITER_FROM: usize = 64;

// spec: revues/architecture.md §9.2 ; carnet architecture-securite.md §9.10 L6
/// `query.par_iter_mut().for_each(func)`, but a plain loop on the calling thread when the
/// query matches fewer than [`PAR_ITER_FROM`] entities.
///
/// Why: each `par_iter` opens a scope on the pool and wakes a pool thread, a few
/// microseconds per call even for three entities, which the time skip pays at every tick.
///
/// **No effect on the result** (R1): `par_iter` is only allowed when each entity writes its
/// own components and draws from the keyed randomness (§9.2, point 2), so the order of the
/// entities does not matter; a one-thread pool already runs `par_iter` as this same plain
/// loop. The count is the sum of the sizes of the matched tables, a function of the world
/// state alone, never of time.
pub fn par_for_each_mut<'s, D, F, Func>(query: &mut Query<'_, 's, D, F>, func: Func)
where
    D: IterQueryData,
    F: QueryFilter,
    Func: for<'a> Fn(QueryItem<'a, 's, D>) + Send + Sync + Clone,
{
    // Upper bound: exact for archetype filters, the matched table sizes otherwise.
    let matched = query.iter_mut().size_hint().1.unwrap_or(usize::MAX);
    if matched < PAR_ITER_FROM {
        query.iter_mut().for_each(func);
    } else {
        query.par_iter_mut().for_each(func);
    }
}
