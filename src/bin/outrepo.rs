//! Write benchmark information to the output repository

use errors::{Error, Result, ResultExt};

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{PathBuf};
use std::process::Command;
use std::str;
use std::time;
use std::thread;

use serde_json;
use rustc_perf_collector::{Commit, CommitData};

pub struct Repo {
    path: PathBuf
}

impl Repo {
    fn git(&self) -> Command {
        let mut command = Command::new("git");
        command.current_dir(&self.path);
        command
    }

    fn git_nooutput(&self, args: &[&str]) -> Result<()> {
        let mut command = self.git();
        info!("git {:?}", args);
        for arg in args {
            command.arg(arg);
        }
        let output = command.output().chain_err(|| "could not spawn `git`")?;
        if !output.status.success() {
            bail!("command `git {:?}` failed at `{}`:\n{}",
                  args,
                  self.path.display(),
                  String::from_utf8_lossy(&output.stderr));
        }
        Ok(())

    }

    fn git_location(&self) -> Result<()> {
        let current = self
            .git().arg("log").arg("--oneline").arg("-1").arg("HEAD")
            .output().chain_err(|| "could not spawn `git`")?;
        if !current.status.success() {
            bail!("`{}` is not a valid repository:\n{}",
                  self.path.display(),
                  String::from_utf8_lossy(&current.stderr));
        }

        info!("at commit {}", String::from_utf8_lossy(&current.stdout).trim());
        Ok(())
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        let result = Repo { path: path };

        result.git_location()?;
        if !result.times().is_dir() {
            bail!("`{}` directory not present", result.times().display());
        }

        result.git_nooutput(&["pull", "-f"])?;
        result.git_nooutput(&["reset", "--hard", "HEAD"])?;
        result.git_location()?;

        Ok(result)
    }

    pub fn success(&self, triple: &str, data: &CommitData) -> Result<()> {
        self.add_commit_data(triple, data)?;
        self.set_last_commit(&data.commit)?;
        self.commit_and_push(&format!("{} - success", data.commit.sha))?;
        Ok(())
    }

    pub fn failure(&self, commit: &Commit, err: &Error) -> Result<()> {
        self.add_broken_commit(commit, err)?;
        self.set_last_commit(commit)?;
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
        self.git_nooutput(&[
            "add",
            "last-commit-sha",
            "broken-commits-log",
            "times"])?;
        self.git_nooutput(&[
            "commit",
            "-m",
            message
        ])?;
        self.git_nooutput(&["push"])?;
        Ok(())
    }


    fn add_commit_data(&self, triple: &str, data: &CommitData) -> Result<()> {
        let commit = &data.commit;
        let filepath = self.times()
            .join(format!("{}-{}-{}.json", commit.date, commit.sha, triple));
        info!("creating file {}", filepath.display());
        let mut file = File::create(&filepath)?;
        serde_json::to_writer(&mut file, &data)?;
        Ok(())
    }

    fn set_last_commit(&self, last_commit: &Commit) -> Result<()> {
        let mut commit_file = File::create(self.commit_file())?;
        commit_file.write_all(last_commit.sha.as_bytes())?;
        Ok(())
    }

    fn add_broken_commit(&self, commit: &Commit, err: &Error) -> Result<()> {
        // FIXME: make file machine-readable?
        let mut file = OpenOptions::new().append(true).create(true).open(
            self.broken_commits_file())?;
        writeln!(file, "{}: {:?}", commit.sha, err)?;
        Ok(())
    }

    fn commit_file(&self) -> PathBuf {
        self.path.join("last-commit-sha")
    }

    fn broken_commits_file(&self) -> PathBuf {
        self.path.join("broken-commits-log")
    }

    fn times(&self) -> PathBuf {
        self.path.join("times")
    }
}
