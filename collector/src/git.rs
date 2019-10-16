use crate::{command_output, run_command, Commit};
use std::{
    env,
    path::PathBuf,
    process::Command,
    time::{Duration, SystemTime},
};

pub fn get_commit_or_fake_it(sha: &str) -> anyhow::Result<Commit> {
    Ok(get_rust_commits()?
        .iter()
        .find(|c| c.sha == sha)
        .cloned()
        .unwrap_or_else(|| {
            log::warn!("utilizing fake commit!");
            Commit {
                sha: sha.to_string(),
                date: crate::Date::ymd_hms(2000, 01, 01, 0, 0, 0),
            }
        }))
}

/// This retrieves the list of Rust commits which may be available in the CI S3 bucket.
pub fn get_rust_commits() -> anyhow::Result<Vec<Commit>> {
    let repo_dir =
        PathBuf::from(env::var("RUST_SRC_REPO").unwrap_or_else(|_| String::from("rust.git")));

    if !repo_dir.exists() {
        run_command(
            Command::new("git")
                .arg("clone")
                .arg("--bare")
                .arg("https://github.com/rust-lang/rust.git")
                .arg(&repo_dir),
        )?;
    } else {
        run_command(
            Command::new("git")
                .current_dir(&repo_dir)
                .arg("fetch")
                .arg("origin")
                .arg("master:master"),
        )?;
    }

    let minute = Duration::from_secs(60);
    let hour = minute * 60;
    let day = hour * 24;

    // As of the time of this writing, we store 168 days of commit artifacts.
    let start = SystemTime::now() - (day * 168);
    let start = start.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let out = command_output(
        Command::new("git")
            .current_dir(&repo_dir)
            .arg("log")
            .arg("--reverse")
            .arg("--author=bors")
            .arg(&format!("--max-age={}", start.as_secs()))
            .arg("master")
            .arg("--format=%H %ct"),
    )?;
    let mut commits = Vec::new();
    for line in std::str::from_utf8(&out.stdout)?.lines() {
        let mut iter = line.split(' ');
        let hash = iter.next().unwrap();
        let date = iter.next().unwrap();
        let date = date
            .parse::<u64>()
            .unwrap_or_else(|e| panic!("{:?}: {:?}", date, e)); // unix timestamp
        let date = chrono::NaiveDateTime::from_timestamp(date as i64, 0);
        let date = crate::Date(chrono::DateTime::from_utc(date, chrono::Utc));

        commits.push(Commit {
            sha: hash.to_string(),
            date,
        });
    }
    Ok(commits)
}
