//! Write benchmark information to the output repository

use collector::{ArtifactData, Commit, CommitData};
use failure::{Error, ResultExt};
use log::{debug, info, trace, warn};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use std::thread;
use std::time::{self, Instant};

pub struct Repo {
    path: PathBuf,
    use_remote: bool,
}

impl Repo {
    fn git(&self, args: &[&str]) -> Result<(), Error> {
        for iteration in 0..5 {
            let mut command = Command::new("git");
            command.current_dir(&self.path);
            info!("[{}/5]: git {:?}", iteration, args);
            command.args(args);
            let mut child = command
                .spawn()
                .with_context(|_| format!("could not spawn git {:?}", args))?;
            let start_time = Instant::now();
            loop {
                if start_time.elapsed().as_secs() > 30 {
                    // network operations may take up to 30sec
                    warn!("killing git command -- timed out");
                    child.kill()?;
                    break;
                }
                match child.try_wait() {
                    Ok(Some(status)) => {
                        if status.success() {
                            return Ok(());
                        } else {
                            bail!(
                                "command `git {:?}` failed in `{}`",
                                args,
                                self.path.display()
                            );
                        }
                    }
                    Ok(None) => {
                        debug!("waiting 250ms...");
                        thread::sleep(time::Duration::from_millis(250));
                    }
                    Err(err) => bail!(
                        "command `git {:?}` failed to try_wait in {:?}: {:?}",
                        args,
                        self.path.display(),
                        err
                    ),
                }
            }
        }
        bail!("failed to run git command, timed out too many times")
    }

    pub fn open(path: PathBuf, allow_new_dir: bool, use_remote: bool) -> Result<Self, Error> {
        let result = Repo {
            path: path,
            use_remote,
        };

        // Don't nuke random repositories, unless specifically requested.
        if !allow_new_dir && !result.perf_file().exists() {
            bail!(
                "`{}` file not present, refusing to run",
                result.perf_file().display()
            );
        }

        if result.use_remote && result.path.join(".git").exists() {
            result.git(&["fetch"])?;
            result.git(&["reset", "--hard", "@{upstream}"])?;
        }

        fs::create_dir_all(result.times())?;

        Ok(result)
    }

    pub fn success(&self, data: &CommitData) -> Result<(), Error> {
        self.add_commit_data(data)?;
        self.commit_and_push(&format!("{} - success", data.commit.sha))?;
        Ok(())
    }

    pub fn success_artifact(&self, data: &ArtifactData) -> Result<(), Error> {
        let filepath = self.times().join(format!("artifact-{}.bincode", data.id));
        info!("creating file {}", filepath.display());
        let serialized = bincode::serialize(&data)?;
        fs::write(&filepath, &serialized)?;
        self.commit_and_push(&format!("{} - success", data.id))?;
        Ok(())
    }

    fn commit_and_push(&self, message: &str) -> Result<(), Error> {
        self.git(&["add", "times"])?;

        // dirty index
        if let Err(_) = self.git(&["diff-index", "--quiet", "--cached", "HEAD"]) {
            self.git(&["commit", "-m", message])?;
            if self.use_remote {
                self.git(&["push"])?;
            }
        } else {
            println!("nothing to commit...");
        }
        Ok(())
    }

    fn load_commit_data_file(&self, filepath: &Path) -> Result<CommitData, Error> {
        trace!("loading file {}", filepath.display());
        let contents =
            fs::read(&filepath).with_context(|_| format!("failed to read {:?}", filepath))?;
        let data = bincode::deserialize(&contents)
            .with_context(|_| format!("failed to read bincode from {:?}", filepath))?;
        Ok(data)
    }

    pub fn load_commit_data(&self, commit: &Commit, triple: &str) -> Result<CommitData, Error> {
        let filepath = self.times().join(format!(
            "{}-{}-{}.bincode",
            commit.date.0.to_rfc3339(),
            commit.sha,
            triple
        ));
        match self.load_commit_data_file(&filepath) {
            Ok(v) => return Ok(v),
            Err(_) => self.load_commit_data_file(
                &self
                    .times()
                    .join(format!("commit-{}-{}.bincode", commit.sha, triple)),
            ),
        }
    }

    pub fn add_commit_data(&self, data: &CommitData) -> Result<(), Error> {
        let commit = &data.commit;
        let filepath = self
            .times()
            .join(format!("commit-{}-{}.bincode", commit.sha, data.triple));
        info!("creating file {}", filepath.display());
        let serialized = bincode::serialize(&data)?;
        fs::write(&filepath, &serialized)?;
        Ok(())
    }

    fn perf_file(&self) -> PathBuf {
        self.path.join(".is-perf-timing-directory")
    }

    fn times(&self) -> PathBuf {
        self.path.join("times")
    }
}
