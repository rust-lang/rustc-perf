use std::collections::{HashMap};
use std::cmp::{max, min};
use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;
use persistent::State;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use json::{self, JsonValue};

use SERVER_ADDRESS;
use errors::*;
use load::{SummarizedWeek, Kind, TestRun, InputData};
use util::{start_idx, end_idx};

const JS_DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.000Z";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GroupBy {
    Crate,
    Phase,
}

impl GroupBy {
    fn from_str(kind: &str) -> Option<GroupBy> {
        match kind {
            "crate" => Some(GroupBy::Crate),
            "phase" => Some(GroupBy::Phase),
            _ => None
        }
    }
}

fn get_summary(_req_body: Option<JsonValue>, data: &InputData) -> Result<JsonValue> {
    let dates = data.summary_rustc.summary.iter()
        .map(|s| s.date.format(JS_DATE_FORMAT).to_string())
        .collect::<Vec<_>>();

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

    // overall number for each week
    let summaries = data.summary_benchmarks.summary.iter().enumerate().map(|(i, s)| {
        summarize(s, &data.summary_rustc.summary[i])
    }).collect::<Vec<_>>();

    fn breakdown(benchmark: &SummarizedWeek, rustc: &SummarizedWeek) -> JsonValue {
        let mut per_bench = HashMap::new();

        for (crate_name, krate) in &benchmark.by_crate {
            let val = krate.get("total").cloned().unwrap_or(0.0);
            per_bench.insert(crate_name.to_string(), format!("{:.1}", val).into());
        }

        let bootstrap = if rustc.by_crate["total"].contains_key("total") {
            rustc.by_crate["total"]["total"]
        } else {
            0.0
        };
        per_bench.insert("bootstrap".to_string(), format!("{:.1}", bootstrap).into());

        per_bench.into()
    }

    // per benchmark, per week
    let breakdown_data = data.summary_benchmarks.summary.iter().enumerate().map(|(i, s)| {
        breakdown(s, &data.summary_rustc.summary[i])
    }).collect::<Vec<JsonValue>>();

    Ok(object! {
        "total_summary" => summarize(&data.summary_benchmarks.total, &data.summary_rustc.total),
        "total_breakdown" => breakdown(&data.summary_benchmarks.total, &data.summary_rustc.total),
        "breakdown" => breakdown_data,
        "summaries" => summaries,
        "dates" => dates
    })
}

fn get_info(_req_body: Option<JsonValue>, data: &InputData) -> Result<JsonValue> {
    Ok(object! {
        "crates" => data.crate_list.iter().cloned().collect::<Vec<String>>(),
        "phases" => data.phase_list.iter().cloned().collect::<Vec<String>>(),
        "benchmarks" => data.benchmarks.iter().cloned().collect::<Vec<String>>()
    })
}

fn assert_request_body_present(body: Option<JsonValue>) -> Result<JsonValue> {
    match body {
        Some(body) => Ok(body),
        None => Err("Missing request body".into())
    }
}

fn get_kind_from_body(body: &JsonValue) -> Result<Kind> {
    let kind = body["kind"].as_str().unwrap();
    match Kind::from_str(kind) {
        Some(kind) => Ok(kind),
        None => Err(format!("bad value kind: {}", kind).into())
    }
}

fn get_group_by_from_body(body: &JsonValue) -> Result<GroupBy> {
    let group_by = body["group_by"].as_str().unwrap();
    match GroupBy::from_str(group_by) {
        Some(group_by) => Ok(group_by),
        None => Err(format!("bad value group_by: {}", group_by).into())
    }
}

fn parse_date(s: &str) -> Result<NaiveDateTime> {
    let date = match NaiveDate::parse_from_str(s, "%a %b %d %Y") {
        Ok(date) => date,
        Err(err) => return Err(err).chain_err(|| format!("while parsing {}", s))
    };

    Ok(date.and_hms(0, 0, 0))
}

