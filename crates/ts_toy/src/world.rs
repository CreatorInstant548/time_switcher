// spec: revues/architecture.md §8.1, §9.1, §10.2, §11.3, §12.1
//! Toy world plugin: each resident of the toy town goes from home to work and back every
//! day. Every move is a dated event of the kernel scheduler, so the world runs by time skip.
//! A few residents take a short stroll in full simulation ([`LodComplete`]) once a week, to
//! exercise the tick by tick mode.
//!
//! It follows the plugin contracts of `ts_kernel` (time skip and AS-27): all its state is
//! declared, no `Local`, no `Added` nor `Changed`, durable references are `StableId`s, and
//! the only per-tick work concerns entities in full simulation. Draws come only from the
//! kernel's keyed streams.

use bevy_ecs::prelude::{Commands, Component, Query, Res, ResMut, Resource, With};
use ts_kernel::rng::{DrawSite, Stream};
use ts_kernel::{
    CanonicalError, DueEvents, EventQueue, LodComplete, Phase, Plugin, SimBuilder, SimClock,
    SimTime, StableEntity, StableId, StableIds, StableIndex, StateDecode, StateHash, StateReader,
    StateWriter, TimedEvent, WorldStreams, par_for_each_mut,
};

use crate::day_start;
use crate::town::{Execution, TownSpec, generate};

// Agenda of the toy world. **Test parameters**, not game values: none comes from the design.
/// One minute, in milliseconds.
const MINUTE: i64 = 60_000;
/// First departure from home, in minutes after midnight (07:00).
const DEPART_FROM: i64 = 7 * 60;
/// Departures from home spread over this many minutes.
const DEPART_SPREAD: u64 = 120;
/// Travel time: a fixed part, plus this many minutes per parcel, plus a drawn delay.
const TRAVEL_BASE: i64 = 5;
const TRAVEL_PER_CELL: i64 = 2;
const TRAVEL_SPREAD: u64 = 10;
/// Time at work: a fixed part plus a drawn part, in minutes.
const WORK_BASE: i64 = 8 * 60;
const WORK_SPREAD: u64 = 60;
/// Residents who stroll in full simulation (the first ones), every `STROLL_EVERY` days at
/// `STROLL_AT` (minutes after midnight), for `STROLL_MS` milliseconds.
pub const STROLLERS: usize = 3;
pub const STROLL_EVERY: i64 = 7;
pub const STROLL_AT: i64 = 12 * 60;
pub const STROLL_MS: i64 = 30_000;

/// Event kinds of the toy world (`TimedEvent::kind`); `arg` is the day.
const DEPART_HOME: u32 = 1;
const ARRIVE_WORK: u32 = 2;
const DEPART_WORK: u32 = 3;
const ARRIVE_HOME: u32 = 4;
const STROLL_START: u32 = 5;
const STROLL_END: u32 = 6;

/// Draw sites: `Population` stream for the agenda, `Ai` for the stroll.
const DEPART: DrawSite = DrawSite::new(Stream::Population, 1, "toy.depart");
const TRAVEL_OUT: DrawSite = DrawSite::new(Stream::Population, 2, "toy.travel_out");
const TRAVEL_BACK: DrawSite = DrawSite::new(Stream::Population, 3, "toy.travel_back");
const WORK: DrawSite = DrawSite::new(Stream::Population, 4, "toy.work");
const STEP: DrawSite = DrawSite::new(Stream::Ai, 5, "toy.step");
/// Every site of the plugin, those of the generator included.
const SITES: [DrawSite; 8] = [
    DEPART,
    TRAVEL_OUT,
    TRAVEL_BACK,
    WORK,
    STEP,
    crate::town::KIND,
    crate::town::MEMBERS,
    crate::town::WORKPLACE,
];

/// Where a resident is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Place {
    /// At home.
    Home = 0,
    /// On the way to work.
    ToWork = 1,
    /// At work.
    Work = 2,
    /// On the way home.
    ToHome = 3,
}

/// State component: home and work parcels, and the distance between them in parcels.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Commuter {
    /// Home parcel.
    pub home: u32,
    /// Work parcel.
    pub work: u32,
    /// Manhattan distance between them, in parcels.
    pub cells: u32,
}

/// State component: where the resident is, since when, and the round trips completed.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Whereabouts {
    /// Current place.
    pub place: Place,
    /// Date of the last move.
    pub since: SimTime,
    /// Round trips completed.
    pub trips: u32,
}

/// State component of the strollers: steps walked in full simulation.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Stroll {
    /// Steps walked.
    pub steps: u64,
}

