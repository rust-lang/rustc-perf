//! Write benchmark information to the output repository

use crate::old::{ArtifactData, CommitData};
use database::Commit;
use std::path::{PathBuf};


pub struct Repo {}

#[allow(unused)]
impl Repo {
    pub fn open(path: PathBuf, allow_new_dir: bool, use_remote: bool) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn success(&self, data: &CommitData) -> anyhow::Result<()> {
        todo!()
    }

    pub fn success_artifact(&self, data: &ArtifactData) -> anyhow::Result<()> {
        todo!()
    }

    pub fn load_commit_data(&self, commit: &Commit, triple: &str) -> anyhow::Result<CommitData> {
        todo!()
    }

    pub fn add_commit_data(&self, data: &CommitData) -> anyhow::Result<()> {
        todo!()
    }
}
