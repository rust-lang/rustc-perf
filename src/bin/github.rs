//! Get commits through the Github API.

use std::str;

use rustc_perf_collector::Commit;

use chrono::{TimeZone, UTC};
use reqwest;
use reqwest::header::Authorization;
use serde_json::{self, Value};

use errors::{Result, ResultExt};

const GH_API_TOKEN: &'static str = env!("GH_API_TOKEN");

fn parse_commit(commit: Value) -> Commit {
    if let Value::Object(mut map) = commit {
        return Commit {
            sha: map.remove("sha").expect("sha to be present").as_str().unwrap().to_string(),
            date: UTC.datetime_from_str(map.remove("commit")
                .and_then(|mut commit| commit.as_object_mut()
                .and_then(|commit| commit.remove("committer")))
                .and_then(|mut committer| committer.as_object_mut()
                .and_then(|committer| committer.remove("date")))
                .expect("commit.comitter.date to be present").as_str().unwrap(),
                "%+").expect("failed to parse date"),
        };
    } else {
        panic!("commit object {:?} is not an object?", commit)
    }
}

fn request_from_gh(client: &reqwest::Client, url: &str) -> Result<(Value, reqwest::Response)> {
    info!("Requesting: {}", url);
    let mut request_ = client.get(url);
    if !GH_API_TOKEN.is_empty() {
        request_ = request_.header(Authorization(format!("token {}", GH_API_TOKEN)));
    }
    let mut response = request_.send().chain_err(|| format!("API request to {}", url))?;
    let value = serde_json::from_reader(&mut response)
        .chain_err(|| format!("API request to {} deserialization", url))?;
    Ok((value, response))
}

pub fn commit_from_sha(client: &reqwest::Client, hash: &str) -> Result<Commit> {
    let url = format!("https://api.github.com/repos/rust-lang/rust/commits/{}", hash);
    let (value, _) = request_from_gh(client, &url)?;
    Ok(parse_commit(value))
}

pub fn get_new_commits(last_commit: &str) -> Result<Vec<Commit>> {
    let client = reqwest::Client::new()?;

    fn request(client: &reqwest::Client, url: &str, last_commit: &str) -> Result<Vec<Commit>> {
        fn convert_to_str_array(url: &str, value: Value) -> Result<Vec<Commit>> {
            let value = if let Value::Array(arr) = value {
                arr.into_iter().map(parse_commit).collect()
            } else {
                bail!("{} returned non-array response: {}", url, value);
            };

            Ok(value)
        }

        let (value, response) = request_from_gh(client, url)?;
        let mut commits = convert_to_str_array(url, value)?;

        if let Some(pos) = commits.iter().position(|commit| commit.sha == last_commit) {
            commits.truncate(pos);
            return Ok(commits);
        }

        if let Some(headers) = response.headers().get_raw("Link") {
            if let Some(next) = headers.iter().map(|s| str::from_utf8(s).unwrap()).flat_map(|s| s.split(", ")).find(|s| s.contains(r#"rel="next"#)) {
                let url = &next[1..next.find(">").unwrap()];
                let next_value = request(&client, url, last_commit)?;
                commits.extend(next_value);
                if let Some(_) = commits.iter().find(|commit| commit.sha == last_commit) {
                    return Ok(commits);
                }

            }
        }

        Ok(commits)
    }

    let commits = request(
        &client,
        "https://api.github.com/repos/rust-lang/rust/commits?author=bors&per_page=100",
        &last_commit,
    )?;

    Ok(commits)
}
