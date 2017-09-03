// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{BTreeSet, BTreeMap};
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Read;
use std::env;

use chrono::Duration;
use serde_json;

use errors::*;
use util;
use git;
use date::Date;

const WEEKS_IN_SUMMARY: usize = 12;

pub use rustc_perf_collector::{Pass, Stat, Run, Patch, Commit, CommitData};

#[derive(Debug)]
pub struct InputData {
    pub summary: Summary,

    /// A set containing all crate names of the bootstrap kind.
    pub crate_list: BTreeSet<String>,

    /// A set containing all phase names, across all crates.
    pub phase_list: BTreeSet<String>,

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
            info!("cloning repository into {}, since it doesn't exist before", repo_loc.display());
            git::execute_command(&env::current_dir()?, &[
                "clone", "https://github.com/rust-lang-nursery/rustc-timing.git",
                repo_loc.to_str().unwrap()
            ])?;
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
        let mut phase_list = BTreeSet::new();
        let mut crate_list = BTreeSet::new();
        let mut stats_list = BTreeSet::new();

        for run in data.values() {
            if last_date.is_none() || last_date.as_ref().unwrap() < &run.commit.date {
                last_date = Some(run.commit.date);
            }

            for patch in run.benchmarks.values()
                .filter(|v| v.is_ok()).flat_map(|v| v.as_ref().unwrap()) {
                crate_list.insert(patch.full_name());
                for pass in &patch.run().passes {
                    phase_list.insert(pass.name.clone());
                }

                for stat in &patch.run().stats {
                    stats_list.insert(stat.name.clone());
                }
            }
        }

        let last_date = last_date.expect("No dates found");

        // Post processing to generate the summary data.
        let summary = Summary::new(&data, last_date);

        Ok(InputData {
            summary: summary,
            crate_list: crate_list,
            phase_list: phase_list,
            stats_list: stats_list,
            last_date: last_date,
            data: data,
        })
    }
}

#[derive(Debug)]
pub struct Comparison {
    pub a: Commit,
    pub b: Commit,

    /// Maps crate names to a map of phases to each phase's delta time over the range.
    pub by_crate: BTreeMap<String, ByCrateComparison>,
}

#[derive(Debug, Default)]
pub struct ByCrateComparison {
    pub passes: BTreeMap<String, f64>,
    pub stats: BTreeMap<String, f64>,
}

#[derive(Debug)]
pub struct Summary {
    pub total: Comparison,
    pub comparisons: Vec<Comparison>,
}

impl Summary {
    // Compute summary data. For each week, we find the last 3 weeks, and use
    // the median timing as the basis of the current week's summary.
    fn new(
        data: &BTreeMap<Commit, CommitData>,
        last_date: Date,
    ) -> Summary {
        // 12 week long mapping of crate names to by-phase percent changes with
        // the previous week.
        let mut weeks = Vec::with_capacity(WEEKS_IN_SUMMARY);

        for i in 0..WEEKS_IN_SUMMARY {
            debug!("summarizing week {}", i);
            let start = last_date.start_of_week() - Duration::weeks(i as i64);
            let end = start + Duration::weeks(1);
            debug!("start: {:?}, end: {:?}", start, end);

            let mut week = util::data_range(data, start, end);
            let first = week.clone().next();
            let last = week.next_back();

            if let (Some((_, first)), Some((_, last))) = (first, last) {
                debug!("actual: start: {:?}, end: {:?}", first.commit.date, last.commit.date);
                weeks.push(Summary::compare_points(first, last));
            } else {
                warn!("week {} - {} has too few commits", start, end);
            }
        }

        let totals = {
            let start = last_date.start_of_week() - Duration::weeks(13);
            let end = last_date + Duration::weeks(1);

            let mut week = util::data_range(data, start, end);
            Summary::compare_points(week.clone().next().expect("first commit exists").1,
                week.next_back().expect("last commit exists").1)
        };

        Summary {
            total: totals,
            comparisons: weeks,
        }
    }

    fn compare_points(a: &CommitData, b: &CommitData) -> Comparison {
        let mut by_crate = BTreeMap::new();
        for a_benchmark in a.benchmarks() {
            if a_benchmark.is_none() { continue; }
            let (crate_name, patches) = a_benchmark.unwrap();
            if !b.benchmarks.contains_key(crate_name) {
                warn!("Comparing {} with {}: a contained {}, but b did not.",
                    a.commit.sha, b.commit.sha, crate_name);
                continue;
            }

            let a_patches = patches;
            let b_patches = b.benchmarks[crate_name].as_ref().map(|v| &v[..]).ok();
            if b_patches.is_none() { continue; }
            let b_patches = b_patches.unwrap();

            for a_patch in a_patches {
                let a_run = a_patch.run();
                let b_patch = b_patches.iter().find(|p| p.run().name == a_run.name);
                if b_patch.is_none() { continue; }
                let b_run = b_patch.unwrap().run();

                for a_pass in &a_run.passes {
                    let a_t = a_pass.time;
                    let b_t = b_run.get_pass(&a_pass.name).map(|p| p.time).unwrap_or(0.0);
                    println!("{}: {}: {} - {}: {}", a_run.name, a_pass.name, b_t, a_t, b_t - a_t);
                    by_crate.entry(a_run.name.clone())
                        .or_insert_with(ByCrateComparison::default)
                        .passes
                        .insert(a_pass.name.clone(), b_t - a_t);
                }

                for a_stat in &a_run.stats {
                    let a_t = a_stat.cnt;
                    let b_t = b_run.get_stat(&a_stat.name).unwrap_or(0.0);
                    println!("{}: {}: {} - {}: {}", a_run.name, a_stat.name, b_t, a_t, b_t - a_t);
                    by_crate.entry(a_run.name.clone())
                        .or_insert_with(ByCrateComparison::default)
                        .stats
                        .insert(a_stat.name.clone(), b_t - a_t);
                }
            }
        }
        Comparison {
            a: a.commit.clone(),
            b: b.commit.clone(),
            by_crate: by_crate,
        }
    }
}

/// One decimal place rounded percent
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Percent(#[serde(with = "util::round_float")] pub f64);
