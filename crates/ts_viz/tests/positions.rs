//! `positions.parquet` end to end: decoded data of the toy world, reproducible across two runs
//! (VZ4), no local path, user name or machine name in the file (VZ4), bounded days (VZ5), and
//! the rules of `--out` shared with `ts-sim` (Y3).
#![allow(clippy::disallowed_methods, clippy::unwrap_used)] // test: throwaway directories.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use ts_data::Limits;
use ts_runner::scenario::{Duration, MAX_DAYS, Scenario, World};
use ts_viz::positions::{self, Encoded};

const SCENARIOS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/scenarios");
const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/golden/ts_toy.json");
const MILLIS_PER_DAY: i64 = 86_400_000;
const RESIDENTS: usize = 1000;
const WIDTH: i64 = 32;

/// A fresh directory outside any Git working copy (the target directory may be in one).
fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ts_viz_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Value of `key` in the golden file of `ts_toy`.
fn golden(key: &str) -> String {
    let text = fs::read_to_string(GOLDEN).unwrap();
    let start = text.find(&format!("\"{key}\": \"")).unwrap() + key.len() + 5;
    text[start..start + text[start..].find('"').unwrap()].to_owned()
}

/// The scenario `name` of `data/scenarios`, with the bytes of its file.
fn scenario(name: &str) -> (Scenario, Vec<u8>) {
    let path = Path::new(SCENARIOS).join(name);
    let (scenario, file) = Scenario::load(&path, &Limits::default()).unwrap();
    (scenario, file.bytes)
}

fn encode(seed: u64) -> Encoded {
    let (scenario, bytes) = scenario("toy_short.ron");
    positions::encode(&scenario, &bytes, seed).unwrap()
}

/// One decoded row: day, tick, id, parcel, x, y, place.
type Row = (i64, i64, i64, i64, i64, i64, String);

/// A decoded file: its rows and its `ts_viz.*` metadata, without the prefix.
#[derive(Debug, PartialEq, Eq)]
struct Decoded {
    rows: Vec<Row>,
    meta: BTreeMap<String, String>,
}

fn decode(path: &Path) -> Decoded {
    let reader = SerializedFileReader::new(fs::File::open(path).unwrap()).unwrap();
    let meta = reader
        .metadata()
        .file_metadata()
        .key_value_metadata()
        .unwrap()
        .iter()
        .map(|kv| {
            let key = kv.key.strip_prefix(positions::META_PREFIX).unwrap();
            (key.to_owned(), kv.value.clone().unwrap())
        })
        .collect();
    let rows = reader
        .get_row_iter(None)
        .unwrap()
        .map(|row| {
            let row = row.unwrap();
            let long = |i| row.get_long(i).unwrap();
            let place = row.get_string(6).unwrap().clone();
            (long(0), long(1), long(2), long(3), long(4), long(5), place)
        })
        .collect();
    Decoded { rows, meta }
}

fn decode_bytes(dir: &Path, name: &str, bytes: &[u8]) -> Decoded {
    let path = dir.join(name);
    fs::write(&path, bytes).unwrap();
    decode(&path)
}

