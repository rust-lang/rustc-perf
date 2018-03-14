// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Read;
use std::env;

use serde_json;

use errors::*;
use util;
use git;
use collector::Date;

pub use collector::{Commit, CommitData, Patch, Run, Stat};

#[derive(Debug)]
pub struct InputData {
    /// A set containing all crate names of the bootstrap kind.
    pub crate_list: BTreeSet<String>,

    /// All known statistics gathered for crates
    pub stats_list: BTreeSet<String>,

    /// The last date that was seen while loading files. The DateTime variant is
    /// used here since the date may or may not contain a time. Since the
    /// timezone is not important, it isn't stored, hence the Naive variant.
    pub last_date: Date,

    pub data: BTreeMap<Commit, CommitData>,
}

impl InputData {
    /// Initialize `InputData from the file system.
    pub fn from_fs(repo_loc: &str) -> Result<InputData> {
        let repo_loc = PathBuf::from(repo_loc);
        let mut skipped = 0;
        let mut data = BTreeMap::new();

        if !repo_loc.exists() {
            // If the repository doesn't yet exist, simplify clone it to the given location.
            info!(
                "cloning repository into {}, since it doesn't exist before",
                repo_loc.display()
            );
            git::execute_command(
                &env::current_dir()?,
                &[
                    "clone",
                    "https://github.com/rust-lang-nursery/rustc-timing.git",
                    repo_loc.to_str().unwrap(),
                ],
            )?;
        }

        // Read all files from repo_loc/processed
        let mut file_count = 0;
        for entry in fs::read_dir(repo_loc.join("times"))? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                continue;
            }
            file_count += 1;

            let filename = entry.file_name();
            let filename = filename.to_str().unwrap();
            let mut file = File::open(entry.path())?;
            let mut file_contents = String::new();
            // Skip files whose size is 0.
            if file.read_to_string(&mut file_contents)? == 0 {
                warn!("Skipping empty file: {}", filename);
                skipped += 1;
                continue;
            }

            let contents: CommitData = match serde_json::from_str(&file_contents) {
                Ok(json) => json,
                Err(err) => {
                    error!("Failed to parse JSON for {}: {:?}", filename, err);
                    skipped += 1;
                    continue;
                }
            };
            if contents.benchmarks.is_empty() {
                warn!("empty benchmarks hash for {}", filename);
                skipped += 1;
                continue;
            }

            data.insert(contents.commit.clone(), contents);
        }

        info!("{} total files", file_count);
        info!("{} skipped files", skipped);
        info!("{} measured", data.len());

        InputData::new(data)
    }

    pub fn new(data: BTreeMap<Commit, CommitData>) -> Result<InputData> {
        let mut last_date = None;
        let mut crate_list = BTreeSet::new();
        let mut stats_list = BTreeSet::new();

        for commit_data in data.values() {
            if last_date.is_none() || last_date.as_ref().unwrap() < &commit_data.commit.date {
                last_date = Some(commit_data.commit.date);
            }

            let benchmarks = commit_data
                .benchmarks
                .values()
                .filter_map(|v| v.as_ref().ok());
            for benchmark in benchmarks {
                for run in &benchmark.runs {
                    let run_name = benchmark.name.clone() + &run.name();
                    crate_list.insert(run_name);
                    for stat in &run.stats {
                        stats_list.insert(stat.name.clone());
                    }
                }
            }
        }

        let last_date = last_date.expect("No dates found");

        Ok(InputData {
            crate_list: crate_list,
            stats_list: stats_list,
            last_date: last_date,
            data: data,
        })
    }
}

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "util::round_float")] pub f64);
