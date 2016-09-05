// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::max;
use std::collections::{HashMap, HashSet};

use iron::prelude::*;
use router::Router;
use persistent::State;
use serde_json;

use git;
use route_handler;
use date::Date;
use util::get_repo_path;
use api::{summary, info, data, tabular, days, stats};
use load::{SummarizedWeek, Kind, TestRun, Timing, InputData};

/// Data associated with a specific date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateData {
    /// The date of this run
    pub date: Date,

    /// Git commit hash of the compiler these results were obtained with.
    pub commit: String,

    /// Keyed by crate names / phases depending on request's group by field;
    ///   - rss: u64 of memory usage in megabytes
    ///   - time: f64 of duration for compiling in seconds
    pub data: HashMap<String, Recording>,
}

impl DateData {
    pub fn for_day(day: &TestRun,
                   crate_names: &[String],
                   phases: &[String],
                   group_by: GroupBy)
                   -> DateData {
        let crates = crate_names.into_iter()
            .filter_map(|crate_name| day.by_crate.get(crate_name).map(|krate| (crate_name, krate)))
            .collect::<Vec<_>>();

        let mut data = HashMap::new();
        for phase_name in phases {
            for &(crate_name, krate) in &crates {
                let entry = match group_by {
                    GroupBy::Crate => data.entry(crate_name.to_string()),
                    GroupBy::Phase => data.entry(phase_name.to_string()),
                };

                entry.or_insert(Recording::new()).record(krate.get(phase_name));
            }
        }

        DateData {
            date: day.date,
            commit: day.commit.clone(),
            data: data,
        }
    }
}

// TODO better name (can't use Timing since we don't have a percent...)
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Recording {
    time: f64,
    rss: u64,
}

impl Recording {
    fn new() -> Recording {
        Recording {
            time: 0.0,
            rss: 0,
        }
    }

