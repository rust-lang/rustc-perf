/// Target representing an Rust target triple, for a full list of targets and
/// their support see;
/// https://doc.rust-lang.org/nightly/rustc/platform-support.html
///
/// Presently we only support x86_64
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize)]
pub enum Target {
    /// `x86_64-unknown-linux-gnu`
    X86_64UnknownLinuxGnu,
}

impl Default for Target {
    fn default() -> Self {
        Self::X86_64UnknownLinuxGnu
    }
}

impl Target {
    pub fn all() -> Vec<Self> {
        vec![Self::X86_64UnknownLinuxGnu]
    }

    pub fn from_db_target(target: &database::Target) -> Target {
        match target {
            database::Target::X86_64UnknownLinuxGnu => Self::X86_64UnknownLinuxGnu,
        }
    }
}
