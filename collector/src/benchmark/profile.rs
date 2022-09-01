#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ArgEnum, serde::Deserialize)]
#[clap(rename_all = "PascalCase")]
pub enum Profile {
    Check,
    Debug,
    Doc,
    Opt,
}

impl Profile {
    pub fn all() -> Vec<Self> {
        vec![Profile::Check, Profile::Debug, Profile::Doc, Profile::Opt]
    }

    pub fn all_non_doc() -> Vec<Self> {
        vec![Profile::Check, Profile::Debug, Profile::Opt]
    }
}
