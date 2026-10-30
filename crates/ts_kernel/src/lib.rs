// spec: 05-architecture.md §2.3 ; securite.md S8 ; attendus-mesurables.md M28
//! Noyau de simulation de time_switcher, sans affichage ni E/S.
//!
//! Squelette de la phase 0 : il ne porte encore que le profil de construction (SEC0-20).

#![forbid(unsafe_code)]

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
    /// `dev-commands` : commandes d'écriture (`SetComponent`, `Spawn`, `AdvanceTime`).
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
