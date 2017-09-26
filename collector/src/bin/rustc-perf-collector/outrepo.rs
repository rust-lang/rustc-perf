//! Write benchmark information to the output repository

use errors::{Result, ResultExt};

use std::fs::{self, read_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str;
use std::collections::HashSet;

use serde_json;
use collector::CommitData;
use rust_sysroot::git::Commit as GitCommit;
use execute::Benchmark;

pub struct Repo {
    path: PathBuf,
    use_remote: bool,
    retries: Vec<String>,
}

impl Repo {
    fn git(&self, args: &[&str]) -> Result<()> {
        let mut command = Command::new("git");
        command.current_dir(&self.path);
        info!("git {:?}", args);
        command.args(args);
        let status = command
            .status()
            .chain_err(|| format!("could not spawn git {:?}", args))?;
        if !status.success() {
            bail!(
                "command `git {:?}` failed in `{}`",
                args,
                self.path.display()
            );
        }
        Ok(())
    }

    pub fn open(path: PathBuf, use_remote: bool) -> Result<Self> {
        let mut result = Repo {
            path: path,
            use_remote,
            retries: vec![],
        };

        if !result.retries_file().exists() {
            // try not to nuke random repositories.
            bail!("`{}` file not present", result.retries_file().display());
        }

        if result.use_remote {
            result.git(&["fetch"])?;
            result.git(&["reset", "--hard", "@{upstream}"])?;
        }

        fs::create_dir_all(result.times()).chain_err(|| "can't create `times/`")?;
        result.load_retries()?;

        Ok(result)
    }

    pub fn success(&self, data: &CommitData) -> Result<()> {
        self.add_commit_data(data)?;
        self.commit_and_push(&format!("{} - success", data.commit.sha))?;
        Ok(())
    }

    pub fn find_missing_commits<'a>(
        &self,
        commits: &'a [GitCommit],
        benchmarks: &[Benchmark],
        triple: &str,
    ) -> Result<Vec<&'a GitCommit>> {
        let mut have = HashSet::new();
        let path = self.times();
        for entry in read_dir(path)? {
            let entry = entry?;
            let filename = entry.file_name().to_string_lossy().to_string();
            let sha =
                &filename[filename.find("00:00").unwrap() + 6..filename.find("-x86").unwrap()];
            have.insert(sha.to_string());
        }

        if let Ok(file) = File::open(self.broken_commits_file()) {
            let file = BufReader::new(file);
            for line in file.lines() {
                let line = line?;
                let sha = &line[..line.find(":").unwrap()];
                have.insert(sha.to_string());
            }
        }

        let missing = commits
            .iter()
            .filter(|c| {
                !have.contains(&c.sha) || {
                    self.load_commit_data(c, triple)
                        .ok()
                        .map(|data| {
                            benchmarks
                                .iter()
                                .any(|b| data.benchmarks.keys().find(|k| **k == b.name).is_none())
                        })
                        .unwrap_or(true)
                }
            })
            .collect::<Vec<_>>();

        Ok(missing)
    }

    fn commit_and_push(&self, message: &str) -> Result<()> {
        self.write_retries()?;
        self.git(&["add", "retries", "times"])?;
        self.git(&["commit", "-m", message])?;
        if self.use_remote {
            self.git(&["push"])?;
        }
        Ok(())
    }

    pub fn load_commit_data(&self, commit: &GitCommit, triple: &str) -> Result<CommitData> {
        let filepath = self.times().join(format!(
            "{}-{}-{}.json",
            commit.date.to_rfc3339(),
            commit.sha,
            triple
        ));
        info!("loading file {}", filepath.display());
        let mut file = File::open(&filepath)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let data = serde_json::from_str(&contents)?;
        Ok(data)
    }

    pub fn add_commit_data(&self, data: &CommitData) -> Result<()> {
        let commit = &data.commit;
        let filepath = self.times().join(format!(
            "{}-{}-{}.json",
            commit.date,
            commit.sha,
            data.triple
        ));
        info!("creating file {}", filepath.display());
        let mut file = File::create(&filepath)?;
        serde_json::to_writer(&mut file, &data)?;
        Ok(())
    }

    fn load_retries(&mut self) -> Result<()> {
        let mut retries = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.retries_file())
            .chain_err(|| {
                format!("can't create `{}`", self.retries_file().display())
            })?;
        let mut retries_s = String::new();
        retries.read_to_string(&mut retries_s)?;
        self.retries = retries_s
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| if line.len() == 40 {
                Ok(line.to_owned())
            } else {
                bail!("bad retry hash `{}`", line)
            })
            .collect::<Result<_>>()?;
        info!("loaded retries: {:?}", self.retries);
        Ok(())
    }

    fn write_retries(&self) -> Result<()> {
        info!("writing retries");
        let mut retries = OpenOptions::new()
            .write(true)
            .truncate(true)
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

    fn broken_commits_file(&self) -> PathBuf {
        self.path.join("broken-commits-log")
    }

    fn retries_file(&self) -> PathBuf {
        self.path.join("retries")
    }

    pub fn times(&self) -> PathBuf {
        self.path.join("times")
    }
}
