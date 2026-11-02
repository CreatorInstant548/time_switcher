//! Approval carried over a pure update from `main` (AS-40, W1 to W7). A `review-ok` or
//! `archsec-ok` comment on an earlier SHA `A` of a PR counts for its head `H` only when every
//! condition holds:
//! - W1: `A` is among the PR's commits listed by GitHub, and an ancestor of `H`;
//! - W2: the PR changes the same files at `A` and at `H` (merge base to commit, no renames), each
//!   with the same blob id and mode;
//! - W3: none of those files changed on `main` between the base of `A` and the base of `H`;
//! - W4: outside those files, the tree of `H` equals the tree of `main` at its base;
//! - W5: an `archsec-ok` is not carried over when `sensitive-paths.txt` at the base of `A` did not
//!   make the change sensitive.
//!
//! When W2 (same blob) or the file half of W3 fails, typically because `main` also changed a file
//! of the PR, AS-55 still carries the approval over if, on the local objects:
//! - X1: the first-parent chain from `H` down to `A` holds only merges of two parents, whose second
//!   parent is an ancestor of the reference `main`;
//! - X2: the tree of each of those merges is `git merge-tree --write-tree <p1> <p2>` (git 2.38 or
//!   later); a conflict, even resolved, fails. The replay reads the attributes of the current
//!   `main` (`.gitattributes` of the work tree); none sets `merge=` today. If one ever does, pin
//!   the source of the attributes (for instance `--attr-source`), or a driver could change what
//!   « clean merge » means between two runs (m2 of the #116 review);
//! - X3: the PR's diff at `H` (against its base) and at `A` (against its base) have the same lines
//!   byte for byte, file headers included; only `index`, `@@` and context lines may differ. A
//!   binary delta is relative to its preimage, which X3 does not see (the `index` line is
//!   dropped): that the head's binary files are A's plus `main`'s comes from X2, a binary file
//!   changed on both sides being a conflict (m3 of the #116 review);
//! - X4: when the head is not among the local objects (`verify-main`, CI included), it is fetched
//!   anonymously (`pull/<n>/head`, once, no tags, no submodules), and only if the API shows a PR
//!   from this repository (`head.repo.full_name`): a fork, or the field missing, refuses the carry
//!   over without any fetch.
//!
//! `merge` and `verify-main` call this same code (W6); `verify-main` then does J4 on `A`. `ci-ok`
//! stays required on `H` itself, by the callers (W7). Every PR fact is read on GitHub on each run,
//! and nothing is cached (V5, AS-39). The head must be based on the
//! reference `main`, and what lands on `main` must have the head's tree. `merge` reads the trees
//! of `A` and `H` from the objects fetched locally; `verify-main` reads them through the API,
//! because in CI it has no PR objects (no `git fetch`). The bases are `main` commits, read from
//! the local history. AS-55 reads the commits between `A` and `H` locally (X4 for the fetch).
//! spec: carnet architecture-securite.md §10 (AS-40, AS-55) ; §9.8 J4

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::github::{Repo, approved_shas, is_sha};
use crate::sensitive;

/// Path to (mode, object id), for blobs and submodules.
type Tree = BTreeMap<String, (String, String)>;

/// Earlier approved SHAs of the PR looked at, most recent first: bounds the GitHub reads.
const MAX_CANDIDATES: usize = 5;

/// AS-55: merges of `main` replayed between `A` and the head, at most (X1, X2).
const MAX_MERGES: usize = 20;

/// X3: `git diff` options that make its output independent of the user's configuration: no
/// external driver or text conversion, no renames (as W2), full binary patches, fixed prefixes,
/// short submodule lines, blank context lines kept as such.
const DIFF: [&str; 14] = [
    "-c",
    "diff.suppressBlankEmpty=false",
    "diff",
    "--no-ext-diff",
    "--no-textconv",
    "--no-color",
    "--no-renames",
    "--binary",
    "--submodule=short",
    "--ignore-submodules=none",
    "--src-prefix=a/",
    "--dst-prefix=b/",
    "--no-relative",
    "--end-of-options",
];

/// How an approval on an earlier SHA is carried over.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rule {
    /// W1 to W5 hold.
    As40,
    /// W2 or W3 failed, X1 to X3 hold.
    As55,
}

/// Paths whose entry differs between two trees.
fn changed(a: &Tree, b: &Tree) -> BTreeSet<String> {
    a.keys()
        .chain(b.keys())
        .filter(|p| a.get(*p) != b.get(*p))
        .cloned()
        .collect()
}

/// X3: the lines of the diff from `base` to `commit` that must stay byte for byte: every line but
/// `index` and `@@` lines and context lines. File headers (`diff --git`, modes, `---`, `+++`),
/// `+` and `-` lines, `\ No newline` and binary patches stay. A binary patch may be a delta from a
/// preimage named only by the dropped `index` line: X2, not X3, guarantees binary contents (m3).
/// Not `git patch-id`, which ignores white space (AS-55).
fn pr_diff(repo: &Repo, base: &str, commit: &str) -> Result<Vec<Vec<u8>>, String> {
    let mut args = DIFF.to_vec();
    args.extend([base, commit]);
    let out = repo.out(&repo.main, "git", &args)?;
    let kept =
        |l: &&[u8]| !(l.starts_with(b"index ") || l.starts_with(b"@@") || l.starts_with(b" "));
    Ok(out
        .split(|b| *b == b'\n')
        .filter(kept)
        .map(<[u8]>::to_vec)
        .collect())
}

/// X3: `None` when both lists of diff lines are equal, else the file header where they first
/// differ (empty before the first header).
fn first_difference(a: &[Vec<u8>], h: &[Vec<u8>]) -> Option<String> {
    let header = |l: &Vec<u8>| l.starts_with(b"diff --git ");
    let mut file = None;
    for i in 0..a.len().max(h.len()) {
        let (x, y) = (a.get(i), h.get(i));
        if x != y {
            let at = [x, y].into_iter().flatten().find(|l| header(l)).or(file);
            return Some(
                at.map(|l| String::from_utf8_lossy(l).into_owned())
                    .unwrap_or_default(),
            );
        }
        if let Some(l) = x.filter(|l| header(l)) {
            file = Some(l);
        }
    }
    None
}

/// The PR as its caller checks it.
pub struct Pr<'a> {
    pub number: u64,
    pub head: &'a str,
    /// `main` commit the head is checked against: `origin/main` for `merge`, the parent of the
    /// squash for `verify-main`.
    pub main: &'a str,
    /// Change whose sensitivity the caller found with the current list: `(from, to, paths)` (W5).
    /// `to` is the commit that lands on `main`: the head for `merge`, the squash for
    /// `verify-main`; its tree must be the head's.
    pub diff: (&'a str, &'a str, &'a [String]),
    /// The PR's objects are local (`merge`, after `git fetch origin pull/<n>/head`): the trees of
    /// `A` and `H` are read with `git ls-tree`. Otherwise (`verify-main`, CI included), by the API.
    pub local_objects: bool,
}

/// Head side, read once: base, tree, tree of the base, files changed by the PR.
struct HeadSide {
    base: String,
    tree: Tree,
    base_tree: Tree,
    files: BTreeSet<String>,
}

pub struct Carry<'a> {
    repo: &'a Repo,
    pr: Pr<'a>,
    commits: Option<Vec<String>>,
    head: Option<HeadSide>,
    /// W1 to W4 (or X1 to X3) per candidate, which do not depend on the tag: `Ok((base of A,
    /// rule))` or the failure.
    seen: BTreeMap<String, Result<(String, Rule), String>>,
    /// X4, decided once: `None` not yet tried; `Some(None)` fetched; `Some(Some(why))` refused.
    fetch: Option<Option<String>>,
}

impl<'a> Carry<'a> {
    pub fn new(repo: &'a Repo, pr: Pr<'a>) -> Self {
        Self {
            repo,
            pr,
            commits: None,
            head: None,
            seen: BTreeMap::new(),
            fetch: None,
        }
    }

    /// True when the approval on `from`, returned by [`Self::resolve`], was carried over by AS-55.
    pub fn by_as55(&self, from: &str) -> bool {
        matches!(self.seen.get(from), Some(Ok((_, Rule::As55))))
    }

