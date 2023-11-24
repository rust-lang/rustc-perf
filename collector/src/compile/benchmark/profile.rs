/// Type of compilation used for benchmarking/profiling.
// Note: This type is very similar to the definition of a profile in the database crate.
// However, these types should not be unified, as they serve slightly different purposes.
// This type is used for specifying profiles to be benchmarked using the CLI, which is not relevant
// to the database crate.
// In general, the database versions of types used in the collector should be considered a DB
// implementation detail, as they may change when we alter database layout.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ValueEnum, serde::Deserialize)]
#[value(rename_all = "PascalCase")]
pub enum Profile {
    Check,
    Debug,
    Doc,
    Opt,
    Clippy,
}

impl Profile {
    pub fn all() -> Vec<Self> {
        vec![
            Profile::Check,
            Profile::Debug,
            Profile::Doc,
            Profile::Opt,
            Profile::Clippy,
        ]
    }
}
