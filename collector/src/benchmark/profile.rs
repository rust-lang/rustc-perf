#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ArgEnum, serde::Deserialize)]
#[clap(rename_all = "PascalCase")]
pub enum Profile {
    Check,
    Debug,
    Doc,
    JsonDoc,
    Opt,
}

impl Profile {
    pub fn all() -> Vec<Self> {
        vec![Profile::Check, Profile::Debug, Profile::Doc, Profile::JsonDoc, Profile::Opt]
    }

    pub fn all_non_doc() -> Vec<Self> {
        vec![Profile::Check, Profile::Debug, Profile::Opt]
    }

    pub fn is_doc(self) -> bool {
        matches!(self, Self::Doc | Self::JsonDoc)
    }
}