    /// What `merge` prints after `carried over`, for an approval that [`Self::resolve`] carried
    /// over from `from` to the head.
    pub fn describe(&self, from: &str) -> String {
        let h = self.pr.head;
        if self.by_as55(from) {
            format!("carried over from {from} to {h} (AS-55: clean merges of main, identical diff)")
        } else {
            format!("carried over from {from} to the head {h} (AS-40, W1 to W5 hold)")
        }
    }

    /// SHA whose `<tag>` comment counts for the head: the head itself, else an earlier SHA of
    /// the PR for which W1 to W5 hold. Candidates go most recent first, those that satisfy
    /// `prefer` before the others (J4 for `verify-main`). `Ok(Err(why))` names the failed
    /// condition of each candidate; a read that fails for one candidate fails that candidate
    /// only. `Err` when the PR's commit list cannot be read.
    pub fn resolve(
        &mut self,
        tag: &str,
        bodies: &[String],
        prefer: &dyn Fn(&str) -> bool,
    ) -> Result<Result<String, String>, String> {
        let head = self.pr.head;
        let missing = format!("comment « {tag}: {head} » absent");
        let shas = approved_shas(bodies, tag);
        if shas.iter().any(|s| s == head) {
            return Ok(Ok(head.to_owned()));
        }
        if shas.is_empty() {
            return Ok(Err(format!(
                "{missing}, and no earlier « {tag} » to carry over (AS-40)"
            )));
        }
        // W1, first half, before the bound: an approval of a SHA foreign to the PR costs nothing.
        if self.commits.is_none() {
            self.commits = Some(self.pr_commits()?);
        }
        let commits = self.commits.as_deref().unwrap_or_default();
        let (mut listed, foreign): (Vec<String>, Vec<String>) =
            shas.into_iter().partition(|s| commits.contains(s));
        let mut why: Vec<String> = foreign
            .iter()
            .map(|a| format!("{a}: W1: not among the PR's commits listed by GitHub"))
            .collect();
        listed.truncate(MAX_CANDIDATES);
        listed.sort_by_key(|a| !prefer(a)); // stable: recency kept within each group
        for a in &listed {
            let pure = match self.seen.get(a) {
                Some(r) => r.clone(),
                None => {
                    let r = self.pure_update(a).unwrap_or_else(|e| {
                        // The read can be a `gh api` call or a local `git` one: `e` names it.
                        Err(format!("unreadable, not carried over ({e})"))
                    });
                    self.seen.insert(a.clone(), r.clone());
                    r
                }
            };
            let r = match pure {
                Ok((base, _)) if tag == "archsec-ok" => match self.sensitive_under(&base) {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(format!(
                        "W5: {} at {base} did not make the PR sensitive: a fresh archsec-ok is \
                         required",
                        sensitive::LIST
                    )),
                    Err(e) => Err(format!("W5 unreadable, not carried over ({e})")),
                },
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
            match r {
                Ok(()) => return Ok(Ok(a.clone())),
                Err(e) => why.push(format!("{a}: {e}")),
            }
        }
        Ok(Err(format!(
            "{missing}; not carried over (AS-40): {}",
            why.join("; ")
        )))
    }

    /// W1 (ancestry) to W4 for a SHA `a` listed among the PR's commits, or X1 to X3 when W2 or
    /// the file half of W3 fails (AS-55): `Ok(Ok((base of a, rule)))`, or `Ok(Err(condition))` on
    /// the first one that fails.
    fn pure_update(&mut self, a: &str) -> Result<Result<(String, Rule), String>, String> {
        let fail = |w: String| Ok(Err(w));
        if self.merge_base(a, self.pr.head)? != a {
            return fail("W1: not an ancestor of the head".to_owned());
        }
        if self.head.is_none() {
            let base = self.merge_base(self.pr.main, self.pr.head)?;
            let (tree, base_tree) = (self.pr_tree(self.pr.head)?, self.local_tree(&base)?);
            let files = changed(&base_tree, &tree);
            self.head = Some(HeadSide {
                base,
                tree,
                base_tree,
                files,
            });
        }
        let Some(h) = &self.head else {
            return Err("head side not read".to_owned());
        };
        // The head is up to date: its base is the reference `main` (m1 of the #60 review).
        if h.base != self.pr.main {
            return fail(format!(
                "W3, W4: the head's base {} is not the reference main {}",
                h.base, self.pr.main
            ));
        }
        // What lands on `main` is the head's tree (the squash, for verify-main).
        let landed = self.pr.diff.1;
        if landed != self.pr.head && self.local_tree(landed)? != h.tree {
            return fail(format!(
                "W4: the tree of {landed} on main differs from the head's tree"
            ));
        }
        let base = self.merge_base(self.pr.main, a)?;
        let (tree, base_tree) = (self.pr_tree(a)?, self.local_tree(&base)?);
        let files = changed(&base_tree, &tree);
        let main = &self.repo.main;
        if !self.repo.ok(
            main,
            "git",
            &["merge-base", "--is-ancestor", &base, &h.base],
        )? {
            return fail(format!(
                "W3: its base {base} is not an ancestor of the head's base {} on main",
                h.base
            ));
        }
        // `h.files` is every path where the head differs from its base: W4 is `h.files ⊆ files`.
        if let Some(p) = h.files.difference(&files).next() {
            return fail(format!(
                "W2, W4: {p} differs between the head and main at its base, but is not changed at \
                 {a} (file added to the PR, or changed outside its files)"
            ));
        }
        if let Some(p) = files.difference(&h.files).next() {
            return fail(format!(
                "W2: {p} is changed at {a} but no longer at the head (file removed from the PR)"
            ));
        }
        // W2 (same blob and mode) and the file half of W3: AS-55 may lift them.
        let lifted = if let Some(p) = files.iter().find(|f| tree.get(*f) != h.tree.get(*f)) {
            Some(format!(
                "W2: {p} has another blob or mode at the head than at {a} (conflict resolution?)"
            ))
        } else {
            files
                .iter()
                .find(|f| base_tree.get(*f) != h.base_tree.get(*f))
                .map(|p| {
                    format!(
                        "W3: {p} changed on main between {base} and the head's base {}",
                        h.base
                    )
                })
        };
        let Some(w) = lifted else {
            return Ok(Ok((base, Rule::As40)));
        };
        match self.clean_merges(a, &base) {
            Ok(Ok(())) => Ok(Ok((base, Rule::As55))),
            Ok(Err(x)) => fail(format!("{w}; AS-55: {x}")),
            Err(e) => Err(format!("{w}; AS-55: {e}")),
        }
    }

    /// AS-55, X4 then X1 to X3, on the local objects: `Ok(Err(condition))` on the first one that
    /// fails.
    fn clean_merges(&mut self, a: &str, base_a: &str) -> Result<Result<(), String>, String> {
        let fail = |x: String| Ok(Err(x));
        if let Err(x4) = self.pr_objects()? {
            return fail(x4);
        }
        let (head, main) = (self.pr.head, self.pr.main);
        // X1: walk the first parents from the head down to `a`; one more line than the bound
        // tells a longer chain from one that ends on `a`.
        let max = format!("--max-count={}", MAX_MERGES + 1);
        let chain = self.repo.git(&[
            "rev-list",
            "--first-parent",
            "--parents",
            &max,
            "--end-of-options",
            head,
        ])?;
        let mut merges = Vec::new();
        let mut next = head;
        let mut reached = false;
        for line in chain.lines() {
            let ids: Vec<&str> = line.split(' ').collect();
            if ids.first() != Some(&next) || !ids.iter().all(|s| is_sha(s)) {
                return Err(format!("unreadable first-parent chain of {head}"));
            }
            if next == a {
                reached = true;
                break;
            }
            let [c, p1, p2] = ids[..] else {
                return fail(format!(
                    "X1: {next} is not a merge of two parents (ordinary commit between {a} and \
                     the head)"
                ));
            };
            merges.push((c, p1, p2));
            next = p1;
        }
        if !reached || merges.is_empty() {
            return fail(format!(
                "X1: {a} is not on the head's first-parent chain within {MAX_MERGES} merges"
            ));
        }
        for (c, p1, p2) in merges {
            let ancestor = ["merge-base", "--is-ancestor", p2, main];
            if !self.repo.ok(&self.repo.main, "git", &ancestor)? {
                return fail(format!(
                    "X1: the second parent {p2} of {c} is not an ancestor of main {main}"
                ));
            }
            // X2: replay the merge; exit 1 is a conflict, anything else but 0 a failed read.
            let args = ["merge-tree", "--write-tree", "--end-of-options", p1, p2];
            let (code, out, err) = self.repo.runner.run(&self.repo.main, "git", &args)?;
            let replayed = match code {
                Some(0) => String::from_utf8_lossy(&out)
                    .lines()
                    .next()
                    .filter(|t| is_sha(t))
                    .map(str::to_owned)
                    .ok_or_else(|| format!("unreadable git merge-tree output for {c}"))?,
                Some(1) => {
                    return fail(format!(
                        "X2: merging {p2} into {p1} conflicts: {c} was resolved by hand"
                    ));
                }
                _ => {
                    return Err(format!(
                        "git merge-tree --write-tree failed for {c} (git 2.38 or later is \
                         needed): {}",
                        err.trim()
                    ));
                }
            };
            if self.repo.git(&["rev-parse", &format!("{c}^{{tree}}")])? != replayed {
                return fail(format!(
                    "X2: the tree of {c} is not the clean merge of its parents (edited merge)"
                ));
            }
        }
        // X3: the PR's lines, byte for byte, at `a` and at the head.
        let at_a = pr_diff(self.repo, base_a, a)?;
        let at_h = pr_diff(self.repo, main, head)?;
        if let Some(at) = first_difference(&at_a, &at_h) {
            return fail(format!(
                "X3: the PR's diff at the head differs from its diff at {a}, byte for byte, in « \
                 {at} »"
            ));
        }
        Ok(Ok(()))
    }

    /// AS-55 reads the commits between `A` and the head locally. `merge` has fetched them;
    /// `verify-main` fetches the PR's head ref, once, when the head is not already local, and
    /// only for a PR of this repository (X4): `Ok(Err(why))` refuses without fetching.
    fn pr_objects(&mut self) -> Result<Result<(), String>, String> {
        let spec = format!("{}^{{commit}}", self.pr.head);
        let local = |c: &Self| c.repo.ok(&c.repo.main, "git", &["cat-file", "-e", &spec]);
        if local(&*self)? {
            return Ok(Ok(()));
        }
        let absent = format!("head {} absent from the local objects", self.pr.head);
        match &self.fetch {
            _ if self.pr.local_objects => return Err(absent),
            Some(Some(why)) => return Ok(Err(why.clone())),
            Some(None) => return Err(absent),
            None => {}
        }
        if let Some(why) = self.foreign()? {
            self.fetch = Some(Some(why.clone()));
            return Ok(Err(why));
        }
        self.fetch = Some(None);
        let r = format!("pull/{}/head", self.pr.number);
        self.repo.git(&[
            "fetch",
            "--quiet",
            "--no-tags",
            "--no-recurse-submodules",
            "origin",
            &r,
        ])?;
        if local(&*self)? {
            Ok(Ok(()))
        } else {
            Err(absent)
        }
    }

    /// X4: `None` when the API shows the PR's head in this repository, else why it is refused
    /// (a fork, or the field missing, for instance once the fork is deleted).
    fn foreign(&self) -> Result<Option<String>, String> {
        let (s, n) = (&self.repo.slug, self.pr.number);
        let pr = self.repo.api(&format!("repos/{s}/pulls/{n}"))?;
        Ok(match pr["head"]["repo"]["full_name"].as_str() {
            Some(name) if name == s.as_str() => None,
            Some(name) => Some(format!(
                "X4: PR #{n} comes from the fork {name}: its objects are never fetched"
            )),
            None => Some(format!(
                "X4: the head repository of PR #{n} is unreadable on GitHub (deleted fork?): \
                 nothing fetched"
            )),
        })
    }

    /// W5: the caller's change is sensitive under the list of `base`. A missing or unreadable
    /// list counts as « not sensitive », which requires a fresh `archsec-ok`.
    fn sensitive_under(&self, base: &str) -> Result<bool, String> {
        let text = self.repo.blob(&self.repo.main, base, sensitive::LIST)?;
        let rules = text
            .and_then(|t| String::from_utf8(t).ok())
            .and_then(|t| sensitive::Rules::parse(&t).ok());
        let Some(rules) = rules else {
            return Ok(false);
        };
        let (from, to, paths) = self.pr.diff;
        Ok(!sensitive::hits(self.repo, &rules, from, to, paths)?.is_empty())
    }

    /// Commits of the PR, as GitHub lists them (W1; at most 250).
    fn pr_commits(&self) -> Result<Vec<String>, String> {
        let (s, n) = (&self.repo.slug, self.pr.number);
        let list = self
            .repo
            .api_all(&format!("repos/{s}/pulls/{n}/commits?per_page=100"))?;
        list.iter()
            .map(|c| {
                c["sha"]
                    .as_str()
                    .filter(|s| is_sha(s))
                    .map(str::to_owned)
                    .ok_or_else(|| format!("unreadable commit in the list of PR #{n}"))
            })
            .collect()
    }

    /// Merge base of `a` and `b`, by the GitHub compare API.
    fn merge_base(&self, a: &str, b: &str) -> Result<String, String> {
        let s = &self.repo.slug;
        let c = self
            .repo
            .api(&format!("repos/{s}/compare/{a}...{b}?per_page=1"))?;
        c["merge_base_commit"]["sha"]
            .as_str()
            .filter(|s| is_sha(s))
            .map(str::to_owned)
            .ok_or_else(|| format!("unreadable merge base of {a} and {b}"))
    }

    /// Tree of a PR commit: local when its objects are, else by the API (m2 of the #60 review).
    fn pr_tree(&self, sha: &str) -> Result<Tree, String> {
        if self.pr.local_objects {
            self.local_tree(sha)
        } else {
            self.api_tree(sha)
        }
    }

    /// Tree of a PR commit, by the API; a truncated answer is an error.
    fn api_tree(&self, sha: &str) -> Result<Tree, String> {
        let s = &self.repo.slug;
        let v = self
            .repo
            .api(&format!("repos/{s}/git/trees/{sha}?recursive=1"))?;
        if v["truncated"] != Value::Bool(false) {
            return Err(format!("tree of {sha} truncated or unreadable"));
        }
        let mut tree = Tree::new();
        for e in v["tree"].as_array().ok_or("tree without entries")? {
            let field = |k: &str| {
                e[k].as_str()
                    .ok_or_else(|| format!("tree entry without {k}"))
            };
            if matches!(field("type")?, "blob" | "commit") {
                let entry = (field("mode")?.to_owned(), field("sha")?.to_owned());
                tree.insert(field("path")?.to_owned(), entry);
            }
        }
        Ok(tree)
    }

    /// Tree of a commit from the local objects: a `main` commit, or a PR commit after a fetch.
    fn local_tree(&self, sha: &str) -> Result<Tree, String> {
        let ls = self
            .repo
            .git(&["ls-tree", "-r", "-z", "--full-tree", sha])?;
        let mut tree = Tree::new();
        for entry in ls.split('\0').filter(|e| !e.is_empty()) {
            let parsed = entry.split_once('\t').and_then(|(meta, path)| {
                match meta.split(' ').collect::<Vec<_>>()[..] {
                    [mode, _, oid] => Some((path, mode, oid)),
                    _ => None,
                }
            });
            let (path, mode, oid) = parsed.ok_or("unreadable git ls-tree output")?;
            tree.insert(path.to_owned(), (mode.to_owned(), oid.to_owned()));
        }
        Ok(tree)
    }
}

/// Small local Git repositories for the tests, behind a fake `gh` whose trees and merge bases are
/// computed from them, as GitHub would.
#[cfg(test)]
pub mod lab {
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::rc::Rc;

