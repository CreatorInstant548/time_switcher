// spec: 05-architecture.md §2.3 ; securite.md S8 ; attendus-mesurables.md M28
//! Build profile (SEC0-20): development features, refusal of `public` together with any of
//! them, and the profile constant.
//!
//! Sensitive path (`tools/ci/sensitive-paths.txt`). The `compile_error!` messages are searched
//! for word for word by the `profiles` step of `cargo xtask check`: do not change them. They
//! therefore stay in French on purpose, until that step is updated.

#[cfg(all(feature = "public", feature = "dev-commands"))]
compile_error!("profil public : la fonctionnalité `dev-commands` est interdite (SEC0-20, M28)");

#[cfg(all(feature = "public", feature = "dev-introspection"))]
compile_error!(
    "profil public : la fonctionnalité `dev-introspection` est interdite (SEC0-20, M28)"
);

#[cfg(all(feature = "public", feature = "external-pilot"))]
compile_error!("profil public : la fonctionnalité `external-pilot` est interdite (SEC0-20, M28)");

#[cfg(all(feature = "public", feature = "dev-bench"))]
compile_error!("profil public : la fonctionnalité `dev-bench` est interdite (SEC0-20, M28)");

/// Build profile: `"public"` if and only if the `public` feature is active, otherwise `"dev"`
/// (E9). The refusal to start online compares against `"public"`: it stays closed by
/// default.
pub const BUILD_PROFILE: &str = if cfg!(feature = "public") {
    "public"
} else {
    "dev"
};

/// Development features, all forbidden in the public profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DevFeature {
    /// `dev-commands`: the `Spawn` and `SetComponent` write commands, only behind this
    /// feature. `AdvanceTime` is not one of them: it is a normal solo command (S8), which the
    /// per-source matrix refuses to online player sessions (AS-33).
    Commands,
    /// `dev-introspection`: inspection of the state outside the filtered view.
    Introspection,
    /// `external-pilot`: the `External` pilot of the tools.
    ExternalPilot,
    /// `dev-bench`: the driving bench and the decision pause of player sessions.
    Bench,
}

impl DevFeature {
    /// All development features, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Commands,
        Self::Introspection,
        Self::ExternalPilot,
        Self::Bench,
    ];

    /// Name of the Cargo feature.
    pub const fn cargo_name(self) -> &'static str {
        match self {
            Self::Commands => "dev-commands",
            Self::Introspection => "dev-introspection",
            Self::ExternalPilot => "external-pilot",
            Self::Bench => "dev-bench",
        }
    }

    /// True if the feature is compiled into this binary.
    pub const fn is_enabled(self) -> bool {
        match self {
            Self::Commands => cfg!(feature = "dev-commands"),
            Self::Introspection => cfg!(feature = "dev-introspection"),
            Self::ExternalPilot => cfg!(feature = "external-pilot"),
            Self::Bench => cfg!(feature = "dev-bench"),
        }
    }
}

/// Development features that are active, for the refusal to start and the check of the
/// artifact (SEC0-20).
pub fn active_dev_features() -> impl Iterator<Item = DevFeature> {
    DevFeature::ALL.into_iter().filter(|f| f.is_enabled())
}

// spec: 05-architecture.md §2.3 ; carnet architecture-securite.md SEC0-20, SEC0-24
/// Items compiled in every profile except `public`: for development code of other crates
/// (for example the fuzzing body of `ts_persist`), which can test this crate's `public`
/// feature only through this macro. Clippy sees the items in the default pass.
///
/// ```
/// ts_kernel::not_public! {
///     pub fn dev_only() {}
/// }
/// ```
#[cfg(not(feature = "public"))]
#[macro_export]
macro_rules! not_public {
    ($($item:item)*) => { $($item)* };
}

/// Items compiled in every profile except `public` (here, `public`: nothing).
#[cfg(feature = "public")]
#[macro_export]
macro_rules! not_public {
    ($($item:item)*) => {};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_profile_is_public_iff_public_feature() {
        assert_eq!(BUILD_PROFILE == "public", cfg!(feature = "public"));
        assert_eq!(BUILD_PROFILE == "dev", !cfg!(feature = "public"));
        if cfg!(feature = "public") {
            assert_eq!(active_dev_features().count(), 0);
        }
    }

    #[test]
    fn dev_features_match_cargo_features() {
        let names: Vec<&str> = active_dev_features().map(DevFeature::cargo_name).collect();
        let expected: Vec<&str> = [
            ("dev-commands", cfg!(feature = "dev-commands")),
            ("dev-introspection", cfg!(feature = "dev-introspection")),
            ("external-pilot", cfg!(feature = "external-pilot")),
            ("dev-bench", cfg!(feature = "dev-bench")),
        ]
        .into_iter()
        .filter_map(|(name, on)| on.then_some(name))
        .collect();
        assert_eq!(names, expected);
    }
}
