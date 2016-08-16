// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{HashMap, HashSet};
use std::cmp::max;

use iron::prelude::*;
use router::Router;
use persistent::State;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde_json::builder::ObjectBuilder;
use serde_json::Value;
use serde;

use git;
use load::{SummarizedWeek, Kind, TestRun, InputData, Timing};
use route_handler;
use util::{get_repo_path, start_idx, end_idx};

const JS_DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.000Z";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupBy {
    #[serde(rename="crate")]
    Crate,
    #[serde(rename="phase")]
    Phase,
}

pub enum OptionalDate {
    Date(NaiveDateTime),
    CouldNotParse(String),
}

impl OptionalDate {
    fn as_start(&self, data: &InputData) -> NaiveDateTime {
        // Handle missing start by returning 30 days before end.
        if let OptionalDate::Date(date) = *self {
            date
        } else {
            let end = self.as_end(data);
            let start = (end - Duration::days(30)).timestamp();
            NaiveDateTime::from_timestamp(start, 0)
        }
    }

    fn as_end(&self, data: &InputData) -> NaiveDateTime {
        // Handle missing end by using the last available date.
        if let OptionalDate::Date(date) = *self {
            date
        } else {
            data.last_date
        }
    }
}

impl serde::Deserialize for OptionalDate {
    fn deserialize<D>(deserializer: &mut D) -> ::std::result::Result<OptionalDate, D::Error>
        where D: serde::de::Deserializer
    {
        struct DateVisitor;

        impl serde::de::Visitor for DateVisitor {
            type Value = OptionalDate;

            fn visit_str<E>(&mut self, value: &str) -> ::std::result::Result<OptionalDate, E>
                where E: serde::de::Error
            {
                match NaiveDate::parse_from_str(value, "%a %b %d %Y") {
                    Ok(date) => Ok(OptionalDate::Date(date.and_hms(0, 0, 0))),
                    Err(err) => {
                        if !value.is_empty() {
                            println!("bad date {:?}: {:?}", value, err);
                        }
                        Ok(OptionalDate::CouldNotParse(value.to_string()))
                    }
                }
            }
        }

        deserializer.deserialize(DateVisitor)
    }
}

impl serde::Serialize for OptionalDate {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: serde::Serializer
    {
        match *self {
            OptionalDate::Date(date) => {
                serializer.serialize_str(&date.format("%a %b %d %Y").to_string())
            }
            OptionalDate::CouldNotParse(_) => serializer.serialize_str(""), // TODO: Warning?
        }
    }
}

fn handle_summary(r: &mut Request) -> IronResult<Response> {
    fn summarize(benchmark: &SummarizedWeek, rustc: &SummarizedWeek) -> String {
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

        format!("{:.1}", sum / (count as f64))
    }

    fn breakdown(benchmark: &SummarizedWeek, rustc: &SummarizedWeek) -> Value {
        let mut per_bench = ObjectBuilder::new();

        for (crate_name, krate) in &benchmark.by_crate {
            let val = krate.get("total").cloned().unwrap_or(0.0);
            per_bench = per_bench.insert(crate_name.as_str(), format!("{:.1}", val));
        }

        let bootstrap = if rustc.by_crate["total"].contains_key("total") {
            rustc.by_crate["total"]["total"]
        } else {
            0.0
        };
        per_bench = per_bench.insert("bootstrap", format!("{:.1}", bootstrap));

        per_bench.build()
    }

    route_handler::handler_get(r, |data| {
        let dates = data.summary_rustc
            .summary
            .iter()
            .map(|s| s.date.format(JS_DATE_FORMAT).to_string())
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
            .collect::<Vec<Value>>();

        ObjectBuilder::new()
            .insert("total_summary",
                    summarize(&data.summary_benchmarks.total, &data.summary_rustc.total))
            .insert("total_breakdown",
                    breakdown(&data.summary_benchmarks.total, &data.summary_rustc.total))
            .insert("breakdown", breakdown_data)
            .insert("summaries", summaries)
            .insert("dates", dates)
            .build()
    })
}

fn handle_info(r: &mut Request) -> IronResult<Response> {
    fn sort(set: &HashSet<String>) -> Vec<&String> {
        let mut vec = set.into_iter().collect::<Vec<_>>();
        vec.sort();
        vec
    }

    route_handler::handler_get(r, |data| {
        ObjectBuilder::new()
            .insert("crates", sort(&data.crate_list))
            .insert("phases", sort(&data.phase_list))
            .insert("benchmarks", sort(&data.benchmarks))
            .build()
    })
}

fn get_data_for_date(day: &TestRun,
                     crate_names: &[String],
                     phases: &[String],
                     group_by: GroupBy)
                     -> Value {
    #[derive(Serialize)]
    struct Recording {
        // TODO better name (can't use Timing since we don't have a percent...)
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
                self.rss = max(self.rss, phase.rss.unwrap());
            }
        }
    }

    let crates = crate_names.into_iter()
        .filter_map(|crate_name| day.by_crate.get(crate_name).map(|krate| (crate_name, krate)))
        .collect::<Vec<_>>();

    let mut data = HashMap::new();
    for phase_name in phases {
        for &(crate_name, krate) in &crates {
            let entry = match group_by {
                GroupBy::Crate => data.entry(crate_name),
                GroupBy::Phase => data.entry(phase_name),
            };

            entry.or_insert(Recording::new()).record(krate.get(phase_name));
        }
    }

    ObjectBuilder::new()
        .insert("date", day.date.format(JS_DATE_FORMAT).to_string())
        .insert("commit", day.commit.clone())
        .insert("data", data)
        .build()
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    // XXX naming
    #[serde(rename="start")]
    pub start_date: OptionalDate,
    #[serde(rename="end")]
    pub end_date: OptionalDate,
    pub kind: Kind,
    pub group_by: GroupBy,
    pub crates: Vec<String>,
    pub phases: Vec<String>,
}

