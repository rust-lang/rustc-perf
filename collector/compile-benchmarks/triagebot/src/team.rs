use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Team {
    Libs,
    Compiler,
    Lang,
}

impl Team {
    pub fn label(&self) -> crate::github::Label {
        match self {
            Team::Libs => crate::github::Label {
                name: String::from("T-libs"),
            },
            Team::Compiler => crate::github::Label {
                name: String::from("T-compiler"),
            },
            Team::Lang => crate::github::Label {
                name: String::from("T-lang"),
            },
        }
    }
}

impl FromStr for Team {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "libs" => Team::Libs,
            "compiler" => Team::Compiler,
            "lang" => Team::Lang,
            _ => anyhow::bail!("unknown team: {:?}", s),
        })
    }
}
