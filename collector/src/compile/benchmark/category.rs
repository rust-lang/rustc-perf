use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "PascalCase")]
pub enum Category {
    Primary,
    Secondary,
    Stable,
}

impl Category {
    pub fn is_stable(self) -> bool {
        self == Category::Stable
    }

    pub fn is_primary_or_secondary(self) -> bool {
        self == Category::Primary || self == Category::Secondary
    }

    // Within the DB, `Category` is represented in two fields:
    // - a `supports_stable` bool,
    // - a `category` which is either "primary" or "secondary".
    pub fn db_representation(self) -> (bool, String) {
        match self {
            Category::Primary => (false, "primary".to_string()),
            Category::Secondary => (false, "secondary".to_string()),
            Category::Stable => (true, "primary".to_string()),
        }
    }

    pub fn from_db_representation(text: &str) -> anyhow::Result<Self> {
        match text {
            "primary" => Ok(Self::Primary),
            "secondary" => Ok(Self::Secondary),
            _ => Err(anyhow::anyhow!("Unknown category {text}")),
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Primary => f.write_str("primary"),
            Category::Secondary => f.write_str("secondary"),
            Category::Stable => f.write_str("stable"),
        }
    }
}
