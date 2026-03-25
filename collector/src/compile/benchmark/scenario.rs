#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ValueEnum, serde::Deserialize)]
#[value(rename_all = "PascalCase")]
pub enum Scenario {
    Full,
    IncrFull,
    IncrUnchanged,
    IncrPatched,
    ParallelFrontend,
}

impl Scenario {
    pub fn all() -> Vec<Scenario> {
        vec![
            Scenario::Full,
            Scenario::IncrFull,
            Scenario::IncrUnchanged,
            Scenario::IncrPatched,
            Scenario::ParallelFrontend,
        ]
    }

    pub fn all_non_incr() -> Vec<Scenario> {
        vec![Scenario::Full, Scenario::ParallelFrontend]
    }

    pub fn is_incr(self) -> bool {
        matches!(
            self,
            Scenario::IncrFull | Scenario::IncrUnchanged | Scenario::IncrPatched
        )
    }
}
