use std::{fmt, str::FromStr};

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

impl FromStr for Target {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "x86_64-unknown-linux-gnu" => Target::X86_64UnknownLinuxGnu,
            _ => return Err(format!("{} is not a valid target", s)),
        })
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Target {
    pub fn all() -> Vec<Self> {
        vec![Self::X86_64UnknownLinuxGnu]
    }
}

impl Target {
    pub fn as_str(self) -> &'static str {
        match self {
            Target::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
        }
    }
}

impl From<database::Target> for Target {
    fn from(value: database::Target) -> Self {
        match value {
            database::Target::X86_64UnknownLinuxGnu => Self::X86_64UnknownLinuxGnu,
        }
    }
}

impl From<Target> for database::Target {
    fn from(value: Target) -> Self {
        match value {
            Target::X86_64UnknownLinuxGnu => database::Target::X86_64UnknownLinuxGnu,
        }
    }
}