fn end_date(body: &JsonValue, last_date: &NaiveDateTime) -> NaiveDateTime {
    // Handle missing end by using the last available date.
    if !body["end"].is_empty() {
        parse_date(body["end"].as_str().unwrap()).unwrap()
    } else {
        *last_date
    }
}

fn start_date(body: &JsonValue, last_date: &NaiveDateTime) -> NaiveDateTime {
    // Handle missing start by returning 30 days before end.
    if !body["start"].is_empty() {
        parse_date(body["start"].as_str().unwrap()).unwrap()
    } else {
        let end = end_date(body, last_date);
        let start = (end - Duration::days(30)).timestamp();
        NaiveDateTime::from_timestamp(start, 0)
    }
}

fn get_data_for_date(day: &TestRun, crates: &JsonValue, phases: &JsonValue, group_by: GroupBy) -> JsonValue {
    let mut data = JsonValue::new_object();
    if group_by == GroupBy::Crate {
        for crate_name in crates.members() {
            let crate_name = crate_name.as_str().unwrap();
            if let Some(krate) = day.by_crate.get(&*crate_name) {
                let mut mem = 0;
                let mut total = 0.0;
                for phase in phases.members() {
                    let phase = phase.as_str().unwrap();
                    if let Some(phase) = krate.get(phase) {
                        total += phase.time;
                        mem = max(mem, phase.rss.unwrap());
                    }
                }
                data[crate_name] = object! {
                    "time" => total,
                    "rss" => mem
                };
            }
        }
    } else { // group_by == GroupBy::Phase
        for phase in phases.members() {
            let phase = phase.as_str().unwrap();
            let mut total = 0.0;
            let mut mem = 0;
            for crate_name in crates.members() {
                let crate_name = crate_name.as_str().unwrap();
                if let Some(krate) = day.by_crate.get(crate_name) {
                    if let Some(phase) = krate.get(phase) {
                        total += phase.time;
                        mem = max(mem, phase.rss.unwrap());
                    }
                }
            }
            data[phase] = object! {
                "time" => total,
                "rss" => mem
            };
        }
    }

    object! {
        "date" => day.date.format(JS_DATE_FORMAT).to_string(),
        "commit" => day.commit.clone(),
        "data" => data
    }
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     start: Date,     // optional
//     end: Date,       // optional
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str],
//     group_by: 'crate' | 'phase',
// }
// crate (rustc only) or phase can be 'total'
fn get_data(req_body: Option<JsonValue>, data: &InputData) -> Result<JsonValue> {
    let body = assert_request_body_present(req_body)?;
    let kind = get_kind_from_body(&body)?;
    let group_by = get_group_by_from_body(&body)?;

    let mut result = Vec::new();
    let mut first_idx = None;
    let mut last_idx = 0;
    // Iterate over date range.
    let start_idx = start_idx(data.by_kind(kind), &start_date(&body, &data.last_date));
    let end_idx = end_idx(data.by_kind(kind), &end_date(&body, &data.last_date));
    for i in start_idx..(end_idx + 1) {
        let today_data = get_data_for_date(&data.by_kind(kind)[i], &body["crates"], &body["phases"], group_by);

        if !today_data["data"].is_empty() {
            last_idx = i - start_idx;
            if first_idx == None {
                first_idx = Some(i - start_idx);
            }
        }

        result.push(today_data);
    }

    // Trim the data
    let result = result.drain(first_idx.unwrap()..(last_idx+1)).collect::<Vec<_>>();
    Ok(result.into())
}

fn handle_date(date_str: &str) -> Result<NaiveDateTime> {
    match parse_date(date_str) {
        Ok(date) => Ok(date),
        Err(err) => Err(err).chain_err(|| format!("bad date: {:?}", date_str))
    }
}