    use serde_json::json;

    use crate::github::{Output, Repo, Run};

    /// Identity and settings of the throwaway repositories, whatever the user's configuration;
    /// no global hook and no template run in them (m5 of the #60 review).
    const CONFIG: [&str; 14] = [
        "-c",
        "user.name=lab",
        "-c",
        "user.email=lab@example.invalid",
        "-c",
        "commit.gpgsign=false",
        "-c",
        "core.autocrlf=false",
        "-c",
        "init.defaultBranch=main",
        "-c",
        "core.hooksPath=",
        "-c",
        "init.templateDir=",
    ];

    /// Answer that the fake `gh` turns into a failed read (HTTP 404).
    pub const FAIL: &str = "!fail";

    /// `git` in `dir` with the lab's settings, outside any caller's repository.
    fn command(dir: &Path, args: &[&str]) -> Command {
        let mut cmd = Command::new("git");
        cmd.args(CONFIG).args(args).current_dir(dir);
        for v in [
            "GIT_DIR",
            "GIT_WORK_TREE",
            "GIT_INDEX_FILE",
            "GIT_COMMON_DIR",
            // Set by `cargo xtask check` for its children (#63, #85); the lab must not depend on
            // them.
            "GIT_NO_REPLACE_OBJECTS",
            "GIT_GRAFT_FILE",
        ] {
            cmd.env_remove(v);
        }
        cmd
    }

