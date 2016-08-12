use std::collections::HashMap;
use std::cmp::max;

use iron::{Iron, Chain};
use router::Router;
use persistent::State;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde_json::builder::ObjectBuilder;
use serde_json::Value;
use serde;

use SERVER_ADDRESS;
use errors::*;
use load::{SummarizedWeek, Kind, TestRun, InputData, Timing};
use util::{start_idx, end_idx};

const JS_DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.000Z";

// Boilerplate for parsing and responding to both GET and POST requests.
mod handler {
    use std::ops::Deref;
    use std::io::Read;

    use serde;
    use serde_json::{self, Value};
    use iron::prelude::*;
    use iron::status;
    use persistent::State;

    use load::InputData;
    use errors::*;

    fn respond(res: Result<Value>) -> IronResult<Response> {
        use iron::headers::{ContentType, AccessControlAllowOrigin};
        use iron::mime::{Mime, TopLevel, SubLevel};
        use iron::modifiers::Header;

        let mut resp = match res {
            Ok(json) => {
                let mut resp = Response::with((status::Ok, serde_json::to_string(&json).unwrap()));
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

    pub trait PostHandler: Sized {
        fn handle(_body: Self, _data: &InputData) -> Result<Value>;

        fn handler(req: &mut Request) -> IronResult<Response>
            where Self: serde::Deserialize {

            let rwlock = req.get::<State<InputData>>().unwrap();
            let data = rwlock.read().unwrap();

            let mut buf = String::new();
            let res = match req.body.read_to_string(&mut buf).unwrap() {
                0 => Err("POST handler with 0 length body.".into()),
                _ => Self::handle(serde_json::from_str(&buf).unwrap(), data.deref())
            };

            respond(res)
        }
    }

    pub trait GetHandler: Sized {
        fn handle(_data: &InputData) -> Result<Value>;

        fn handler(req: &mut Request) -> IronResult<Response> {
            let rwlock = req.get::<State<InputData>>().unwrap();
            let data = rwlock.read().unwrap();

            respond(Self::handle(data.deref()))
        }
    }
}

use self::handler::{PostHandler, GetHandler};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GroupBy {
    Crate,
    Phase,
}

impl serde::Deserialize for GroupBy {
    fn deserialize<D>(deserializer: &mut D) -> ::std::result::Result<GroupBy, D::Error>
        where D: serde::de::Deserializer
    {
        struct GroupByVisitor;

        impl serde::de::Visitor for GroupByVisitor {
            type Value = GroupBy;

            fn visit_str<E>(&mut self, value: &str) -> ::std::result::Result<GroupBy, E>
                where E: serde::de::Error
            {
                match value {
                    "crate" => Ok(GroupBy::Crate),
                    "phase" => Ok(GroupBy::Phase),
                    _ => {
                        let msg = format!("unexpected {} value for group by", value);
                        Err(serde::de::Error::custom(msg))
                    }
                }
            }
        }

        deserializer.deserialize(GroupByVisitor)
    }
}

enum OptionalDate {
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

struct Summary;
impl GetHandler for Summary {
    fn handle(data: &InputData) -> Result<Value> {
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

        // per benchmark, per week
        let breakdown_data = data.summary_benchmarks.summary.iter().enumerate().map(|(i, s)| {
            breakdown(s, &data.summary_rustc.summary[i])
        }).collect::<Vec<Value>>();

        Ok(ObjectBuilder::new()
            .insert("total_summary", summarize(&data.summary_benchmarks.total, &data.summary_rustc.total))
            .insert("total_breakdown", breakdown(&data.summary_benchmarks.total, &data.summary_rustc.total))
            .insert("breakdown", breakdown_data)
            .insert("summaries", summaries)
            .insert("dates", dates)
            .build())
    }
}

struct Info;
impl GetHandler for Info {
    fn handle(data: &InputData) -> Result<Value> {
        Ok(ObjectBuilder::new()
            .insert("crates", &data.crate_list)
            .insert("phases", &data.phase_list)
            .insert("benchmarks", &data.benchmarks)
            .build())
    }
}

fn get_data_for_date(day: &TestRun, crate_names: &[String], phases: &[String], group_by: GroupBy) -> Value {
    #[derive(Serialize)]
    struct Recording { // TODO better name (can't use Timing since we don't have a percent...)
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

    let crates = crate_names.into_iter().filter_map(|crate_name| {
        day.by_crate.get(crate_name).map(|krate| {
            (crate_name, krate)
        })
    }).collect::<Vec<_>>();

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

#[derive(Deserialize)]
struct Data { // XXX naming
    #[serde(rename(deserialize="start"))]
    start_date: OptionalDate,
    #[serde(rename(deserialize="end"))]
    end_date: OptionalDate,
    kind: Kind,
    group_by: GroupBy,
    crates: Vec<String>,
    phases: Vec<String>,
}

impl PostHandler for Data {
    fn handle(body: Self, data: &InputData) -> Result<Value> {
        let mut result = Vec::new();
        let mut first_idx = None;
        let mut last_idx = 0;
        // Iterate over date range.
        let start_idx = start_idx(data.by_kind(body.kind), body.start_date.as_start(data));
        let end_idx = end_idx(data.by_kind(body.kind), body.end_date.as_end(data));
        for i in start_idx..(end_idx + 1) {
            let today_data = get_data_for_date(
                &data.by_kind(body.kind)[i],
                &body.crates,
                &body.phases,
                body.group_by
            );

            if !today_data.find("data").unwrap().as_object().unwrap().is_empty() {
                last_idx = i - start_idx;
                if first_idx == None {
                    first_idx = Some(i - start_idx);
                }
            }

            result.push(today_data);
        }

        // Trim the data
        let result = result.drain(first_idx.unwrap()..(last_idx+1)).collect::<Vec<_>>();
        Ok(Value::Array(result))
    }
}

#[derive(Deserialize)]
struct Tabular { // XXX naming
    kind: Kind,
    date: OptionalDate,
}

impl PostHandler for Tabular {
    fn handle(body: Self, data: &InputData) -> Result<Value> {
        let kind_data = data.by_kind(body.kind);
        let day = &kind_data[end_idx(kind_data, body.date.as_end(data))];

        Ok(ObjectBuilder::new()
            .insert("date", day.date.format(JS_DATE_FORMAT).to_string())
            .insert("commit", &day.commit)
            .insert("data", &day.by_crate)
            .build())
    }
}

#[derive(Deserialize)]
struct Days { // XXX naming
    kind: Kind,
    dates: Vec<OptionalDate>,
    crates: Vec<String>,
    phases: Vec<String>,
    group_by: GroupBy,
}

impl PostHandler for Days {
    fn handle(body: Self, data: &InputData) -> Result<Value> {
        let data = data.by_kind(body.kind);
        let mut result = Vec::new();
        for date in body.dates {
            if let OptionalDate::Date(date) = date {
                let day = get_data_for_date(
                    &data[end_idx(data, date)],
                    &body.crates,
                    &body.phases,
                    body.group_by
                );
                result.push(day);
            }
        }
        Ok(Value::Array(result))
    }
}

#[derive(Deserialize)]
struct Stats { // XXX naming
    kind: Kind,
    #[serde(rename(deserialize="start"))]
    start_date: OptionalDate,
    #[serde(rename(deserialize="end"))]
    end_date: OptionalDate,
    // kind rustc only: crate or phase can be 'total'
    crates: Vec<String>,
    phases: Vec<String>,
}

impl PostHandler for Stats {
    fn handle(body: Self, data: &InputData) -> Result<Value> {
        if body.kind == Kind::Benchmarks && body.crates.iter().any(|s| s == "total") {
            return Err("unexpected total crate with benchmarks kind".into());
        }

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

        Ok(ObjectBuilder::new()
            .insert("startDate", start_date.format(JS_DATE_FORMAT).to_string())
            .insert("endDate", end_date.format(JS_DATE_FORMAT).to_string())
            .insert("crates", crates.build())
            .build())
    }
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
        if i < q1_idx { // Within the first quartile
            q1_total += cur;
        }
        if i >= q4_idx { // Within the fourth quartile
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

pub fn start(data: InputData) {
    let mut router = Router::new();

    router.get("/summary", Summary::handler);
    router.get("/info", Info::handler);
    router.post("/data", Data::handler);
    router.post("/get_tabular", Tabular::handler);
    router.post("/get", Days::handler);
    router.post("/stats", Stats::handler);

    let mut chain = Chain::new(router);
    chain.link(State::<InputData>::both(data));

    Iron::new(chain).http(SERVER_ADDRESS).unwrap();
}
