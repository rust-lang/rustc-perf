use database::{intern_target_name, TargetName};
use std::{fmt, str::FromStr};

/// Target representing an Rust target triple, for a full list of targets and
/// their support see;
/// https://doc.rust-lang.org/nightly/rustc/platform-support.html
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Deserialize)]
pub enum Target {
    /// `x86_64-unknown-linux-gnu`
    X86_64UnknownLinuxGnu,

    /// `aarch64-unknown-linux-gnu`
    AArch64UnknownLinuxGnu,
    /// Custom target
    Custom(TargetName),
}

impl FromStr for Target {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x86_64-unknown-linux-gnu" => Target::X86_64UnknownLinuxGnu,
            "aarch64-unknown-linux-gnu" => Target::AArch64UnknownLinuxGnu,
            name => Target::Custom(intern_target_name(name)),
        })
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Target {
    pub fn as_str(self) -> &'static str {
        match self {
            Target::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Target::AArch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Target::Custom(name) => name.as_str(),
        }
    }

    #[cfg(target_arch = "x86_64")]
    pub fn host() -> Self {
        Self::X86_64UnknownLinuxGnu
    }

    #[cfg(target_arch = "aarch64")]
    pub fn host() -> Self {
        Self::AArch64UnknownLinuxGnu
    }

    #[cfg(target_arch = "loongarch64")]
    pub fn host() -> Self {
        Target::Custom(intern_target_name("loongarch64-unknown-linux-gnu"))
    }
}

impl From<database::Target> for Target {
    fn from(value: database::Target) -> Self {
        match value {
            database::Target::X86_64UnknownLinuxGnu => Self::X86_64UnknownLinuxGnu,
            database::Target::AArch64UnknownLinuxGnu => Self::AArch64UnknownLinuxGnu,
            database::Target::Custom(name) => Self::Custom(name),
        }
    }
}

impl From<Target> for database::Target {
    fn from(value: Target) -> Self {
        match value {
            Target::X86_64UnknownLinuxGnu => database::Target::X86_64UnknownLinuxGnu,
            Target::AArch64UnknownLinuxGnu => database::Target::AArch64UnknownLinuxGnu,
            Target::Custom(name) => database::Target::Custom(name),
        }
    }
}
