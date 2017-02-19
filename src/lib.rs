#[macro_use] extern crate serde_derive;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pass {
    pub name: String,
    pub time: f64,
    pub mem: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    pub name: String,
    pub passes: Vec<Pass>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Patch {
    pub patch: String,
    pub name: String,
    pub commit: Commit,
    pub runs: Vec<Run>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
    pub sha: String,
    pub date: String,
}