    fn record(&mut self, phase: Option<&Timing>) {
        if let Some(phase) = phase {
            self.time += phase.time;
            self.rss = max(self.rss, phase.rss.unwrap_or(0));
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupBy {
    #[serde(rename="crate")]
    Crate,
    #[serde(rename="phase")]
    Phase,
}

#[test]
fn serialize_kind() {
    assert_eq!(serde_json::to_string(&GroupBy::Crate).unwrap(),
               r#""crate""#);
    assert_eq!(serde_json::from_str::<GroupBy>(r#""crate""#).unwrap(),
               GroupBy::Crate);
    assert_eq!(serde_json::to_string(&GroupBy::Phase).unwrap(),
               r#""phase""#);
    assert_eq!(serde_json::from_str::<GroupBy>(r#""phase""#).unwrap(),
               GroupBy::Phase);
}

fn handle_summary(r: &mut Request) -> IronResult<Response> {
    fn summarize(benchmark: &SummarizedWeek, rustc: &SummarizedWeek) -> summary::Percent {
        let mut sum = 0.0;
        let mut count = 0;
        for krate in benchmark.by_crate.values() {
            if krate.contains_key("total") {
                sum += krate["total"];
                count += 1;
            }
        }

        if rustc.by_crate["total"].contains_key("total") {
            sum += 2.0 * rustc.by_crate["total"]["total"];
            count += 2;
        }

        summary::Percent(sum / (count as f64))
    }

    fn breakdown(benchmark: &SummarizedWeek,
                 rustc: &SummarizedWeek)
                 -> HashMap<String, summary::Percent> {
        let mut per_bench = HashMap::new();

        for (crate_name, krate) in &benchmark.by_crate {
            let val = krate.get("total").cloned().unwrap_or(0.0);
            per_bench.insert(crate_name.to_string(), summary::Percent(val));
        }

        let bootstrap = if rustc.by_crate["total"].contains_key("total") {
            rustc.by_crate["total"]["total"]
        } else {
            0.0
        };
        per_bench.insert("bootstrap".to_string(), summary::Percent(bootstrap));

        per_bench
    }

    route_handler::handler_get(r, |data| {
        let dates = data.summary_rustc
            .summary
            .iter()
            .map(|s| s.date)
            .collect::<Vec<_>>();

        // overall number for each week
        let summaries = data.summary_benchmarks
            .summary
            .iter()
            .enumerate()
            .map(|(i, s)| summarize(s, &data.summary_rustc.summary[i]))
            .collect::<Vec<_>>();

        // per benchmark, per week
        let breakdown_data = data.summary_benchmarks
            .summary
            .iter()
            .enumerate()
            .map(|(i, s)| breakdown(s, &data.summary_rustc.summary[i]))
            .collect::<Vec<_>>();

        summary::Response {
            total_summary: summarize(&data.summary_benchmarks.total, &data.summary_rustc.total),
            total_breakdown: breakdown(&data.summary_benchmarks.total, &data.summary_rustc.total),
            breakdown: breakdown_data,
            summaries: summaries,
            dates: dates,
        }
    })
}

fn handle_info(r: &mut Request) -> IronResult<Response> {
    fn sort(set: &HashSet<String>) -> Vec<String> {
        let mut vec = set.into_iter().cloned().collect::<Vec<_>>();
        vec.sort();
        vec
    }

    route_handler::handler_get(r, |data| {
        info::Response {
            crates: sort(&data.crate_list),
            phases: sort(&data.phase_list),
            benchmarks: sort(&data.benchmarks),
            as_of: data.last_date,
        }
    })
}

fn handle_data(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<data::Request, _, _>(r, |body, data| {
        let mut result = data.kinded_range(body.kind, &body.start_date, &body.end_date)
            .into_iter()
            .map(|day| DateData::for_day(day, &body.crates, &body.phases, body.group_by))
            .collect::<Vec<_>>();

        // Return everything from the first non-empty data to the last non-empty data.
        // Data may contain "holes" of empty data.
        let first_idx = result.iter().position(|day| !day.data.is_empty()).unwrap_or(0);
        let last_idx = result.iter().rposition(|day| !day.data.is_empty()).unwrap_or(0);
        let result = result.drain(first_idx..(last_idx + 1)).collect();
        data::Response(result)
    })
}

fn handle_tabular(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<tabular::Request, _, _>(r, |body, data| {
        let day = data.kinded_end_day(body.kind, &body.date);

        tabular::Response {
            date: day.date,
            commit: day.commit.clone(),
            data: day.by_crate.clone(),
        }
    })
}

fn handle_days(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<days::Request, _, _>(r, |body, data| {
        days::Response {
            a: DateData::for_day(&data.kinded_start_day(body.kind, &body.date_a),
                                 &body.crates,
                                 &body.phases,
                                 body.group_by),
            b: DateData::for_day(&data.kinded_end_day(body.kind, &body.date_b),
                                 &body.crates,
                                 &body.phases,
                                 body.group_by),
        }
    })
}

fn handle_stats(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<stats::Request, _, _>(r, |body, data| {
        assert!(body.kind != Kind::Benchmarks || body.crates.iter().all(|s| s != "total"));

        let mut start_date = body.start_date.as_start(data.last_date);
        let mut end_date = body.end_date.as_end(data.last_date);

        let mut counted = Vec::new();

        // Iterate over date range.
        for today_data in data.kinded_range(body.kind, &body.start_date, &body.end_date) {
            if !today_data.by_crate.is_empty() {
                if counted.is_empty() {
                    start_date = today_data.date;
                }
                end_date = today_data.date;
                counted.push(today_data);
            }
        }

        let mut crates = HashMap::new();
        for crate_name in body.crates {
            let stats = Stats::from(&counted, &crate_name, &body.phases);
            crates.insert(crate_name.to_string(), stats);
        }

        stats::Response {
            start_date: start_date,
            end_date: end_date,
            crates: crates,
        }
    })
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Stats {
    first: f64,
    last: f64,
    min: f64,
    max: f64,
    mean: f64,
    variance: f64,
    trend: f64,
    trend_b: f64,
    n: usize,
}

impl Stats {
    fn from(data: &[&TestRun], crate_name: &str, phases: &[String]) -> Stats {
        let sums = data.iter()
            .filter(|day| if let Some(krate) = day.by_crate.get(crate_name) {
                !krate.is_empty()
            } else {
                false
            })
            .map(|day| {
                let krate = &day.by_crate[crate_name];
                let mut sum = 0.0;
                for phase in phases {
                    sum += krate[phase].time;
                }
                sum
            })
            .collect::<Vec<_>>();

        if sums.is_empty() {
            return Stats::default();
        }

        let first = sums[0];
        let last = *sums.last().unwrap();

        let mut min = first;
        let mut max = first;
        let q1_idx = data.len() / 4;
        let q4_idx = 3 * data.len() / 4;
        let mut total = 0.0;
        let mut q1_total = 0.0;
        let mut q4_total = 0.0;
        for (i, &cur) in sums.iter().enumerate() {
            min = min.min(cur);
            max = max.max(cur);

            total += cur;
            if i < q1_idx {
                // Within the first quartile
                q1_total += cur;
            }
            if i >= q4_idx {
                // Within the fourth quartile
                q4_total += cur;
            }
        }

        // Calculate the variance
        let mean = total / (sums.len() as f64);
        let mut var_total = 0.0;
        for sum in &sums {
            let diff = sum - mean;
            var_total += diff * diff;
        }
        let variance = var_total / ((sums.len() - 1) as f64);

        let trend = if sums.len() >= 10 && sums.len() == data.len() {
            let q1_mean = q1_total / (q1_idx as f64);
            let q4_mean = q4_total / ((data.len() - q4_idx) as f64);
            100.0 * ((q4_mean - q1_mean) / first)
        } else {
            0.0
        };
        let trend_b = 100.0 * ((last - first) / first);

        Stats {
            first: first,
            last: last,
            min: min,
            max: max,
            mean: mean,
            variance: variance,
            trend: trend,
            trend_b: trend_b,
            n: sums.len(),
        }
    }
}

fn on_push(req: &mut Request) -> IronResult<Response> {
    // FIXME we are throwing everything away and starting again. It would be
    // better to read just the added files. These should be available in the
    // body of the request.

    debug!("received onpush hook");

    let mut responder = || {
        let repo_path = get_repo_path()?;

        git::update_repo(&repo_path)?;

        info!("updating from filesystem...");
        let new_data = InputData::from_fs(&repo_path)?;

        // Retrieve the stored InputData from the request.
        let rwlock = req.get::<State<InputData>>().unwrap();
        let mut data = rwlock.write().unwrap();

        // Write the new data back into the request
        *data = new_data;

        Ok(serde_json::to_value("Successfully updated from filesystem"))
    };

    Ok(route_handler::unwrap_with_or_internal_err(responder(), route_handler::respond))
}

pub fn chain(data: InputData) -> Chain {
    let mut router = Router::new();

    router.get("/summary", handle_summary);
    router.get("/info", handle_info);
    router.post("/data", handle_data);
    router.post("/get_tabular", handle_tabular);
    router.post("/get", handle_days);
    router.post("/stats", handle_stats);
    router.post("/onpush", on_push);

    let mut chain = Chain::new(router);
    chain.link(State::<InputData>::both(data));

    chain
}

pub fn start(data: InputData) {
    Iron::new(chain(data)).http(::SERVER_ADDRESS).unwrap();
}