    pub fn git_in(dir: &Path, args: &[&str]) -> Output {
        let out = command(dir, args).output().expect("git");
        (
            out.status.code(),
            out.stdout,
            String::from_utf8_lossy(&out.stderr).into_owned(),
        )
    }

    /// `git` in `dir` with `input` on its standard input; its standard output.
    pub fn git_input(dir: &Path, args: &[&str], input: &[u8]) -> Vec<u8> {
        use std::io::Write;
        use std::process::Stdio;
        let mut child = command(dir, args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("git");
        child.stdin.take().expect("stdin").write_all(input).unwrap();
        child.wait_with_output().expect("git").stdout
    }

    pub struct Lab {
        pub dir: PathBuf,
    }

    impl Drop for Lab {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir); // this test's throwaway repository only
        }
    }

    impl Lab {
        pub fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("xtask-lab-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir); // leftover of this test only
            std::fs::create_dir_all(&dir).unwrap();
            let lab = Self { dir };
            lab.git(&["init", "-q"]);
            lab
        }

        /// Runs git, which must succeed; trimmed output.
        pub fn git(&self, args: &[&str]) -> String {
            let (code, out, err) = git_in(&self.dir, args);
            assert_eq!(code, Some(0), "git {args:?}: {err}");
            String::from_utf8(out).unwrap().trim().to_owned()
        }

        /// Writes (`Some`) or removes (`None`) files, then commits them all; the new SHA.
        pub fn commit(&self, files: &[(&str, Option<&str>)]) -> String {
            for (path, text) in files {
                let p = self.dir.join(path);
                match text {
                    Some(t) => {
                        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                        crate::sys::write(&p, t.as_bytes()).unwrap();
                    }
                    None => std::fs::remove_file(&p).unwrap(),
                }
            }
            self.git(&["add", "-A"]);
            self.git(&["commit", "-q", "--allow-empty", "-m", "x"]);
            self.git(&["rev-parse", "HEAD"])
        }

        /// A repository whose `git` is this one and whose `gh` is [`Hub`] with `answers`.
        pub fn repo(&self, answers: Vec<(String, String)>) -> (Repo, Rc<RefCell<Vec<String>>>) {
            let calls = Rc::new(RefCell::new(Vec::new()));
            let repo = Repo {
                main: self.dir.clone(),
                here: self.dir.clone(),
                slug: "o/r".to_owned(),
                dry_run: true,
                ci: false,
                cache: self.dir.join(".cache"),
                runner: Box::new(Hub {
                    answers,
                    calls: Rc::clone(&calls),
                }),
            };
            (repo, calls)
        }
    }

    /// A call takes the answer of the longest matching prefix ([`FAIL`]: a failed read); without
    /// one, `git` runs for real in the lab, and `gh api` trees and compare are computed from the
    /// lab, as GitHub would. Every call is noted.
    pub struct Hub {
        answers: Vec<(String, String)>,
        calls: Rc<RefCell<Vec<String>>>,
    }

    impl Run for Hub {
        fn run(&self, dir: &Path, prog: &str, args: &[&str]) -> Result<Output, String> {
            let line = format!("{prog} {}", args.join(" "));
            self.calls.borrow_mut().push(line.clone());
            let best = self
                .answers
                .iter()
                .filter(|(k, _)| line.starts_with(k.as_str()))
                .max_by_key(|(k, _)| k.len());
            match best {
                Some((_, out)) if out == FAIL => {
                    return Ok((Some(1), Vec::new(), "gh: Not Found (HTTP 404)".to_owned()));
                }
                Some((_, out)) => return Ok((Some(0), out.clone().into_bytes(), String::new())),
                None if prog == "git" => return Ok(git_in(dir, args)),
                None => {}
            }
            let text = |a: &[&str]| String::from_utf8(git_in(dir, a).1).unwrap();
            let ok =
                |v: serde_json::Value| Ok((Some(0), v.to_string().into_bytes(), String::new()));
            if let Some(rest) = line.strip_prefix("gh api repos/o/r/git/trees/") {
                let sha = rest.split('?').next().unwrap_or_default();
                let ls = text(&["ls-tree", "-r", "-t", "-z", "--full-tree", sha]);
                let tree: Vec<_> = ls
                    .split('\0')
                    .filter(|e| !e.is_empty())
                    .map(|e| {
                        let (meta, path) = e.split_once('\t').unwrap();
                        let m: Vec<&str> = meta.split(' ').collect();
                        json!({"path": path, "mode": m[0], "type": m[1], "sha": m[2]})
                    })
                    .collect();
                return ok(json!({"sha": sha, "truncated": false, "tree": tree}));
            }
            if let Some(rest) = line.strip_prefix("gh api repos/o/r/compare/") {
                let spec = rest.split('?').next().unwrap_or_default();
                let (a, b) = spec.split_once("...").unwrap();
                let base = text(&["merge-base", a, b]);
                return ok(json!({"merge_base_commit": {"sha": base.trim()}}));
            }
            Err(format!("unexpected command: {line}"))
        }
    }

    pub const PR_FILE: &str = "tools/xtask/a.rs";

    /// `c0` on main; `a` on the PR changes [`PR_FILE`]; `m1` on main changes `docs/x.md`; `h`
    /// merges `m1` into the PR: the nominal pure update from `main`.
    pub struct Case {
        pub lab: Lab,
        pub c0: String,
        pub a: String,
        pub m1: String,
        pub h: String,
    }

    pub fn case(name: &str) -> Case {
        let lab = Lab::new(name);
        let c0 = lab.commit(&[
            ("tools/ci/sensitive-paths.txt", Some("tools/xtask/\n")),
            (PR_FILE, Some("a0\n")),
            ("docs/x.md", Some("x0\n")),
            ("other.md", Some("o0\n")),
        ]);
        lab.git(&["checkout", "-q", "-b", "pr"]);
        let a = lab.commit(&[(PR_FILE, Some("a1\n"))]);
        lab.git(&["checkout", "-q", "main"]);
        let m1 = lab.commit(&[("docs/x.md", Some("x1\n"))]);
        lab.git(&["checkout", "-q", "pr"]);
        lab.git(&["merge", "-q", "--no-edit", &m1]);
        let h = lab.git(&["rev-parse", "HEAD"]);
        Case { lab, c0, a, m1, h }
    }