fn get_date_from_body(body: &JsonValue) -> Result<NaiveDateTime> {
    if let Some(date_str) = body["date"].as_str() {
        handle_date(date_str)
    } else {
        Err(format!("non-string date: {:?}", body["date"]).into())
    }
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     date: Date
// }
fn get_tabular(req_body: Option<JsonValue>, data: &InputData) -> Result<JsonValue> {
    let body = assert_request_body_present(req_body)?;
    let kind = get_kind_from_body(&body)?;
    let date = get_date_from_body(&body)?;

    let data = data.by_kind(kind);
    let day = &data[end_idx(data, &date)];
    let mut by_crate = JsonValue::new_object();
    for (crate_name, krate) in &day.by_crate {
        let mut krate_obj = JsonValue::new_object();
        for (phase_name, timing) in krate {
            krate_obj[phase_name] = object! {
                "percent" => timing.percent,
                "time" => timing.time,
                "rss" => timing.rss
            };
        }

        by_crate[crate_name] = krate_obj;
    }

    Ok(object! {
        "date" => day.date.format(JS_DATE_FORMAT).to_string(),
        "commit" => day.commit.clone(),
        "data" => by_crate
    })
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     dates: [Date],
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str],
//     group_by: 'crate' | 'phase'
// }
// crate or phase can be 'total'
fn get_days(req_body: Option<JsonValue>, data: &InputData) -> Result<JsonValue> {
    let body = assert_request_body_present(req_body)?;
    let kind = get_kind_from_body(&body)?;
    let group_by = get_group_by_from_body(&body)?;

    let data = data.by_kind(kind);
    let mut result = JsonValue::new_array();
    for orig_date in body["dates"].members() {
        let date = match handle_date(orig_date.as_str().unwrap()) {
            Ok(date) => date,
            Err(err) => {
                println!("bad date {:?}: {:?}", orig_date, err);
                continue;
            }
        };
        let day = get_data_for_date(&data[end_idx(data, &date)], &body["crates"], &body["phases"], group_by);
        result.push(day)?;
    }
    Ok(result)
}

// Expected fields on body: {
//     kind: 'rustc' | 'benchmarks',
//     start: Date,     // optional
//     end: Date,       // optional
//     crates: [str],   // crate == benchmarks for benchmark mode
//     phases: [str]
// }
// crate (rustc only) or phase can be 'total'
fn get_stats(req_body: Option<JsonValue>, data: &InputData) -> Result<JsonValue> {
    let body = assert_request_body_present(req_body)?;
    let kind = get_kind_from_body(&body)?;

    if kind == Kind::Benchmarks && body["crates"].contains("total") {
        return Err("unexpected total crate with benchmarks kind".into());
    }

    let mut start_date = start_date(&body, &data.last_date);
    let mut end_date = end_date(&body, &data.last_date);

    let mut counted = Vec::new();
    // Iterate over date range.
    let start_idx = start_idx(data.by_kind(kind), &start_date);
    let end_idx = end_idx(data.by_kind(kind), &end_date);
    for i in start_idx..(end_idx + 1) {
        let today_data = &data.by_kind(kind)[i];
        if !today_data.by_crate.is_empty() {
            if counted.is_empty() {
                start_date = today_data.date;
            }
            end_date = today_data.date;
            counted.push(today_data);
        }
    }

    let mut crates = JsonValue::new_object();
    for crate_name in body["crates"].members() {
        let crate_name = crate_name.as_str().unwrap();
        crates[crate_name] = mk_stats(&counted, crate_name, &body["phases"]);
    }

    Ok(object! {
        "startDate" => start_date.format(JS_DATE_FORMAT).to_string(),
        "endDate" => end_date.format(JS_DATE_FORMAT).to_string(),
        "crates" => crates
    })
}

fn mk_stats(data: &[&TestRun], crate_name: &str, phases: &JsonValue) -> JsonValue {
    let mut count = 0;
    let mut first = 0;
    let skip_list = data.iter().enumerate().map(|(i, d)| {
        if d.by_crate.contains_key(crate_name) && !d.by_crate[crate_name].is_empty() {
            if count == 0 {
                first = i;
            }

            count += 1;
            false
        } else {
            true
        }
    }).collect::<Vec<_>>();

    if count == 0 {
        return object! {
            "first" => 0,
            "last" => 0,
            "min" => 0,
            "max" => 0,
            "mean" => 0,
            "variance" => 0,
            "trend" => 0,
            "trend_b" => 0,
            "n" => 0
        };
    }

    let sums = data.iter().enumerate().map(|(i, d)| {
        if skip_list[i] {
            return 0.0;
        }

        let krate = &d.by_crate[crate_name];
        let mut sum = 0.0;
        for phase in phases.members() {
            sum += krate[phase.as_str().unwrap()].time;
        }
        sum
    }).collect::<Vec<_>>();

    let first = sums[first];
    let last = *sums.last().unwrap();

    let mut min = first;
    let mut max = first;
    let q1_idx = data.len() / 4;
    let q4_idx = 3 * data.len() / 4;
    let mut total = 0.0;
    let mut q1_total = 0.0;
    let mut q4_total = 0.0;
    for i in 0..data.len() {
        if skip_list[i] {
            continue;
        }
        let cur = sums[i];
        total += cur;
        min = min.min(cur);
        max = max.max(cur);
        if i < q1_idx { // Within the first quartile
            q1_total += cur;
        }
        if i >= q4_idx { // Within the fourth quartile
            q4_total += cur;
        }
    }

    // Calculate the variance
    let mean = total / (count as f64);
    let mut var_total = 0.0;
    for i in 0..data.len() {
        if skip_list[i] {
            continue;
        }
        let diff = sums[i] - mean;
        var_total += diff * diff;
    }
    let variance = var_total / ((count - 1) as f64);

    let trend = if count >= 10 && count == data.len() {
        let q1_mean = q1_total / (q1_idx as f64);
        let q4_mean = q4_total / ((data.len() - q4_idx) as f64);
        100.0 * ((q4_mean - q1_mean) / first)
    } else {
        0.0
    };
    let trend_b = 100.0 * ((last - first) / first);

    object! {
        "first" => first,
        "last" => last,
        "min" => min,
        "max" => max,
        "mean" => mean,
        "variance" => variance,
        "trend" => trend,
        "trend_b" => trend_b,
        "n" => count
    }
}

/// Pre and post processes the request and response to prepare it for the
/// handler passed in. Specifically, it parses JSON data from the request if
/// the request had greater than 0 length, and hands that into the handler.
/// Post-processing consists of applying access control headers and [potentially]
/// printing the error message.
fn get_handler<F>(handler: F, req: &mut Request) -> IronResult<Response>
    where F: Fn(Option<JsonValue>, &InputData) -> Result<JsonValue> {
    use std::ops::Deref;
    use iron::headers::{ContentType, AccessControlAllowOrigin};
    use iron::mime::{Mime, TopLevel, SubLevel};
    use iron::modifiers::Header;

    let rwlock = req.get::<State<InputData>>().unwrap();
    let data = rwlock.read().unwrap();

    let mut buf = String::new();
    let res = match req.body.read_to_string(&mut buf).unwrap() {
        0 => handler(None, data.deref()),
        _ => handler(Some(json::parse(&buf).unwrap()), data.deref())
    };

    let mut resp = match res {
        Ok(json) => {
            let mut resp = Response::with((status::Ok, json.dump()));
            resp.set_mut(Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![]))));
            resp
        },
        Err(err) => {
            // TODO: Print to stderr
            println!("An error occurred: {:?}", err);
            Response::with((status::InternalServerError, err.to_string()))
        }
    };
    resp.set_mut(Header(AccessControlAllowOrigin::Any));
    Ok(resp)
}

pub fn start(data: InputData) {
    let mut router = Router::new();

    router.get("/summary", |r: &mut Request| get_handler(get_summary, r));
    router.get("/info", |r: &mut Request| get_handler(get_info, r));
    router.post("/data", |r: &mut Request| get_handler(get_data, r));
    router.post("/get_tabular", |r: &mut Request| get_handler(get_tabular, r));
    router.post("/get", |r: &mut Request| get_handler(get_days, r));
    router.post("/stats", |r: &mut Request| get_handler(get_stats, r));

    let mut chain = Chain::new(router);
    chain.link(State::<InputData>::both(data));

    Iron::new(chain).http(SERVER_ADDRESS).unwrap();
}