#[test]
fn short_run_decodes_to_the_table_of_the_toy_world() {
    let dir = temp("table");
    let encoded = encode(1);
    let d = decode_bytes(&dir, "a.parquet", &encoded.bytes);
    assert_eq!(encoded.rows, 14 * RESIDENTS);
    assert_eq!(d.rows.len(), encoded.rows);
    // Same world as the determinism proof of ts_toy (seed 1, 14 days).
    assert_eq!(d.meta["state_hash"], golden("short.world"));
    assert_eq!(d.meta["rules_fingerprint"], golden("short.rules"));
    assert_eq!(
        (&d.meta["state_hash"], &d.meta["rules_fingerprint"]),
        (&encoded.state_hash, &encoded.rules)
    );
    assert_eq!(d.meta["seed_kind"], "public_test");
    assert_eq!(d.meta["format"], positions::FORMAT.to_string());
    let tick_millis: i64 = d.meta["tick_millis"].parse().unwrap();
    let sample_at: i64 = d.meta["sample_at_ms"].parse().unwrap();
    for (n, rows) in d.rows.chunks(RESIDENTS).enumerate() {
        let day = i64::try_from(n).unwrap();
        // The sample of a day is taken at its sample time, on a tick start.
        assert_eq!((day * MILLIS_PER_DAY + sample_at) / tick_millis, rows[0].1);
        assert!(rows.windows(2).all(|w| w[0].2 < w[1].2), "ids increase");
        for (d, tick, _, parcel, x, y, place) in rows {
            assert_eq!((*d, *tick), (day, rows[0].1));
            assert!((0..WIDTH * WIDTH).contains(parcel));
            assert_eq!((*x, *y), (parcel % WIDTH, parcel / WIDTH));
            assert!(["home", "to_work", "work", "to_home"].contains(&place.as_str()));
        }
    }
    // At 08:00 of day 0 the residents are spread over several places: the file is not trivial.
    let day0: std::collections::BTreeSet<&str> =
        d.rows[..RESIDENTS].iter().map(|r| r.6.as_str()).collect();
    assert!(day0.len() >= 2, "{day0:?}");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn two_runs_decode_to_the_same_data() {
    let dir = temp("twice");
    let (first, second) = (encode(1), encode(1));
    assert_eq!(
        decode_bytes(&dir, "a.parquet", &first.bytes),
        decode_bytes(&dir, "b.parquet", &second.bytes)
    );
    // Our own writer is deterministic too: same bytes.
    assert_eq!(first, second);
    // Another seed is another town.
    let other = encode(2);
    assert_ne!(
        decode_bytes(&dir, "c.parquet", &other.bytes).rows,
        decode_bytes(&dir, "a.parquet", &first.bytes).rows
    );
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn recording_id_is_derived_from_the_seed_and_the_scenario() {
    let (short, short_bytes) = scenario("toy_short.ron");
    let (_, year_bytes) = scenario("toy_year.ron");
    let id = positions::recording_id(1, &short_bytes);
    assert_eq!(id, positions::recording_id(1, &short_bytes));
    assert_eq!(id.len(), 32);
    assert!(
        id.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    );
    assert_ne!(id, positions::recording_id(2, &short_bytes));
    assert_ne!(id, positions::recording_id(1, &year_bytes));
    // The file says it.
    assert_eq!(
        positions::encode(&short, &short_bytes, 1)
            .unwrap()
            .recording_id,
        id
    );
}

#[test]
fn days_are_bounded() {
    let make = |days| Scenario {
        name: "too_long".to_owned(),
        world: World::Toy,
        duration: Duration::Days(days),
    };
    for days in [0, MAX_DAYS + 1, u32::MAX] {
        let err = positions::encode(&make(days), b"", 1).unwrap_err();
        assert!(err.contains("between 1 and 3660"), "{err}");
    }
}

fn ts_viz(cache: &Path, args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ts-viz"))
        .env("TS_CACHE", cache)
        .arg("positions")
        .args(args)
        .output()
        .unwrap()
}

/// The strings that would identify this machine or this user, long enough to mean something.
fn local_strings(cache: &Path) -> Vec<String> {
    let mut found = vec![cache.to_string_lossy().into_owned()];
    for name in [
        "HOME",
        "USERPROFILE",
        "USERNAME",
        "USER",
        "LOGNAME",
        "COMPUTERNAME",
        "HOSTNAME",
    ] {
        found.extend(std::env::var(name).ok());
    }
    let host = Command::new("hostname").output().ok();
    found.extend(host.map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned()));
    found.retain(|s| s.len() >= 3);
    found
}

#[test]
fn the_file_holds_no_local_path_user_or_host() {
    let cache = temp("cache");
    let out = cache.join("run");
    let scenario = Path::new(SCENARIOS).join("toy_short.ron");
    let o = ts_viz(
        &cache,
        &[
            &scenario,
            "--seed".as_ref(),
            "1".as_ref(),
            "--out".as_ref(),
            &out,
        ],
    );
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    let bytes = fs::read(out.join(positions::FILE_NAME)).unwrap();
    let text = String::from_utf8_lossy(&bytes);
    let mut forbidden = local_strings(&cache);
    forbidden.extend(["TS_CACHE".to_owned(), "HOME".to_owned()]);
    assert!(forbidden.len() >= 3, "{forbidden:?}");
    for needle in forbidden {
        assert!(!text.contains(&needle), "the file contains a local string");
    }
    for local in [":\\", "/tmp", "/home", "/Users"] {
        assert!(!text.contains(local), "{local}");
    }
    // The tool says what it wrote, without the path.
    let stdout = String::from_utf8_lossy(&o.stdout);
    assert!(stdout.contains("14000 rows"), "{stdout}");
    // The file written is the one `encode` gives: same bytes as in memory.
    assert_eq!(bytes, encode(1).bytes);
    fs::remove_dir_all(&cache).unwrap();
}

#[test]
fn the_rules_of_out_are_those_of_ts_sim() {
    let cache = temp("rules");
    let outside = temp("outside");
    let scenario = Path::new(SCENARIOS).join("toy_short.ron");
    let seed: [&Path; 2] = ["--seed".as_ref(), "1".as_ref()];
    let run = |out: &Path| {
        ts_viz(
            &cache,
            &[&scenario, seed[0], seed[1], "--out".as_ref(), out],
        )
    };
    let out = cache.join("a");
    assert!(run(&out).status.success());
    // A second run into the same directory never overwrites the file.
    let before = fs::read(out.join(positions::FILE_NAME)).unwrap();
    assert!(!run(&out).status.success());
    assert_eq!(before, fs::read(out.join(positions::FILE_NAME)).unwrap());
    // `..` and a directory outside `TS_CACHE` are refused.
    assert!(!run(&cache.join("a").join("..").join("b")).status.success());
    assert!(!run(&outside).status.success());
    assert!(!outside.join(positions::FILE_NAME).exists());
    // A seed is an unsigned number.
    let o = ts_viz(
        &cache,
        &[
            &scenario,
            "--seed".as_ref(),
            "-1".as_ref(),
            "--out".as_ref(),
            &out,
        ],
    );
    assert!(!o.status.success());
    fs::remove_dir_all(&cache).unwrap();
    fs::remove_dir_all(&outside).unwrap();
}