fn handle_data(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<_, Data>(r, |body, data| {
        let mut result = Vec::new();
        let mut first_idx = None;
        let mut last_idx = 0;
        // Iterate over date range.
        let start_idx = start_idx(data.by_kind(body.kind), body.start_date.as_start(data));
        let end_idx = end_idx(data.by_kind(body.kind), body.end_date.as_end(data));
        for i in start_idx..(end_idx + 1) {
            let today_data = get_data_for_date(&data.by_kind(body.kind)[i],
                                               &body.crates,
                                               &body.phases,
                                               body.group_by);

            if !today_data.find("data").unwrap().as_object().unwrap().is_empty() {
                last_idx = i - start_idx;
                if first_idx == None {
                    first_idx = Some(i - start_idx);
                }
            }

            result.push(today_data);
        }

        // Trim the data
        let result = result.drain(first_idx.unwrap()..(last_idx + 1)).collect::<Vec<_>>();
        Value::Array(result)
    })
}

#[derive(Serialize, Deserialize)]
pub struct Tabular {
    // XXX naming
    pub kind: Kind,
    pub date: OptionalDate,
}

fn handle_tabular(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<_, Tabular>(r, |body, data| {
        let kind_data = data.by_kind(body.kind);
        let day = &kind_data[end_idx(kind_data, body.date.as_end(data))];

        ObjectBuilder::new()
            .insert("date", day.date.format(JS_DATE_FORMAT).to_string())
            .insert("commit", &day.commit)
            .insert("data", &day.by_crate)
            .build()
    })
}

#[derive(Serialize, Deserialize)]
pub struct Days {
    // XXX naming
    pub kind: Kind,
    pub dates: Vec<OptionalDate>,
    pub crates: Vec<String>,
    pub phases: Vec<String>,
    pub group_by: GroupBy,
}

fn handle_days(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<_, Days>(r, |body, data| {
        let data = data.by_kind(body.kind);
        let mut result = Vec::new();
        for date in body.dates {
            if let OptionalDate::Date(date) = date {
                let day = get_data_for_date(&data[end_idx(data, date)],
                                            &body.crates,
                                            &body.phases,
                                            body.group_by);
                result.push(day);
            }
        }
        Value::Array(result)
    })
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    // XXX naming
    pub kind: Kind,
    #[serde(rename="start")]
    pub start_date: OptionalDate,
    #[serde(rename="end")]
    pub end_date: OptionalDate,
    // kind rustc only: crate or phase can be 'total'
    pub crates: Vec<String>,
    pub phases: Vec<String>,
}

fn handle_stats(r: &mut Request) -> IronResult<Response> {
    route_handler::handler_post::<_, Stats>(r, |body, data| {
        assert!(body.kind != Kind::Benchmarks || body.crates.iter().all(|s| s != "total"));

        let kinded_data = data.by_kind(body.kind);
        let mut start_date = body.start_date.as_start(data);
        let mut end_date = body.end_date.as_end(data);

        let mut counted = Vec::new();

        // Iterate over date range.
        let start_idx = start_idx(kinded_data, start_date);
        let end_idx = end_idx(kinded_data, end_date);
        for i in start_idx..(end_idx + 1) {
            let today_data = &kinded_data[i];
            if !today_data.by_crate.is_empty() {
                if counted.is_empty() {
                    start_date = today_data.date;
                }
                end_date = today_data.date;
                counted.push(today_data);
            }
        }

        let mut crates = ObjectBuilder::new();
        for crate_name in body.crates {
            let stats = mk_stats(&counted, &crate_name, &body.phases);
            crates = crates.insert(crate_name, stats);
        }

        ObjectBuilder::new()
            .insert("startDate", start_date.format(JS_DATE_FORMAT).to_string())
            .insert("endDate", end_date.format(JS_DATE_FORMAT).to_string())
            .insert("crates", crates.build())
            .build()
    })
}

fn mk_stats(data: &[&TestRun], crate_name: &str, phases: &[String]) -> Value {
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
        return ObjectBuilder::new()
            .insert("first", 0)
            .insert("last", 0)
            .insert("min", 0)
            .insert("max", 0)
            .insert("mean", 0)
            .insert("variance", 0)
            .insert("trend", 0)
            .insert("trend_b", 0)
            .insert("n", 0)
            .build();
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

    ObjectBuilder::new()
        .insert("first", first)
        .insert("last", last)
        .insert("min", min)
        .insert("max", max)
        .insert("mean", mean)
        .insert("variance", variance)
        .insert("trend", trend)
        .insert("trend_b", trend_b)
        .insert("n", sums.len())
        .build()
}

fn on_push(req: &mut Request) -> IronResult<Response> {
    // FIXME we are throwing everything away and starting again. It would be
    // better to read just the added files. These should be available in the
    // body of the request.

    println!("received onpush hook");

    let mut responder = || {
        let repo_path = get_repo_path()?;

        git::update_repo(&repo_path)?;

        println!("updating from filesystem...");
        let new_data = InputData::from_fs(&repo_path)?;

        // Retrieve the stored InputData from the request.
        let rwlock = req.get::<State<InputData>>().unwrap();
        let mut data = rwlock.write().unwrap();

        // Write the new data back into the request
        *data = new_data;

        Ok(Value::String("Successfully updated from filesystem".to_owned()))
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