    /// Answer of `pulls/7/commits`.
    pub fn commits(list: &[&str]) -> Vec<(String, String)> {
        let list: Vec<_> = list.iter().map(|s| json!({"sha": s})).collect();
        vec![(
            "gh api --paginate repos/o/r/pulls/7/commits".to_owned(),
            json!(list).to_string(),
        )]
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::lab::{Case, Lab, PR_FILE, case, commits};
    use super::*;

    /// `resolve` of `tag` for the head `h`, checked against `main` = `m1`, with one comment
    /// approving `approved` and the PR commits `list`.
    fn resolve(
        c: &Case,
        h: &str,
        list: &[&str],
        tag: &str,
        approved: &str,
    ) -> Result<String, String> {
        let (repo, _) = c.lab.repo(commits(list));
        let files = c
            .lab
            .git(&["diff", "--name-only", "--no-renames", &c.m1, h]);
        let files: Vec<String> = files.lines().map(str::to_owned).collect();
        let pr = Pr {
            number: 7,
            head: h,
            main: &c.m1,
            diff: (&c.m1, h, &files),
            local_objects: false,
        };
        let bodies = [format!("{tag}: {approved}\nok")];
        Carry::new(&repo, pr)
            .resolve(tag, &bodies, &|_| true)
            .unwrap()
    }

    /// Nominal: a merge of `main` that touches nothing of the PR carries both approvals over; an
    /// approval on the head itself reads nothing on GitHub.
    #[test]
    fn nominal_update_from_main_carries_over() {
        let c = case("nominal");
        for tag in ["review-ok", "archsec-ok"] {
            assert_eq!(resolve(&c, &c.h, &[&c.a, &c.h], tag, &c.a), Ok(c.a.clone()));
        }
        let (repo, calls) = c.lab.repo(Vec::new());
        let pr = Pr {
            number: 7,
            head: &c.h,
            main: &c.m1,
            diff: (&c.m1, &c.h, &[]),
            local_objects: false,
        };
        let bodies = [format!("review-ok: {}", c.a), format!("review-ok: {}", c.h)];
        let mut carry = Carry::new(&repo, pr);
        assert_eq!(
            carry.resolve("review-ok", &bodies, &|_| true),
            Ok(Ok(c.h.clone()))
        );
        assert!(calls.borrow().is_empty(), "{:?}", calls.borrow());
        let e = carry
            .resolve("archsec-ok", &bodies, &|_| true)
            .unwrap()
            .unwrap_err();
        assert!(e.contains("no earlier"), "{e}");
    }

    /// W1: `A` missing from the PR's commits on GitHub, or not an ancestor of the head.
    #[test]
    fn w1_same_pr() {
        let c = case("w1");
        let e = resolve(&c, &c.h, &[&c.h], "review-ok", &c.a).unwrap_err();
        assert!(e.contains("W1: not among"), "{e}");
        // Same change as `a`, on another branch: listed, but not an ancestor.
        c.lab.git(&["checkout", "-q", "-b", "other", &c.c0]);
        let twin = c
            .lab
            .commit(&[(PR_FILE, Some("a1\n")), ("twin", Some("t\n"))]);
        let e = resolve(&c, &c.h, &[&c.a, &twin, &c.h], "review-ok", &twin).unwrap_err();
        assert!(e.contains("W1: not an ancestor"), "{e}");
    }

    /// W2: a conflict resolution, or any edit, in a PR file during the merge changes its blob.
    #[test]
    fn w2_conflict_resolution_in_a_pr_file() {
        let c = case("w2-blob");
        c.lab.git(&["checkout", "-q", "-b", "resolved", &c.a]);
        c.lab.git(&["merge", "-q", "--no-commit", &c.m1]);
        let h = c.lab.commit(&[(PR_FILE, Some("a1 resolved\n"))]);
        let e = resolve(&c, &h, &[&c.a, &h], "review-ok", &c.a).unwrap_err();
        assert!(
            e.contains(&format!("W2: {PR_FILE} has another blob")),
            "{e}"
        );
    }

    /// W2: a file added to the PR after `A`, or a PR file brought back to `main`'s version.
    #[test]
    fn w2_file_added_or_removed() {
        let c = case("w2-set");
        let added = c.lab.commit(&[("tools/xtask/b.rs", Some("b\n"))]);
        let e = resolve(&c, &added, &[&c.a, &c.h, &added], "review-ok", &c.a).unwrap_err();
        assert!(e.contains("W2, W4: tools/xtask/b.rs"), "{e}");
        // `a2` changes two files; then the PR gives one of them up.
        c.lab.git(&["checkout", "-q", "-b", "two", &c.a]);
        let a2 = c.lab.commit(&[("docs/y.md", Some("y\n"))]);
        c.lab.git(&["merge", "-q", "--no-edit", &c.m1]);
        let removed = c.lab.commit(&[("docs/y.md", None)]);
        let list: [&str; 3] = [&c.a, &a2, &removed];
        let e = resolve(&c, &removed, &list, "review-ok", &a2).unwrap_err();
        assert!(e.contains("W2: docs/y.md is changed at"), "{e}");
    }

    /// W3: `main` changed a PR file, and the merge kept the PR's version: same blob, but `main`'s
    /// change is silently dropped.
    #[test]
    fn w3_pr_file_changed_on_main() {
        let c = case("w3");
        c.lab.git(&["checkout", "-q", "main"]);
        let m2 = c.lab.commit(&[(PR_FILE, Some("a2\n"))]);
        c.lab.git(&["checkout", "-q", "pr"]);
        c.lab.git(&["merge", "-q", "--no-edit", "-X", "ours", &m2]);
        let h = c.lab.git(&["rev-parse", "HEAD"]);
        let (repo, _) = c.lab.repo(commits(&[&c.a, &c.h, &h]));
        let files = [PR_FILE.to_owned()];
        let pr = Pr {
            number: 7,
            head: &h,
            main: &m2,
            diff: (&m2, &h, &files),
            local_objects: false,
        };
        let bodies = [format!("review-ok: {}", c.a)];
        let e = Carry::new(&repo, pr)
            .resolve("review-ok", &bodies, &|_| true)
            .unwrap();
        let e = e.unwrap_err();
        assert!(e.contains(&format!("W3: {PR_FILE} changed on main")), "{e}");
    }

    /// W4: after the merge, a file outside the PR differs from `main` at the head's base.
    #[test]
    fn w4_file_outside_the_pr_differs() {
        let c = case("w4");
        let h = c.lab.commit(&[("other.md", Some("hidden\n"))]);
        let e = resolve(&c, &h, &[&c.a, &c.h, &h], "review-ok", &c.a).unwrap_err();
        assert!(e.contains("W2, W4: other.md differs"), "{e}");
    }

    /// W5: `main` makes the PR sensitive after `A`: `review-ok` is carried over, `archsec-ok` is
    /// not.
    #[test]
    fn w5_sensitive_list_change_needs_a_fresh_archsec_ok() {
        let lab = Lab::new("w5");
        let list = "tools/ci/sensitive-paths.txt";
        lab.commit(&[
            (list, Some("tools/xtask/\n")),
            ("secret/s.md", Some("s0\n")),
        ]);
        lab.git(&["checkout", "-q", "-b", "pr"]);
        let a = lab.commit(&[("secret/s.md", Some("s1\n"))]);
        lab.git(&["checkout", "-q", "main"]);
        let m1 = lab.commit(&[(list, Some("tools/xtask/\nsecret/\n"))]);
        lab.git(&["checkout", "-q", "pr"]);
        lab.git(&["merge", "-q", "--no-edit", &m1]);
        let h = lab.git(&["rev-parse", "HEAD"]);
        let files = ["secret/s.md".to_owned()];
        for (tag, carried) in [("review-ok", true), ("archsec-ok", false)] {
            let (repo, _) = lab.repo(commits(&[&a, &h]));
            let pr = Pr {
                number: 7,
                head: &h,
                main: &m1,
                diff: (&m1, &h, &files),
                local_objects: false,
            };
            let bodies = [format!("{tag}: {a}")];
            let r = Carry::new(&repo, pr)
                .resolve(tag, &bodies, &|_| true)
                .unwrap();
            if carried {
                assert_eq!(r, Ok(a.clone()));
            } else {
                let e = r.unwrap_err();
                assert!(e.contains("W5:"), "{e}");
            }
        }
    }

    /// SHAs foreign to the PR do not use up the bound; `prefer` goes before recency; W1 to W4 are
    /// read once for both tags.
    #[test]
    fn candidate_order() {
        let c = case("order");
        let h2 = c.lab.commit(&[]);
        let (repo, calls) = c.lab.repo(commits(&[&c.a, &c.h, &h2]));
        let pr = Pr {
            number: 7,
            head: &h2,
            main: &c.m1,
            diff: (&c.m1, &h2, &[]),
            local_objects: false,
        };
        let mut bodies = vec![format!("review-ok: {}", c.a)];
        bodies.extend((0..6).map(|i| format!("review-ok: {}", format!("{i}").repeat(40))));
        let mut carry = Carry::new(&repo, pr);
        assert_eq!(
            carry.resolve("review-ok", &bodies, &|_| true),
            Ok(Ok(c.a.clone()))
        );
        let both = [format!("review-ok: {}", c.a), format!("review-ok: {}", c.h)];
        assert_eq!(
            carry.resolve("review-ok", &both, &|_| true),
            Ok(Ok(c.h.clone()))
        );
        let prefer_a = |s: &str| s == c.a;
        assert_eq!(
            carry.resolve("review-ok", &both, &prefer_a),
            Ok(Ok(c.a.clone()))
        );
        let trees = calls
            .borrow()
            .iter()
            .filter(|l| l.contains("/git/trees/"))
            .count();
        assert_eq!(trees, 3, "head, a and h, each once");
    }

    /// `resolve` of `tag` with the comments `bodies`, for `head` against `main`, `landed` being
    /// what lands on `main`, with the `gh` answers `answers`.
    #[allow(clippy::too_many_arguments)]
    fn resolve_in(
        lab: &Lab,
        answers: Vec<(String, String)>,
        head: &str,
        main: &str,
        landed: &str,
        local_objects: bool,
        tag: &str,
        bodies: &[String],
    ) -> (Result<Result<String, String>, String>, Vec<String>) {
        let (repo, calls) = lab.repo(answers);
        let files = lab.git(&["diff", "--name-only", "--no-renames", main, landed]);
        let files: Vec<String> = files.lines().map(str::to_owned).collect();
        let pr = Pr {
            number: 7,
            head,
            main,
            diff: (main, landed, &files),
            local_objects,
        };
        let r = Carry::new(&repo, pr).resolve(tag, bodies, &|_| true);
        let calls = calls.borrow().clone();
        (r, calls)
    }

    /// m1: the head must be based on the reference `main`, and what lands on `main` must have
    /// the head's tree.
    #[test]
    fn head_on_reference_main_and_landed_tree() {
        let c = case("m1");
        let bodies = [format!("review-ok: {}", c.a)];
        c.lab.git(&["checkout", "-q", "main"]);
        let m2 = c.lab.commit(&[("docs/z.md", Some("z\n"))]);
        let list = commits(&[&c.a, &c.h]);
        let (r, _) = resolve_in(
            &c.lab,
            list.clone(),
            &c.h,
            &m2,
            &c.h,
            false,
            "review-ok",
            &bodies,
        );
        let e = r.unwrap().unwrap_err();
        assert!(e.contains("W3, W4: the head's base"), "{e}");
        // A squash on `m1` whose tree is not the head's.
        let tree = format!("{}^{{tree}}", c.m1);
        let x = c
            .lab
            .git(&["commit-tree", &tree, "-p", &c.m1, "-m", "squash"]);
        let (r, _) = resolve_in(
            &c.lab,
            list.clone(),
            &c.h,
            &c.m1,
            &x,
            false,
            "review-ok",
            &bodies,
        );
        let e = r.unwrap().unwrap_err();
        assert!(e.contains(&format!("W4: the tree of {x}")), "{e}");
        let tree = format!("{}^{{tree}}", c.h);
        let x = c
            .lab
            .git(&["commit-tree", &tree, "-p", &c.m1, "-m", "squash"]);
        let (r, _) = resolve_in(&c.lab, list, &c.h, &c.m1, &x, false, "review-ok", &bodies);
        assert_eq!(r, Ok(Ok(c.a.clone())));
    }

    /// m2: with local objects (merge), the trees of `A` and `H` are read with `git ls-tree`,
    /// never through the API; the result is the same.
    #[test]
    fn local_trees_for_merge() {
        let c = case("m2");
        for tag in ["review-ok", "archsec-ok"] {
            let bodies = [format!("{tag}: {}", c.a)];
            let list = commits(&[&c.a, &c.h]);
            let (r, calls) = resolve_in(&c.lab, list, &c.h, &c.m1, &c.h, true, tag, &bodies);
            assert_eq!(r, Ok(Ok(c.a.clone())), "{tag}");
            assert!(
                !calls.iter().any(|l| l.contains("/git/trees/")),
                "{calls:#?}"
            );
        }
    }

    /// m4: a change of mode alone is a change of the PR file (W2).
    #[test]
    fn w2_mode_change_alone() {
        let c = case("mode");
        c.lab.git(&["update-index", "--chmod=+x", PR_FILE]);
        c.lab.git(&["commit", "-q", "-m", "x"]);
        let h = c.lab.git(&["rev-parse", "HEAD"]);
        let e = resolve(&c, &h, &[&c.a, &c.h, &h], "review-ok", &c.a).unwrap_err();
        assert!(
            e.contains(&format!("W2: {PR_FILE} has another blob or mode")),
            "{e}"
        );
    }

    /// m4, W5: `sensitive-paths.txt` absent or unreadable at the base of `A`: `archsec-ok` is
    /// not carried over, `review-ok` is.
    #[test]
    fn w5_list_absent_or_unreadable_at_base_of_a() {
        for (name, old) in [
            ("w5-absent", None),
            ("w5-bad", Some("tools/xtask/\nnope:x\n")),
        ] {
            let lab = Lab::new(name);
            let list = "tools/ci/sensitive-paths.txt";
            let mut first = vec![(PR_FILE, Some("a0\n"))];
            first.extend(old.map(|t| (list, Some(t))));
            lab.commit(&first);
            lab.git(&["checkout", "-q", "-b", "pr"]);
            let a = lab.commit(&[(PR_FILE, Some("a1\n"))]);
            lab.git(&["checkout", "-q", "main"]);
            let m1 = lab.commit(&[(list, Some("tools/xtask/\n"))]);
            lab.git(&["checkout", "-q", "pr"]);
            lab.git(&["merge", "-q", "--no-edit", &m1]);
            let h = lab.git(&["rev-parse", "HEAD"]);
            for (tag, carried) in [("review-ok", true), ("archsec-ok", false)] {
                let bodies = [format!("{tag}: {a}")];
                let (r, _) = resolve_in(&lab, commits(&[&a, &h]), &h, &m1, &h, false, tag, &bodies);
                match r.unwrap() {
                    Ok(from) => assert!(carried && from == a, "{name} {tag}"),
                    Err(e) => assert!(!carried && e.contains("W5:"), "{name} {tag}: {e}"),
                }
            }
        }
    }

    /// m4, W3: a base of `A` that is not an ancestor of the head's base (answer forged here,
    /// since GitHub computes merge bases on `main`'s linear history).
    #[test]
    fn w3_base_of_a_not_an_ancestor() {
        let c = case("w3-anc");
        c.lab.git(&["checkout", "-q", "-b", "side", &c.c0]);
        let side = c.lab.commit(&[("side.md", Some("s\n"))]);
        let mut answers = commits(&[&c.a, &c.h]);
        answers.push((
            format!("gh api repos/o/r/compare/{}...{}", c.m1, c.a),
            serde_json::json!({"merge_base_commit": {"sha": side}}).to_string(),
        ));
        let bodies = [format!("review-ok: {}", c.a)];
        let (r, _) = resolve_in(
            &c.lab,
            answers,
            &c.h,
            &c.m1,
            &c.h,
            false,
            "review-ok",
            &bodies,
        );
        let e = r.unwrap().unwrap_err();
        assert!(
            e.contains(&format!("W3: its base {side} is not an ancestor")),
            "{e}"
        );
    }

    /// m4: a GitHub read that fails fails its candidate only, the next one can still carry; an
    /// unreadable list of the PR's commits is an error.
    #[test]
    fn failed_github_read() {
        let c = case("read");
        let h2 = c.lab.commit(&[]);
        let mut answers = commits(&[&c.a, &c.h, &h2]);
        answers.push((
            format!("gh api repos/o/r/compare/{}...{h2}", c.h),
            super::lab::FAIL.to_owned(),
        ));
        let one = [format!("review-ok: {}", c.h)];
        let (r, _) = resolve_in(
            &c.lab,
            answers.clone(),
            &h2,
            &c.m1,
            &h2,
            false,
            "review-ok",
            &one,
        );
        let e = r.unwrap().unwrap_err();
        assert!(
            e.contains("unreadable, not carried over")
                && !e.contains("on GitHub")
                && e.contains("HTTP 404"),
            "{e}"
        );
        let two = [format!("review-ok: {}", c.a), format!("review-ok: {}", c.h)];
        let (r, _) = resolve_in(&c.lab, answers, &h2, &c.m1, &h2, false, "review-ok", &two);
        assert_eq!(r, Ok(Ok(c.a.clone())));
        let failed = vec![(
            "gh api --paginate repos/o/r/pulls/7/commits".to_owned(),
            super::lab::FAIL.to_owned(),
        )];
        let (r, _) = resolve_in(&c.lab, failed, &h2, &c.m1, &h2, false, "review-ok", &two);
        assert!(r.is_err(), "{r:?}");
    }

    /// A failed local `git` read (objects of the PR not fetched) is not labelled as a GitHub one
    /// (#63); the Git error stays attached.
    #[test]
    fn failed_local_read_is_not_labelled_github() {
        let c = case("local-read");
        let f = "f".repeat(40);
        let mut answers = commits(&[&f, &c.h]);
        let compare = |a: &str, b: &str, base: &str| {
            (
                format!("gh api repos/o/r/compare/{a}...{b}"),
                json!({"merge_base_commit": {"sha": base}}).to_string(),
            )
        };
        answers.push(compare(&f, &c.h, &f));
        answers.push(compare(&c.m1, &f, &c.c0));
        let (repo, _) = c.lab.repo(answers);
        let pr = Pr {
            number: 7,
            head: &c.h,
            main: &c.m1,
            diff: (&c.m1, &c.h, &[]),
            local_objects: true,
        };
        let bodies = [format!("review-ok: {f}")];
        let e = Carry::new(&repo, pr)
            .resolve("review-ok", &bodies, &|_| true)
            .unwrap()
            .unwrap_err();
        assert!(
            e.contains("unreadable, not carried over") && e.contains("git ls-tree"),
            "{e}"
        );
        assert!(!e.contains("on GitHub"), "{e}");
    }

    /// Eight lines, so that two changes three lines apart merge cleanly.
    const LINES: &str = "l1\nl2\nl3\nl4\nl5\nl6\nl7\nl8\n";

    /// [`LINES`] with line `n` (1-based) replaced by `with`.
    fn edit(n: usize, with: &str) -> String {
        LINES
            .lines()
            .enumerate()
            .map(|(i, l)| format!("{}\n", if i + 1 == n { with } else { l }))
            .collect()
    }

    /// AS-55 lab: `c0` on main; `a` on the PR changes line 2 of [`PR_FILE`]; `m1` on main changes
    /// its line 5 (inside the diff's context); `h` merges `m1` into the PR without conflict.
    fn overlap(name: &str) -> Case {
        let lab = Lab::new(name);
        let c0 = lab.commit(&[
            ("tools/ci/sensitive-paths.txt", Some("tools/xtask/\n")),
            (PR_FILE, Some(LINES)),
            ("other.md", Some("o0\n")),
        ]);
        lab.git(&["checkout", "-q", "-b", "pr"]);
        let a = lab.commit(&[(PR_FILE, Some(&edit(2, "L2")))]);
        lab.git(&["checkout", "-q", "main"]);
        let m1 = lab.commit(&[(PR_FILE, Some(&edit(5, "L5")))]);
        lab.git(&["checkout", "-q", "pr"]);
        lab.git(&["merge", "-q", "--no-edit", &m1]);
        let h = lab.git(&["rev-parse", "HEAD"]);
        Case { lab, c0, a, m1, h }
    }

    /// `resolve` of `review-ok` approved on `c.a`, for `head` against `main` = `c.m1`, the PR's
    /// commits being `list`, with the PR's objects local (`merge`).
    fn resolve55(c: &Case, head: &str, list: &[&str]) -> Result<String, String> {
        let bodies = [format!("review-ok: {}", c.a)];
        let (r, _) = resolve_in(
            &c.lab,
            commits(list),
            head,
            &c.m1,
            head,
            true,
            "review-ok",
            &bodies,
        );
        r.unwrap()
    }

    /// AS-55: `main` changed the PR's file, the merge is clean and the PR's lines are the same
    /// (context aside): both approvals are carried over, with the AS-55 message. Without the PR's
    /// objects flag (`verify-main`), the local objects are used and nothing is fetched.
    #[test]
    fn as55_clean_merge_identical_diff_carries_over() {
        let c = overlap("as55-ok");
        for (tag, local) in [
            ("review-ok", true),
            ("archsec-ok", true),
            ("archsec-ok", false),
        ] {
            let bodies = [format!("{tag}: {}", c.a)];
            let (repo, calls) = c.lab.repo(commits(&[&c.a, &c.h]));
            let files = [PR_FILE.to_owned()];
            let pr = Pr {
                number: 7,
                head: &c.h,
                main: &c.m1,
                diff: (&c.m1, &c.h, &files),
                local_objects: local,
            };
            let mut carry = Carry::new(&repo, pr);
            assert_eq!(carry.resolve(tag, &bodies, &|_| true), Ok(Ok(c.a.clone())));
            assert!(carry.by_as55(&c.a));
            assert_eq!(
                carry.describe(&c.a),
                format!(
                    "carried over from {} to {} (AS-55: clean merges of main, identical diff)",
                    c.a, c.h
                )
            );
            assert!(
                !calls.borrow().iter().any(|l| l.contains("fetch")),
                "{:?}",
                calls.borrow()
            );
        }
    }

    /// X2: a conflict on the PR's line, resolved by hand, even back to the PR's version.
    #[test]
    fn as55_conflict_resolved_by_hand() {
        let c = overlap("as55-conflict");
        c.lab.git(&["checkout", "-q", "main"]);
        let m2 = c.lab.commit(&[(PR_FILE, Some(&edit(2, "M2")))]);
        c.lab.git(&["checkout", "-q", "-b", "resolved", &c.a]);
        let (code, _, _) = super::lab::git_in(&c.lab.dir, &["merge", "-q", "--no-edit", &m2]);
        assert_ne!(code, Some(0), "the merge must conflict");
        let h = c.lab.commit(&[(PR_FILE, Some(&edit(2, "L2")))]);
        let bodies = [format!("review-ok: {}", c.a)];
        let (r, _) = resolve_in(
            &c.lab,
            commits(&[&c.a, &h]),
            &h,
            &m2,
            &h,
            true,
            "review-ok",
            &bodies,
        );
        let e = r.unwrap().unwrap_err();
        assert!(
            e.contains("W3: ") && e.contains("AS-55: X2: merging"),
            "{e}"
        );
    }

    /// X2: an « evil » merge, clean but with a line added to the PR's file in the merge commit;
    /// a change of white space only in the merge commit is caught the same way.
    #[test]
    fn as55_evil_merge() {
        let c = overlap("as55-evil");
        for (name, text) in [
            (
                "evil",
                format!("{}evil\n", edit(5, "L5").replace("l2", "L2")),
            ),
            ("blank", edit(5, "L5").replace("l2", "L2 ")),
        ] {
            c.lab.git(&["checkout", "-q", "-b", name, &c.a]);
            c.lab.git(&["merge", "-q", "--no-commit", &c.m1]);
            let h = c.lab.commit(&[(PR_FILE, Some(&text))]);
            let e = resolve55(&c, &h, &[&c.a, &h]).unwrap_err();
            assert!(
                e.contains("W2: ") && e.contains("X2: the tree of") && e.contains("edited merge"),
                "{name}: {e}"
            );
        }
    }

    /// X3 is byte for byte: a change of white space only in the PR's lines differs, where the
    /// context, the `@@` and `index` lines may differ. `git patch-id` would see no change.
    #[test]
    fn as55_x3_white_space_counts() {
        let c = overlap("as55-blank");
        c.lab.git(&["checkout", "-q", "-b", "blank", &c.c0]);
        let blank = c.lab.commit(&[(PR_FILE, Some(&edit(2, "L2 ")))]);
        let (repo, _) = c.lab.repo(Vec::new());
        let at_a = pr_diff(&repo, &c.c0, &c.a).unwrap();
        assert_eq!(
            first_difference(&at_a, &pr_diff(&repo, &c.m1, &c.h).unwrap()),
            None
        );
        let at = first_difference(&pr_diff(&repo, &c.c0, &blank).unwrap(), &at_a);
        let header = format!("diff --git a/{PR_FILE} b/{PR_FILE}");
        assert_eq!(at.as_deref(), Some(header.as_str()));
        let patch_id = |x: &str| {
            let (_, diff, _) = super::lab::git_in(&c.lab.dir, &["diff", &c.c0, x]);
            let out = super::lab::git_input(&c.lab.dir, &["patch-id", "--stable"], &diff);
            let out = String::from_utf8(out).unwrap();
            out.split(' ').next().unwrap_or_default().to_owned()
        };
        assert!(!patch_id(&blank).is_empty());
        assert_eq!(
            patch_id(&blank),
            patch_id(&c.a),
            "patch-id ignores white space"
        );
    }

    /// X3: the PR made a file executable; `main` did the same, then the merge is clean but the
    /// PR's diff loses its mode lines.
    #[test]
    fn as55_mode_change() {
        let c = overlap("as55-mode");
        c.lab.git(&["checkout", "-q", "-b", "modes", &c.c0]);
        crate::sys::write(&c.lab.dir.join(PR_FILE), edit(2, "L2").as_bytes()).unwrap();
        c.lab.git(&["add", PR_FILE]);
        c.lab.git(&["update-index", "--chmod=+x", PR_FILE]);
        c.lab.git(&["commit", "-q", "-m", "x"]);
        let a = c.lab.git(&["rev-parse", "HEAD"]);
        // The executable bit lives in the index only: the work tree's is ignored.
        let no_mode = ["-c", "core.fileMode=false"];
        c.lab
            .git(&[&no_mode[..], &["checkout", "-q", "main"]].concat());
        c.lab.git(&["update-index", "--chmod=+x", PR_FILE]);
        c.lab.git(&["commit", "-q", "-m", "x"]);
        let m2 = c.lab.git(&["rev-parse", "HEAD"]);
        c.lab
            .git(&[&no_mode[..], &["checkout", "-q", "modes"]].concat());
        c.lab
            .git(&[&no_mode[..], &["merge", "-q", "--no-edit", &m2]].concat());
        let h = c.lab.git(&["rev-parse", "HEAD"]);
        let bodies = [format!("review-ok: {a}")];
        let (r, _) = resolve_in(
            &c.lab,
            commits(&[&a, &h]),
            &h,
            &m2,
            &h,
            true,
            "review-ok",
            &bodies,
        );
        let e = r.unwrap().unwrap_err();
        assert!(e.contains("W2: ") && e.contains("AS-55: X3: "), "{e}");
    }

    /// X1: an ordinary commit slipped between `A` and the merge, or after it; a merge whose second
    /// parent is not on `main`.
    #[test]
    fn as55_ordinary_commit_or_foreign_merge() {
        let c = overlap("as55-x1");
        c.lab.git(&["checkout", "-q", "-b", "slipped", &c.a]);
        let o = c.lab.commit(&[]);
        c.lab.git(&["merge", "-q", "--no-edit", &c.m1]);
        let h = c.lab.git(&["rev-parse", "HEAD"]);
        let e = resolve55(&c, &h, &[&c.a, &o, &h]).unwrap_err();
        assert!(e.contains(&format!("X1: {o} is not a merge")), "{e}");
        c.lab.git(&["checkout", "-q", "pr"]);
        let after = c.lab.commit(&[]);
        let e = resolve55(&c, &after, &[&c.a, &c.h, &after]).unwrap_err();
        assert!(e.contains(&format!("X1: {after} is not a merge")), "{e}");
        // `side` has the tree of `m1`, but is not a `main` commit.
        let tree = format!("{}^{{tree}}", c.m1);
        let side = c
            .lab
            .git(&["commit-tree", &tree, "-p", &c.c0, "-m", "side"]);
        c.lab.git(&["checkout", "-q", "-b", "foreign", &c.h]);
        c.lab.git(&["merge", "-q", "--no-edit", &side]);
        let h = c.lab.git(&["rev-parse", "HEAD"]);
        let e = resolve55(&c, &h, &[&c.a, &c.h, &h]).unwrap_err();
        assert!(e.contains(&format!("X1: the second parent {side}")), "{e}");
    }

    /// Two approved SHAs before a merge of `main`, and a head absent from the local objects (as in
    /// CI, `verify-main`): the answers of `gh` for the PR, and those that make the head absent.
    fn absent_head(
        c: &Case,
        head_repo: serde_json::Value,
    ) -> (String, String, Vec<(String, String)>) {
        c.lab.git(&["checkout", "-q", "-B", "two", &c.a]);
        let a2 = c.lab.commit(&[]);
        c.lab.git(&["merge", "-q", "--no-edit", &c.m1]);
        let h2 = c.lab.git(&["rev-parse", "HEAD"]);
        let mut answers = commits(&[&c.a, &a2, &h2]);
        answers.push((
            format!("git cat-file -e {h2}^{{commit}}"),
            super::lab::FAIL.to_owned(),
        ));
        answers.push((
            "gh api repos/o/r/pulls/7".to_owned(),
            json!({"number": 7, "head": {"sha": h2, "repo": head_repo}}).to_string(),
        ));
        (a2, h2, answers)
    }

    /// X4: a PR from a fork, or whose head repository is missing, is refused without any fetch;
    /// the API is read once for both candidates.
    #[test]
    fn as55_never_fetches_a_fork() {
        let c = overlap("as55-fork");
        for (repo, why) in [
            (
                json!({"full_name": "x/r"}),
                "X4: PR #7 comes from the fork x/r",
            ),
            (
                json!(null),
                "X4: the head repository of PR #7 is unreadable",
            ),
            (json!({}), "X4: the head repository of PR #7 is unreadable"),
        ] {
            let (a2, h2, answers) = absent_head(&c, repo);
            let bodies = [format!("review-ok: {a2}"), format!("review-ok: {}", c.a)];
            let (r, calls) = resolve_in(
                &c.lab,
                answers,
                &h2,
                &c.m1,
                &h2,
                false,
                "review-ok",
                &bodies,
            );
            let e = r.unwrap().unwrap_err();
            assert_eq!(e.matches(why).count(), 2, "{e}");
            assert!(!calls.iter().any(|l| l.contains("fetch")), "{calls:#?}");
            let reads = calls.iter().filter(|l| *l == "gh api repos/o/r/pulls/7");
            assert_eq!(reads.count(), 1, "{calls:#?}");
        }
    }

    /// `verify-main` without the head among the local objects: `pull/<n>/head` of a PR of this
    /// repository is fetched once, whatever the number of candidates, and a failed fetch refuses
    /// them. `merge`, whose objects were fetched, never fetches again.
    #[test]
    fn as55_fetches_the_head_once() {
        let c = overlap("as55-fetch");
        for local in [false, true] {
            let (a2, h2, answers) = absent_head(&c, json!({"full_name": "o/r"}));
            let bodies = [format!("review-ok: {a2}"), format!("review-ok: {}", c.a)];
            let (r, calls) = resolve_in(
                &c.lab,
                answers,
                &h2,
                &c.m1,
                &h2,
                local,
                "review-ok",
                &bodies,
            );
            let e = r.unwrap().unwrap_err();
            let fetches: Vec<_> = calls.iter().filter(|l| l.contains("fetch")).collect();
            if local {
                assert!(fetches.is_empty(), "{fetches:?}");
            } else {
                assert_eq!(
                    fetches,
                    ["git fetch --quiet --no-tags --no-recurse-submodules origin pull/7/head"],
                    "{calls:#?}"
                );
            }
            assert_eq!(
                e.matches("unreadable, not carried over (W2: ").count(),
                2,
                "{e}"
            );
            let absent = e.matches("absent from the local objects").count();
            assert_eq!(absent, if local { 2 } else { 1 }, "{e}");
        }
    }

    /// A failed local read on the AS-55 path refuses the candidate, W2 kept in the reason.
    #[test]
    fn as55_failed_read_refuses() {
        let c = overlap("as55-read");
        let mut answers = commits(&[&c.a, &c.h]);
        answers.push(("git rev-list".to_owned(), super::lab::FAIL.to_owned()));
        let bodies = [format!("review-ok: {}", c.a)];
        let (r, _) = resolve_in(
            &c.lab,
            answers,
            &c.h,
            &c.m1,
            &c.h,
            true,
            "review-ok",
            &bodies,
        );
        let e = r.unwrap().unwrap_err();
        assert!(
            e.contains("unreadable, not carried over (W2: ") && e.contains("AS-55: "),
            "{e}"
        );
    }

    /// Most recent approval first; a repeated SHA is looked at once; a quoted line does not count.
    #[test]
    fn candidates_most_recent_first() {
        let (a, b) = ("a".repeat(40), "b".repeat(40));
        let bodies = [
            format!("review-ok: {a}"),
            format!("review-ok: {b}  "),
            format!("review-ok: {a}"),
            format!("Not yet.\nreview-ok: {}", "c".repeat(40)),
            format!("archsec-ok: {}", "d".repeat(40)),
            "review-ok: HEAD".to_owned(),
        ];
        assert_eq!(approved_shas(&bodies, "review-ok"), [a, b]);
        assert!(approved_shas(&bodies[..1], "archsec-ok").is_empty());
    }
}
