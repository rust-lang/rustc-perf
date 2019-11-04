//! Write benchmark information to the output repository

use anyhow::{anyhow, Context};
use collector::{ArtifactData, Commit, CommitData};
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
    fn git(&self, args: &[&str]) -> anyhow::Result<()> {
        for iteration in 0..5 {
            let mut command = Command::new("git");
            command.current_dir(&self.path);
            info!("[{}/5]: git {:?}", iteration, args);
            command.args(args);
            let mut child = command
                .spawn()
                .with_context(|| format!("could not spawn git {:?}", args))?;
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
                            return Err(anyhow!(
                                "command `git {:?}` failed in `{}`",
                                args,
                                self.path.display()
                            ));
                        }
                    }
                    Ok(None) => {
                        debug!("waiting 250ms...");
                        thread::sleep(time::Duration::from_millis(250));
                    }
                    Err(err) => {
                        return Err(anyhow!(
                            "command `git {:?}` failed to try_wait in {:?}: {:?}",
                            args,
                            self.path.display(),
                            err
                        ))
                    }
                }
            }
        }
        Err(anyhow!(
            "failed to run git command, timed out too many times"
        ))
    }

    pub fn open(path: PathBuf, allow_new_dir: bool, use_remote: bool) -> anyhow::Result<Self> {
        let result = Repo {
            path: path,
            use_remote,
        };

        // Don't nuke random repositories, unless specifically requested.
        if !allow_new_dir && !result.perf_file().exists() {
            return Err(anyhow!(
                "`{}` file not present, refusing to run",
                result.perf_file().display()
            ));
        }

        if result.use_remote && result.path.join(".git").exists() {
            result.git(&["fetch"])?;
            result.git(&["reset", "--hard", "@{upstream}"])?;
        }

        fs::create_dir_all(result.times())?;

        Ok(result)
    }

    pub fn success(&self, data: &CommitData) -> anyhow::Result<()> {
        self.add_commit_data(data)?;
        self.commit_and_push(&format!("{} - success", data.commit.sha))?;
        Ok(())
    }

    pub fn success_artifact(&self, data: &ArtifactData) -> anyhow::Result<()> {
        let filepath = self.times().join(format!("artifact-{}.json", data.id));
        info!("creating file {}", filepath.display());
        let serialized = serde_json::to_string(&data)?;
        fs::write(&filepath, &serialized)?;
        self.commit_and_push(&format!("{} - success", data.id))?;
        Ok(())
    }

    fn commit_and_push(&self, message: &str) -> anyhow::Result<()> {
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

    fn load_commit_data_file(&self, filepath: &Path) -> anyhow::Result<CommitData> {
        trace!("loading file {}", filepath.display());
        let contents =
            fs::read(&filepath).with_context(|| format!("failed to read {:?}", filepath))?;
        let c;
        let contents = if filepath.to_str().map_or(false, |s| s.ends_with(".sz")) {
            use std::io::Read;
            let mut out = Vec::with_capacity(snap::decompress_len(&contents).unwrap_or(0));
            let mut szip_reader = snap::Reader::new(&contents[..]);
            szip_reader.read_to_end(&mut out).unwrap();
            c = out;
            &c
        } else {
            &contents
        };
        let data = serde_json::from_slice(&contents)
            .with_context(|| format!("failed to read JSON from {:?}", filepath))?;
        Ok(data)
    }

    pub fn load_commit_data(&self, commit: &Commit, triple: &str) -> anyhow::Result<CommitData> {
        let filepath = self.times().join(format!(
            "{}-{}-{}.json",
            commit.date.0.to_rfc3339(),
            commit.sha,
            triple
        ));
        if let Ok(v) = self.load_commit_data_file(&filepath) {
            return Ok(v);
        }
        let filepath = self
            .times()
            .join(format!("commit-{}-{}.json", commit.sha, triple));
        if let Ok(v) = self.load_commit_data_file(&filepath) {
            return Ok(v);
        }
        let filepath = self
            .times()
            .join(format!("commit-{}-{}.json.sz", commit.sha, triple));
        Ok(self.load_commit_data_file(&filepath)?)
    }

    pub fn add_commit_data(&self, data: &CommitData) -> anyhow::Result<()> {
        let commit = &data.commit;
        let filepath = self
            .times()
            .join(format!("commit-{}-{}.json.sz", commit.sha, data.triple));
        info!("creating file {}", filepath.display());
        let mut v = snap::Writer::new(Vec::new());
        serde_json::to_writer(&mut v, &data)?;
        fs::write(&filepath, v.into_inner()?)?;
        Ok(())
    }

    fn perf_file(&self) -> PathBuf {
        self.path.join(".is-perf-timing-directory")
    }

    fn times(&self) -> PathBuf {
        self.path.join("times")
    }
}
