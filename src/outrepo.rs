//! Write benchmark information to the output repository

use errors::{Error, Result, ResultExt};

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{PathBuf};
use std::process::Command;
use std::str;
use std::time;
use std::thread;

use serde_json;
use rustc_perf_collector::{Commit, CommitData};

pub struct Repo {
    path: PathBuf,
    retries: Vec<String>
}

pub enum CommitKind {
    Progress,
    Retry
}

impl Repo {
    fn git(&self, args: &[&str]) -> Result<()> {
        let mut command = Command::new("git");
        command.current_dir(&self.path);
        info!("git {:?}", args);
        command.args(args);
        let status = command.status().chain_err(|| format!("could not spawn git {:?}", args))?;
        if !status.success() {
            bail!("command `git {:?}` failed in `{}`", args, self.path.display());
        }
        Ok(())
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        let mut result = Repo { path: path, retries: vec![] };

        if !result.commit_file().exists() {
            // try not to nuke random repositories.
            bail!("`{:?}` file not present", result.commit_file().display());
        }

        result.git(&["fetch"])?;
        result.git(&["reset", "--hard", "@{upstream}"])?;

        fs::create_dir_all(result.times()).chain_err(|| "can't create `times/`")?;
        OpenOptions::new().append(true).create(true).open(result.broken_commits_file())
            .chain_err(|| "can't create `broken_commits`")?;
        result.load_retries()?;

        Ok(result)
    }

    pub fn success(&self, data: &CommitData, kind: CommitKind) -> Result<()> {
        self.add_commit_data(data)?;
        self.set_last_commit(&data.commit, kind)?;
        self.commit_and_push(&format!("{} - success", data.commit.sha))?;
        Ok(())
    }

    pub fn failure(&self, commit: &Commit, err: &Error, kind: CommitKind) -> Result<()> {
        self.add_broken_commit(commit, err)?;
        self.set_last_commit(commit, kind)?;
        self.commit_and_push(&format!("{} - FAILURE", commit.sha))?;

        // sleep for a minute to avoid triggering rate-limits.
        info!("timing commit failed - sleeping");
        thread::sleep(time::Duration::from_secs(60));
        Ok(())
    }

    pub fn get_last_commit(&self) -> Result<String> {
        let mut commit_file = File::open(self.commit_file())
            .chain_err(|| format!("expected {} to exist, and contain the last tested commit sha",
                                  self.commit_file().display()))?;

        let mut last_commit = String::new();
        commit_file.read_to_string(&mut last_commit)?;
        let last_commit = last_commit.trim();

        if last_commit.is_empty() {
            bail!("{} was empty", self.commit_file().display());
        }

        Ok(last_commit.to_owned())
    }

    fn commit_and_push(&self, message: &str) -> Result<()> {
        self.write_retries()?;
        self.git(&["add", "last-commit-sha", "broken-commits-log", "retries", "times"])?;
        self.git(&["commit", "-m", message])?;
        self.git(&["push"])?;
        Ok(())
    }


    fn add_commit_data(&self, data: &CommitData) -> Result<()> {
        let commit = &data.commit;
        let filepath = self.times()
            .join(format!("{}-{}-{}.json", commit.date.to_rfc3339(), commit.sha, data.triple));
        info!("creating file {}", filepath.display());
        let mut file = File::create(&filepath)?;
        serde_json::to_writer(&mut file, &data)?;
        Ok(())
    }

    fn set_last_commit(&self, last_commit: &Commit, kind: CommitKind) -> Result<()> {
        if let CommitKind::Progress = kind {
            let mut commit_file = File::create(self.commit_file())?;
            commit_file.write_all(last_commit.sha.as_bytes())?;
        }
        Ok(())
    }

    fn add_broken_commit(&self, commit: &Commit, err: &Error) -> Result<()> {
        // FIXME: make file machine-readable?
        let mut file = OpenOptions::new().append(true).create(true).open(
            self.broken_commits_file())?;
        writeln!(file, "{}: {:?}", commit.sha, err)?;
        Ok(())
    }

    fn load_retries(&mut self) -> Result<()> {
        let mut retries = OpenOptions::new().read(true).write(true).create(true).open(self.retries_file())
            .chain_err(|| format!("can't create `{}`", self.retries_file().display()))?;
        let mut retries_s = String::new();
        retries.read_to_string(&mut retries_s)?;
        self.retries = retries_s.split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                if line.len() == 40 {
                    Ok(line.to_owned())
                } else {
                    bail!("bad retry hash `{}`", line)
                }
            }).collect::<Result<_>>()?;
        info!("loaded retries: {:?}", self.retries);
        Ok(())
    }

    fn write_retries(&self) -> Result<()> {
        info!("writing retries");
        let mut retries = OpenOptions::new().write(true).truncate(true)
            .open(self.retries_file())
            .chain_err(|| "can't create `retries`")?;
        for retry in self.retries.iter() {
            writeln!(retries, "{}", retry)?;
        }
        Ok(())
    }

    pub fn next_retry(&mut self) -> Option<String> {
        if self.retries.len() == 0 {
            None
        } else {
            Some(self.retries.remove(0))
        }
    }

    fn commit_file(&self) -> PathBuf {
        self.path.join("last-commit-sha")
    }

    fn broken_commits_file(&self) -> PathBuf {
        self.path.join("broken-commits-log")
    }

    fn retries_file(&self) -> PathBuf {
        self.path.join("retries")
    }

    fn times(&self) -> PathBuf {
        self.path.join("times")
    }
}
