//! Local cache of what `verify-main` read on GitHub for each commit of `main` (issue #8, archi-sec
//! corrections V1 to V3). It holds **facts, never a verdict**: the PR (number, `merge_commit_sha`,
//! head), whether G3 found `ci-ok` green on that head, and whether `review-ok` and `archsec-ok`
//! were present on it. The local checks (squash shape, sensitive paths, J4 against §9) are redone
//! on every run. Key: `BUILT_FROM` (V2) and the SHA-256 of `tools/ci/sensitive-paths.txt` from
//! `origin/main`; any other value, or any read or format error, drops the whole cache and every
//! commit is checked in full. `TS_CACHE` is writable by every session, so an entry can be forged:
//! the nightly job runs `verify-main --no-cache` over the whole history (V3).
//! spec: carnet architecture-securite.md SEC0-17 ; §9.6 G3 ; §9.8 J3, J4

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::github::{Repo, is_sha};
use crate::{sensitive, sys};

const VERSION: u32 = 1;

/// GitHub facts about one commit of `main`. A `false` flag means « not read », never « read and
/// failed »: entries are only written after a full and successful check of the commit.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Facts {
    pub pr: u64,
    pub merge_commit_sha: String,
    pub head: String,
    /// G3: `ci-ok` green on `head`, from `ci.yml` of this repository.
    pub ci_ok: bool,
    /// First-line `review-ok: <head>` and `archsec-ok: <head>` comments present on the PR.
    pub review_ok: bool,
    pub archsec_ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    version: u32,
    built_from: String,
    sensitive_paths: String,
    commits: BTreeMap<String, Facts>,
}

pub struct Cache {
    path: PathBuf,
    built_from: String,
    sensitive_paths: String,
    commits: BTreeMap<String, Facts>,
    changed: bool,
}

/// Cache file under `TS_CACHE`, outside the repository.
pub fn path(repo: &Repo) -> PathBuf {
    repo.cache.join("verify-main").join("cache.json")
}

/// SHA-256 of `tools/ci/sensitive-paths.txt` in `origin/main`.
pub fn sensitive_fingerprint(repo: &Repo) -> Result<String, String> {
    let list = repo
        .blob(&repo.main, "origin/main", sensitive::LIST)?
        .ok_or_else(|| format!("{} absent d'origin/main", sensitive::LIST))?;
    Ok(crate::sha256::hex(&list))
}

/// Entries of the cache file if it exists, is well formed and has this key; `Err(reason)` else.
fn load(
    path: &Path,
    built_from: &str,
    sensitive_paths: &str,
) -> Result<BTreeMap<String, Facts>, String> {
    if !path.exists() {
        return Err("no cache file".to_owned());
    }
    let file: File =
        serde_json::from_slice(&sys::read(path)?).map_err(|e| format!("unreadable cache ({e})"))?;
    if file.version != VERSION {
        return Err(format!("cache version {}", file.version));
    }
    if file.built_from != built_from {
        return Err("written by an xtask built from other inputs (BUILT_FROM)".to_owned());
    }
    if file.sensitive_paths != sensitive_paths {
        return Err(format!("{} changed", sensitive::LIST));
    }
    for (sha, f) in &file.commits {
        if !is_sha(sha) || f.merge_commit_sha != *sha || !is_sha(&f.head) || f.pr == 0 {
            return Err(format!("malformed entry for {sha}"));
        }
    }
    Ok(file.commits)
}

impl Cache {
    /// The cache for this run; empty with `--no-cache` (`use_cache` false) or when it cannot be
    /// used, in which case every commit is checked in full.
    pub fn open(repo: &Repo, use_cache: bool) -> Result<Self, String> {
        let built_from = crate::merge::BUILT_FROM;
        Self::open_with(
            path(repo),
            built_from,
            &sensitive_fingerprint(repo)?,
            use_cache,
        )
    }

    pub fn open_with(
        path: PathBuf,
        built_from: &str,
        sensitive_paths: &str,
        use_cache: bool,
    ) -> Result<Self, String> {
        let commits = if use_cache {
            load(&path, built_from, sensitive_paths).unwrap_or_else(|why| {
                println!("  cache not used ({why}): full check");
                BTreeMap::new()
            })
        } else {
            println!("  --no-cache: full check, cache rewritten from what is read now");
            BTreeMap::new()
        };
        Ok(Self {
            path,
            built_from: built_from.to_owned(),
            sensitive_paths: sensitive_paths.to_owned(),
            commits,
            changed: !use_cache,
        })
    }

