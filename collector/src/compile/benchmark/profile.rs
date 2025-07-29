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
    /// Perform the equivalent of `cargo check`.
    Check,
    /// Perform the equivalent of `cargo build`.
    Debug,
    /// Perform the equivalent of `cargo doc`.
    Doc,
    /// Perform the equivalent of `cargo doc` with `--output-format=json`.
    DocJson,
    /// Perform the equivalent of `cargo build --release`.
    Opt,
    /// Perform the equivalent of `cargo clippy`.
    Clippy,
}

impl Profile {
    pub fn all() -> Vec<Self> {
        vec![
            Profile::Check,
            Profile::Debug,
            Profile::Doc,
            Profile::DocJson,
            Profile::Opt,
            Profile::Clippy,
        ]
    }

    pub fn is_doc(&self) -> bool {
        match self {
            Profile::Doc | Profile::DocJson => true,
            Profile::Check | Profile::Debug | Profile::Opt | Profile::Clippy => false,
        }
    }
}

impl From<Profile> for database::Profile {
    fn from(value: Profile) -> Self {
        match value {
            Profile::Check => database::Profile::Check,
            Profile::Debug => database::Profile::Debug,
            Profile::Doc => database::Profile::Doc,
            Profile::DocJson => database::Profile::DocJson,
            Profile::Opt => database::Profile::Opt,
            Profile::Clippy => database::Profile::Clippy,
        }
    }
}