/// State resource: counters of the world.
#[derive(Resource, Debug, Clone, Default, PartialEq, Eq)]
pub struct Tally {
    /// Departures (from home or from work).
    pub departures: u64,
    /// Arrivals (at work or at home).
    pub arrivals: u64,
    /// Residents at work now.
    pub at_work: u32,
    /// Residents on the way now.
    pub travelling: u32,
}

impl StateHash for Commuter {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u32(self.home);
        w.u32(self.work);
        w.u32(self.cells);
    }
}

impl StateDecode for Commuter {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self {
            home: r.u32()?,
            work: r.u32()?,
            cells: r.u32()?,
        })
    }
}

impl StateHash for Whereabouts {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u8(self.place as u8);
        self.since.hash_state(w);
        w.u32(self.trips);
    }
}

impl StateDecode for Whereabouts {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        let place = match r.u8()? {
            0 => Place::Home,
            1 => Place::ToWork,
            2 => Place::Work,
            3 => Place::ToHome,
            _ => return Err(CanonicalError::Invalid("toy place")),
        };
        Ok(Self {
            place,
            since: SimTime::decode_state(r)?,
            trips: r.u32()?,
        })
    }
}

impl StateHash for Stroll {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u64(self.steps);
    }
}

impl StateDecode for Stroll {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        r.u64().map(|steps| Self { steps })
    }
}

impl StateHash for Tally {
    fn hash_state(&self, w: &mut StateWriter<'_>) {
        w.u64(self.departures);
        w.u64(self.arrivals);
        w.u32(self.at_work);
        w.u32(self.travelling);
    }
}

impl StateDecode for Tally {
    fn decode_state(r: &mut StateReader<'_>) -> Result<Self, CanonicalError> {
        Ok(Self {
            departures: r.u64()?,
            arrivals: r.u64()?,
            at_work: r.u32()?,
            travelling: r.u32()?,
        })
    }
}

/// `minutes` after `at`.
const fn after(at: SimTime, minutes: i64) -> SimTime {
    at.add_millis(minutes * MINUTE)
}

/// Drawn number in `0..spread`.
fn spread(keys: &WorldStreams, site: DrawSite, who: StableId, day: u64, spread: u64) -> i64 {
    // Exact: below `spread`, itself small.
    (keys.draw(site, who, day) % spread) as i64
}

/// Departure from home on `day`.
fn departure(keys: &WorldStreams, who: StableId, day: u64) -> SimTime {
    let minutes = DEPART_FROM + spread(keys, DEPART, who, day, DEPART_SPREAD);
    after(day_start(day as i64), minutes)
}

/// Travel time of leg `leg` ([`TRAVEL_OUT`] to work, [`TRAVEL_BACK`] home) on `day`, in
/// minutes.
fn travel(keys: &WorldStreams, who: StableId, c: &Commuter, day: u64, leg: DrawSite) -> i64 {
    let jitter = spread(keys, leg, who, day, TRAVEL_SPREAD);
    TRAVEL_BASE + TRAVEL_PER_CELL * i64::from(c.cells) + jitter
}

/// Start of the stroll of `day`.
const fn stroll_start(day: u64) -> SimTime {
    after(day_start(day as i64), STROLL_AT)
}

