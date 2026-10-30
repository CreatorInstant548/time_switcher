// spec: 05-architecture.md §2.3 ; securite.md S8 ; attendus-mesurables.md M28
//! Profil de construction (SEC0-20) : fonctionnalités de développement, refus de `public` avec
//! l'une d'elles, et constante de profil.
//!
//! Chemin sensible (`tools/ci/sensitive-paths.txt`). Les messages des `compile_error!` sont
//! cherchés mot pour mot par l'étape `profiles` de `cargo xtask check` : ne pas les modifier.

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

/// Profil de construction : `"public"` si et seulement si la fonctionnalité `public` est active,
/// sinon `"dev"` (E9). Le refus de démarrer en ligne compare à `"public"` : il reste fermé par
/// défaut.
pub const BUILD_PROFILE: &str = if cfg!(feature = "public") {
    "public"
} else {
    "dev"
};

/// Fonctionnalités de développement, toutes interdites dans le profil public.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DevFeature {
    /// `dev-commands` : commandes d'écriture `Spawn` et `SetComponent`, seules derrière cette
    /// fonctionnalité. `AdvanceTime` n'en fait pas partie : c'est une commande normale du solo
    /// (S8), que la matrice par source refuse aux sessions joueur en ligne (AS-33).
    Commands,
    /// `dev-introspection` : inspection de l'état hors de la vue filtrée.
    Introspection,
    /// `external-pilot` : pilote `Externe` des outils.
    ExternalPilot,
    /// `dev-bench` : banc de pilotage et pause de décision des sessions joueuses.
    Bench,
}

impl DevFeature {
    /// Toutes les fonctionnalités de développement, dans l'ordre de déclaration.
    pub const ALL: [Self; 4] = [
        Self::Commands,
        Self::Introspection,
        Self::ExternalPilot,
        Self::Bench,
    ];

    /// Nom de la fonctionnalité Cargo.
    pub const fn cargo_name(self) -> &'static str {
        match self {
            Self::Commands => "dev-commands",
            Self::Introspection => "dev-introspection",
            Self::ExternalPilot => "external-pilot",
            Self::Bench => "dev-bench",
        }
    }

    /// Vrai si la fonctionnalité est compilée dans ce binaire.
    pub const fn is_enabled(self) -> bool {
        match self {
            Self::Commands => cfg!(feature = "dev-commands"),
            Self::Introspection => cfg!(feature = "dev-introspection"),
            Self::ExternalPilot => cfg!(feature = "external-pilot"),
            Self::Bench => cfg!(feature = "dev-bench"),
        }
    }
}

/// Fonctionnalités de développement actives, pour le refus de démarrer et la vérification de
/// l'artefact (SEC0-20).
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
