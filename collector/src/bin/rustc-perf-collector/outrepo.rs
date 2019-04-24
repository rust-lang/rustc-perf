//! Write benchmark information to the output repository

use collector::{ArtifactData, Commit, CommitData};
use failure::{Error, ResultExt};
use log::{debug, info, trace, warn};
use serde_json;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use std::thread;
use std::time::{self, Instant};

pub struct Repo {
    path: PathBuf,
    use_remote: bool,
    retries: Vec<String>,
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
                if start_time.elapsed().as_secs() > 30 { // network operations may take up to 30sec
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
        let mut result = Repo {
            path: path,
            use_remote,
            retries: vec![],
        };

        // Don't nuke random repositories, unless specifically requested.
        if !allow_new_dir && !result.retries_file().exists() {
            bail!("`{}` file not present", result.retries_file().display());
        }

        if result.use_remote {
            result.git(&["fetch"])?;
            result.git(&["reset", "--hard", "@{upstream}"])?;
        }

        fs::create_dir_all(result.times()).context("can't create `times/`")?;
        result.load_retries()?;

        Ok(result)
    }

    pub fn success(&self, data: &CommitData) -> Result<(), Error> {
        self.add_commit_data(data)?;
        self.commit_and_push(&format!("{} - success", data.commit.sha))?;
        Ok(())
    }

    pub fn success_artifact(&self, data: &ArtifactData) -> Result<(), Error> {
        let filepath = self.times().join(format!("artifact-{}.json", data.id));
        info!("creating file {}", filepath.display());
        let mut file = File::create(&filepath)?;
        serde_json::to_writer(&mut file, &data)?;
        self.commit_and_push(&format!("{} - success", data.id))?;
        Ok(())
    }

    fn commit_and_push(&self, message: &str) -> Result<(), Error> {
        self.write_retries()?;
        self.git(&["add", "retries", "times"])?;

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
        let mut file = File::open(&filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data = serde_json::from_str(&contents)
            .with_context(|_| format!("failed to read JSON from {:?}", filepath))?;
        Ok(data)
    }

    pub fn load_commit_data(&self, commit: &Commit, triple: &str) -> Result<CommitData, Error> {
        let filepath = self.times().join(format!(
            "{}-{}-{}.json",
            commit.date.0.to_rfc3339(),
            commit.sha,
            triple
        ));
        match self.load_commit_data_file(&filepath) {
            Ok(v) => return Ok(v),
            Err(_) => self.load_commit_data_file(
                &self
                    .times()
                    .join(format!("commit-{}-{}.json", commit.sha, triple)),
            ),
        }
    }

    pub fn add_commit_data(&self, data: &CommitData) -> Result<(), Error> {
        let commit = &data.commit;
        let filepath = self
            .times()
            .join(format!("commit-{}-{}.json", commit.sha, data.triple));
        info!("creating file {}", filepath.display());
        let mut file = File::create(&filepath)?;
        serde_json::to_writer(&mut file, &data)?;
        Ok(())
    }

    fn load_retries(&mut self) -> Result<(), Error> {
        let mut retries = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.retries_file())
            .with_context(|_| format!("can't create `{}`", self.retries_file().display()))?;
        let mut retries_s = String::new();
        retries.read_to_string(&mut retries_s)?;
        self.retries = retries_s
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                if line.len() == 40 {
                    Ok(line.to_owned())
                } else {
                    bail!("bad retry hash `{}`", line)
                }
            })
            .collect::<Result<_, _>>()?;
        info!("loaded retries: {:?}", self.retries);
        Ok(())
    }

    fn write_retries(&self) -> Result<(), Error> {
        info!("writing retries");
        let mut retries = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.retries_file())
            .context("can't create `retries`")?;
        for retry in self.retries.iter() {
            writeln!(retries, "{}", retry)?;
        }
        Ok(())
    }

    pub fn write_broken_commit(&self, commit: &Commit, err: Error) -> Result<(), Error> {
        info!("writing broken commit {:?}: {:?}", commit, err);
        let mut broken = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.broken_commits_file())
            .context("can't create `broken-commits-log`")?;
        writeln!(broken, "{}: \"{:?}\"", commit.sha, err)?;
        Ok(())
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