    pub fn get(&self, sha: &str) -> Option<&Facts> {
        self.commits.get(sha)
    }

    /// Records the facts of a commit whose full check succeeded.
    pub fn put(&mut self, sha: &str, facts: Facts) {
        if self.commits.get(sha) != Some(&facts) {
            self.commits.insert(sha.to_owned(), facts);
            self.changed = true;
        }
    }

    /// Writes the cache if it changed, by atomic replacement; a failure is only a warning.
    pub fn save(&self) {
        if !self.changed {
            return;
        }
        let file = File {
            version: VERSION,
            built_from: self.built_from.clone(),
            sensitive_paths: self.sensitive_paths.clone(),
            commits: self.commits.clone(),
        };
        let tmp = self
            .path
            .with_extension(format!("{}.tmp", std::process::id()));
        let written = self
            .path
            .parent()
            .map_or(Ok(()), |d| {
                std::fs::create_dir_all(d).map_err(|e| format!("{} : {e}", d.display()))
            })
            .and_then(|()| serde_json::to_vec_pretty(&file).map_err(|e| e.to_string()))
            .and_then(|bytes| sys::write(&tmp, &bytes))
            .and_then(|()| sys::rename(&tmp, &self.path));
        if let Err(e) = written {
            let _ = std::fs::remove_file(&tmp); // our own temporary file only
            println!("  AVERTISSEMENT : cache of verify-main not written ({e})");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "e3e2f2a842612bebfba1a8388f86fd296f64344c";
    const HEAD: &str = "c41e984ba8594dedd592744fcb69a70f3716cd87";

    fn facts() -> Facts {
        Facts {
            pr: 7,
            merge_commit_sha: SHA.to_owned(),
            head: HEAD.to_owned(),
            ci_ok: true,
            ..Facts::default()
        }
    }

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("xtask-cache-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d); // leftover of a previous run of this test only
        d
    }

    /// Absent, corrupted, other key or malformed entry: the cache is ignored, never trusted.
    #[test]
    fn unusable_cache_is_ignored() {
        let d = dir("ignored");
        let p = d.join("verify-main").join("cache.json");
        assert!(
            Cache::open_with(p.clone(), "b", "s", true)
                .unwrap()
                .commits
                .is_empty()
        );
        let mut c = Cache::open_with(p.clone(), "b", "s", true).unwrap();
        c.put(SHA, facts());
        c.save();
        assert_eq!(
            Cache::open_with(p.clone(), "b", "s", true)
                .unwrap()
                .get(SHA),
            Some(&facts())
        );
        // sensitive-paths.txt or BUILT_FROM changed.
        assert!(
            Cache::open_with(p.clone(), "b", "s2", true)
                .unwrap()
                .get(SHA)
                .is_none()
        );
        assert!(
            Cache::open_with(p.clone(), "b2", "s", true)
                .unwrap()
                .get(SHA)
                .is_none()
        );
        // --no-cache reads nothing.
        assert!(
            Cache::open_with(p.clone(), "b", "s", false)
                .unwrap()
                .get(SHA)
                .is_none()
        );
        let text = sys::read_text(&p).unwrap();
        for bad in [
            text[..text.len() / 2].to_owned(),
            String::new(),
            text.replace("\"ci_ok\"", "\"verdict\": true, \"ci_ok\""),
            text.replace(HEAD, "HEAD"),
            text.replace(
                &format!("\"merge_commit_sha\": \"{SHA}\""),
                &format!("\"merge_commit_sha\": \"{HEAD}\""),
            ),
            text.replace("\"version\": 1", "\"version\": 2"),
        ] {
            sys::write(&p, bad.as_bytes()).unwrap();
            assert!(load(&p, "b", "s").is_err(), "{bad}");
            assert!(
                Cache::open_with(p.clone(), "b", "s", true)
                    .unwrap()
                    .get(SHA)
                    .is_none()
            );
        }
        let _ = std::fs::remove_dir_all(&d); // temporary files of this test only
    }
}