// spec: revues/architecture.md §8.1, §9.1
/// Phase `Scheduled`: applies the due events in `(at, seq)` order and schedules the next
/// move of each resident.
fn on_events(
    due: Res<DueEvents>,
    index: Res<StableIndex>,
    streams: Res<WorldStreams>,
    mut queue: ResMut<EventQueue>,
    mut tally: ResMut<Tally>,
    mut people: Query<(&Commuter, &mut Whereabouts)>,
    mut commands: Commands,
) {
    let keys = &*streams;
    for item in due.iter() {
        let TimedEvent { kind, subject, arg } = item.event;
        let Some(e) = index.get(subject) else {
            continue;
        };
        let Ok((commuter, mut at)) = people.get_mut(e) else {
            continue;
        };
        let mut next = |when: SimTime, kind: u32, day: u64| {
            queue.schedule(
                when,
                TimedEvent {
                    kind,
                    subject,
                    arg: day,
                },
            );
        };
        let moved = |place: Place, at: &mut Whereabouts| {
            at.place = place;
            at.since = item.at;
        };
        match kind {
            DEPART_HOME => {
                moved(Place::ToWork, &mut at);
                tally.departures += 1;
                tally.travelling += 1;
                let minutes = travel(keys, subject, commuter, arg, TRAVEL_OUT);
                next(after(item.at, minutes), ARRIVE_WORK, arg);
            }
            ARRIVE_WORK => {
                moved(Place::Work, &mut at);
                tally.arrivals += 1;
                // Plain subtraction: an arrival without its departure panics (overflow-checks).
                tally.travelling -= 1;
                tally.at_work += 1;
                let minutes = WORK_BASE + spread(keys, WORK, subject, arg, WORK_SPREAD);
                next(after(item.at, minutes), DEPART_WORK, arg);
            }
            DEPART_WORK => {
                moved(Place::ToHome, &mut at);
                tally.departures += 1;
                tally.at_work -= 1;
                tally.travelling += 1;
                let minutes = travel(keys, subject, commuter, arg, TRAVEL_BACK);
                next(after(item.at, minutes), ARRIVE_HOME, arg);
            }
            ARRIVE_HOME => {
                moved(Place::Home, &mut at);
                at.trips += 1;
                tally.arrivals += 1;
                tally.travelling -= 1;
                next(departure(keys, subject, arg + 1), DEPART_HOME, arg + 1);
            }
            STROLL_START => {
                commands.entity(e).insert(LodComplete);
                next(item.at.add_millis(STROLL_MS), STROLL_END, arg);
            }
            STROLL_END => {
                commands.entity(e).remove::<LodComplete>();
                let day = arg + STROLL_EVERY as u64;
                next(stroll_start(day), STROLL_START, day);
            }
            _ => {}
        }
    }
}

// spec: revues/architecture.md §8.1, §9.2
/// Phase `Act`, every tick: the residents in full simulation walk. Each writes only its own
/// component, with a keyed draw per tick; a plain loop while they are few (issue #67).
fn stroll(
    clock: Res<SimClock>,
    streams: Res<WorldStreams>,
    mut q: Query<(&StableEntity, &mut Stroll), With<LodComplete>>,
) {
    let slot = clock.now().as_millis().unsigned_abs();
    let keys = &*streams;
    par_for_each_mut(&mut q, |(s, mut st)| {
        st.steps += 1 + keys.draw(STEP, s.id(), slot) % 3;
    });
}

/// The toy world plugin, on the town of `town`. **Test plugin**, never a rule (D-DEV-02).
#[derive(Debug, Clone, Copy)]
pub struct ToyWorld {
    /// Town to generate.
    pub town: TownSpec,
}

impl Plugin for ToyWorld {
    fn name(&self) -> &'static str {
        "toy_world"
    }

    fn version(&self) -> u32 {
        1
    }

    fn draw_sites(&self) -> &'static [DrawSite] {
        &SITES
    }

    fn build(&self, b: &mut SimBuilder) {
        b.state_component::<Commuter>("toy")
            .state_component::<Whereabouts>("toy")
            .state_component::<Stroll>("toy")
            .state_resource::<Tally>("toy")
            .insert_resource(Tally::default());
        let world = b.world_mut();
        let town = generate(
            &self.town,
            world.resource::<WorldStreams>(),
            Execution::Pool,
        );
        let mut people = Vec::new();
        for r in &town.residents {
            let Some(id) = world.resource_mut::<StableIds>().allocate() else {
                break;
            };
            let commuter = Commuter {
                home: r.home,
                work: r.work,
                cells: town.cells(r.home, r.work),
            };
            let whereabouts = Whereabouts {
                place: Place::Home,
                since: SimTime::EPOCH,
                trips: 0,
            };
            let mut e = world.spawn((StableEntity::new(id), commuter, whereabouts));
            if people.len() < STROLLERS {
                e.insert(Stroll { steps: 0 });
            }
            people.push(id);
        }
        let keys = world.resource::<WorldStreams>();
        let mut first: Vec<(SimTime, TimedEvent)> = people
            .iter()
            .map(|&subject| {
                let kind = DEPART_HOME;
                let event = TimedEvent {
                    kind,
                    subject,
                    arg: 0,
                };
                (departure(keys, subject, 0), event)
            })
            .collect();
        for &subject in people.iter().take(STROLLERS) {
            let kind = STROLL_START;
            first.push((
                stroll_start(0),
                TimedEvent {
                    kind,
                    subject,
                    arg: 0,
                },
            ));
        }
        let mut queue = world.resource_mut::<EventQueue>();
        for (at, event) in first {
            queue.schedule(at, event);
        }
        b.add_systems(Phase::Scheduled, on_events);
        b.add_systems(Phase::Act, stroll);
    }
}

/// The rules of the toy world: this plugin alone.
pub fn rules(town: TownSpec) -> Vec<Box<dyn Plugin>> {
    vec![Box::new(ToyWorld { town })]
}
