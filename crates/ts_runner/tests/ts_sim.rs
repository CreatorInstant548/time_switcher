//! `ts-sim run` end to end: golden hash of the toy world, reproducible outputs without local
//! path, readable Parquet, and the rules of `--out` (Y3, A6).
#![allow(clippy::disallowed_methods, clippy::unwrap_used)] // test: throwaway directories.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use ts_runner::out_dir::{OutError, cache_root, prepare};

const SCENARIOS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/scenarios");
const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/golden/ts_toy.json");

/// A fresh directory outside any Git working copy (the target directory may be in one).
fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("ts_runner_{name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn ts_sim(cache: &Path, args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ts-sim"))
        .env("TS_CACHE", cache)
        .arg("run")
        .args(args)
        .output()
        .unwrap()
}

/// Value of `key` in the golden file of `ts_toy`.
fn golden(key: &str) -> String {
    let text = fs::read_to_string(GOLDEN).unwrap();
    let start = text.find(&format!("\"{key}\": \"")).unwrap() + key.len() + 5;
    text[start..start + text[start..].find('"').unwrap()].to_owned()
}

#[test]
fn short_run_gives_the_golden_hash_and_the_same_bytes_twice() {
    let cache = temp("cache_short");
    let scenario = Path::new(SCENARIOS).join("toy_short.ron");
    let mut outputs = Vec::new();
    for name in ["a", "b/c"] {
        let out = cache.join(name);
        let seed = Path::new("1");
        let o = ts_sim(
            &cache,
            &[&scenario, "--seed".as_ref(), seed, "--out".as_ref(), &out],
        );
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        let manifest = fs::read_to_string(out.join("manifest.ron")).unwrap();
        let metrics = fs::read(out.join("metrics.parquet")).unwrap();
        outputs.push((manifest, metrics, out));
    }
    let (manifest, metrics, out) = &outputs[0];
    assert_eq!((manifest, metrics), (&outputs[1].0, &outputs[1].1));
    // Same world as the determinism proof of ts_toy (seed 1, 14 days).
    for (field, key) in [
        ("state_hash", "short.world"),
        ("rules_fingerprint", "short.rules"),
        ("ticks", "short.ticks"),
        ("skipped_ticks", "short.skipped"),
    ] {
        let value = golden(key);
        assert!(manifest.contains(&format!("{field}: ")), "{field}");
        assert!(manifest.contains(&value), "{field} {value}\n{manifest}");
    }
    assert!(manifest.contains("kind: \"public_test\""));
    // SEC0-26: no local path in the manifest.
    let tmp = std::env::temp_dir();
    for local in [
        cache.to_str().unwrap(),
        tmp.to_str().unwrap(),
        ":\\",
        "/tmp",
        "/home",
    ] {
        assert!(!manifest.contains(local), "{local}");
    }
    // Readable Parquet: 14 days, every resident leaves and comes back twice a day.
    let reader =
        SerializedFileReader::new(fs::File::open(out.join("metrics.parquet")).unwrap()).unwrap();
    assert_eq!(reader.metadata().file_metadata().num_rows(), 14);
    let mut departures = 0;
    for row in reader.get_row_iter(None).unwrap() {
        departures += row.unwrap().get_long(3).unwrap();
    }
    assert_eq!(departures, 14 * 2 * 1000);
    // A second run into the same directory never overwrites it.
    let again = ts_sim(
        &cache,
        &[
            &scenario,
            "--seed".as_ref(),
            "1".as_ref(),
            "--out".as_ref(),
            out,
        ],
    );
    assert!(!again.status.success());
    fs::remove_dir_all(&cache).unwrap();
}

#[test]
fn outside_the_cache_only_with_the_option() {
    let cache = temp("cache_outside");
    let elsewhere = temp("elsewhere");
    let scenario = elsewhere.join("one_day.ron");
    fs::write(
        &scenario,
        "Scenario(name: \"one_day\", world: Toy, duration: Days(1))",
    )
    .unwrap();
    let out = elsewhere.join("run");
    let args = [
        &*scenario,
        "--seed".as_ref(),
        "7".as_ref(),
        "--out".as_ref(),
        &out,
    ];
    let refused = ts_sim(&cache, &args);
    assert!(!refused.status.success());
    assert!(!out.exists());
    let mut with_option = args.to_vec();
    with_option.push("--allow-outside-cache".as_ref());
    let o = ts_sim(&cache, &with_option);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(out.join("manifest.ron").is_file());
    for dir in [cache, elsewhere] {
        fs::remove_dir_all(dir).unwrap();
    }
}

#[test]
fn out_rules() {
    let raw = temp("cache_rules");
    let cache = cache_root(&raw).unwrap();
    // Strictly under TS_CACHE, created level by level.
    let run = prepare(&cache.join("x").join("y"), &cache, false).unwrap();
    assert!(run.is_dir() && run.starts_with(&cache));
    assert_eq!(prepare(&cache, &cache, false), Err(OutError::OutsideCache));
    assert_eq!(
        // `raw`: a verbatim `PathBuf` (`\\?\`) would resolve `..` while joining.
        prepare(&raw.join("x").join("..").join("z"), &cache, false),
        Err(OutError::BadPath)
    );
    // Inside a Git working copy: refused, even with the option (A6), and nothing created.
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("never_written");
    assert_eq!(prepare(&repo, &cache, true), Err(OutError::InRepository));
    assert!(!repo.exists());
    let repo_cache = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(cache_root(repo_cache), Err(OutError::BadCache));
    // A link inside TS_CACHE is never followed, even towards TS_CACHE itself.
    let target = raw.join("target_dir");
    fs::create_dir(&target).unwrap();
    let link = raw.join("link");
    let made = make_link(&target, &link);
    assert!(made || cfg!(not(windows)), "mklink /J failed");
    if made {
        assert_eq!(
            prepare(&link.join("run"), &cache, false),
            Err(OutError::Link)
        );
        assert_eq!(prepare(&link, &cache, true), Err(OutError::Link));
        assert!(!target.join("run").exists());
        assert_eq!(cache_root(&link), Err(OutError::BadCache));
    }
    fs::remove_dir_all(&raw).unwrap();
}

/// A symbolic link on Unix; a junction on Windows (no privilege needed).
fn make_link(target: &Path, link: &Path) -> bool {
    #[cfg(unix)]
    return std::os::unix::fs::symlink(target, link).is_ok();
    #[cfg(windows)]
    return Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(link)
        .arg(target)
        .output()
        .is_ok_and(|o| o.status.success());
}
