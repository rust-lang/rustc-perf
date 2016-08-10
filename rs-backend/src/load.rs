use std::collections::{HashMap, HashSet};
use std::cmp::{Ordering, max};
use std::fs::{self, File};
use std::path::Path;
use std::io::Read;

use chrono::{Duration, NaiveDateTime};
use serde_json::{self, Value};
use serde;

use util::start_idx;
use errors::*;

const WEEKS_IN_SUMMARY: usize = 12;

/// Contains the TestRun loaded from the file system.
/// Useful to avoid passing around each individual field, instead passing the
/// entire struct.
pub struct InputData {
    pub summary_rustc: Summary,
    pub summary_benchmarks: Summary,

    /// A set containing all crate names.
    pub crate_list: HashSet<String>,

    /// A set containing all phase names, across all crates.
    pub phase_list: HashSet<String>,

    /// TODO: Better docs. It's unknown how this is used on the client.  A set
    /// of test names. Test names are found from the file path, everything
    /// before the `--` is included.
    pub benchmarks: HashSet<String>,

    /// The last date that was seen while loading files. The DateTime variant is
    /// used here since the date may or may not contain a time. Since the
    /// timezone is not important, it isn't stored, hence the Naive variant.
    pub last_date: NaiveDateTime,

    /// Private due to access being exposed through by_kind method.
    /// Care must be taken to sort these after modifying them.
    data_rustc: Vec<TestRun>,
    data_benchmarks: Vec<TestRun>,
}

/// Allows storing `InputData` in Iron's persistent data structure.
impl ::iron::typemap::Key for InputData {
    type Value = InputData;
}

struct InputHeader {
    date: NaiveDateTime,
    commit: String,
}

impl InputData {
    /// Initialize `InputData from the file system.
    pub fn from_fs() -> Result<InputData> {
        // TODO: Read this at runtime, not hardcoded.
        let repo_loc = Path::new("../data");

        let mut last_date = None;
        let mut crate_list = HashSet::new();
        let mut phase_list = HashSet::new();
        let mut skipped = 0;
        let mut c_benchmarks_add = 0;

        let mut data_rustc = Vec::new();
        let mut data_benchmarks = Vec::new();
        let mut benchmarks = HashSet::new();

        // Read all files from repo_loc/processed
        let mut file_count = 0;
        for entry in fs::read_dir(repo_loc.join("processed"))? {
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
                skipped += 1;
                continue;
            }

            let contents: Value = match serde_json::from_str(&file_contents) {
                Ok(json) => json,
                Err(err) => {
                    println!("Failed to parse JSON for {}: {:?}", filename, err);
                    skipped += 1;
                    continue;
                }
            };
            if contents.find("times").unwrap().as_array().unwrap().is_empty() {
                skipped += 1;
                continue;
            }

            /// Parse date from JSON header in file contents.
            fn parse_from_header(date: &str) -> Result<NaiveDateTime> {
                // TODO: Determine correct format of header date and move into
                // variable/constant
                NaiveDateTime::parse_from_str(date, "%c %z")
                    .map_err(|err| err.into())
            }

            /// Parse date from filename.
            fn parse_from_filename(filename: &str) -> Result<NaiveDateTime> {
                let date_str = &filename[(filename.find("--").unwrap() + 2)..filename.find(".json").unwrap()];

                // Ignore seconds: they aren't key to the data processing, and
                // parsing them requires logic to replace them with 0 when
                // they are absent.
                NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d-%H-%M")
                    .map_err(|err| err.into())
            }

            let header = InputHeader {
                commit: contents.lookup("header.commit").unwrap().as_str().unwrap().to_string(),
                date: parse_from_header(contents.lookup("header.date").unwrap().as_str().unwrap())
                    .or_else(|_| parse_from_filename(&filename))?,
            };
            let date = header.date;

            let test_name = &filename[0..filename.find("--").unwrap()];

            let times = contents.find("times").unwrap().as_array().unwrap();
            if test_name == "rustc" {
                data_rustc.push(TestRun::new(date, header, times, true));

                for timing in times {
                    let crate_name = timing.find("crate").unwrap().as_str().unwrap().to_string();
                    crate_list.insert(crate_name);
                }
            } else {
                benchmarks.insert(test_name.to_string());
                let index = data_benchmarks.iter().position(|benchmark: &TestRun| benchmark.date == date);
                if let Some(index) = index {
                    c_benchmarks_add += 1;
                    let crate_name = times[0].find("crate").unwrap().as_str().unwrap();
                    data_benchmarks[index].by_crate.insert(test_name.to_string(),
                        make_times(times, false).remove(crate_name).unwrap());
                } else {
                    data_benchmarks.push(TestRun::new(date, header, times, false));
                }
            }

            for timing in times {
                for (phase, _) in timing.find("times").unwrap().as_object().unwrap() {
                    phase_list.insert(phase.to_string());
                }
            }

            if last_date.is_none() || last_date.as_ref().unwrap() < &date {
                last_date = Some(date);
            }
        }

        let last_date = last_date.expect("No dates found");

        data_rustc.sort();
        data_benchmarks.sort();

        // Post processing to generate the summary data.
        let summary_rustc = Summary::new(&data_rustc, last_date);
        let summary_benchmarks = Summary::new(&data_benchmarks, last_date);

        println!("{} total files", file_count);
        println!("{} skipped files", skipped);
        println!("{} bootstrap times", data_rustc.len());
        println!("{} benchmarks times", data_benchmarks.len());
        println!("{} benchmarks times (appended)", c_benchmarks_add);

        Ok(InputData {
            summary_rustc: summary_rustc,
            summary_benchmarks: summary_benchmarks,
            crate_list: crate_list,
            phase_list: phase_list,
            benchmarks: benchmarks,
            last_date: last_date,
            data_rustc: data_rustc,
            data_benchmarks: data_benchmarks,
        })
    }

    pub fn by_kind(&self, kind: Kind) -> &[TestRun] {
        match kind {
            Kind::Benchmarks => &self.data_benchmarks,
            Kind::Rustc => &self.data_rustc
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Kind {
    Benchmarks,
    Rustc,
}

impl Kind {
    pub fn from_str(kind: &str) -> Option<Kind> {
        match kind {
            "rustc" => Some(Kind::Rustc),
            "benchmarks" => Some(Kind::Benchmarks),
            _ => None
        }
    }
}

impl serde::Deserialize for Kind {
    fn deserialize<D>(deserializer: &mut D) -> ::std::result::Result<Kind, D::Error>
        where D: serde::de::Deserializer
    {
        struct KindVisitor;

        impl serde::de::Visitor for KindVisitor {
            type Value = Kind;

            fn visit_str<E>(&mut self, value: &str) -> ::std::result::Result<Kind, E>
                where E: serde::de::Error
            {
                match Kind::from_str(value) {
                    Some(group_by) => Ok(group_by),
                    None => {
                        let msg = format!("unexpected {} value for kind", value);
                        Err(serde::de::Error::custom(msg))
                    }
                }
            }
        }

        deserializer.deserialize(KindVisitor)
    }
}

/// The data loaded for a single date, and all associated crates.
pub struct TestRun {
    pub date: NaiveDateTime,
    pub commit: String,

    /// Map of crate names to a map of phases to timings per phase.
    pub by_crate: HashMap<String, HashMap<String, Timing>>
}

impl PartialEq for TestRun {
    fn eq(&self, other: &TestRun) -> bool {
        self.commit == other.commit && self.date == other.date
    }
}

impl Eq for TestRun {}

impl PartialOrd for TestRun {
    fn partial_cmp(&self, other: &TestRun) -> Option<Ordering> {
        Some(<TestRun as Ord>::cmp(self, other))
    }
}

impl Ord for TestRun {
    fn cmp(&self, other: &TestRun) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl TestRun {
    fn new(date: NaiveDateTime, header: InputHeader, times: &[Value], is_rustc: bool) -> TestRun {
        TestRun {
            date: date,
            commit: header.commit.clone(),
            by_crate: make_times(times, is_rustc)
        }
    }
}

/// Contains a single timing, associated with a phase (though the phase name
/// is not included in the timing).
#[derive(Debug, PartialEq)]
pub struct Timing {
    pub percent: f64,
    pub time: f64,
    pub rss: Option<u64>,
}

impl Timing {
    fn new() -> Timing {
        Timing {
            percent: 0.0,
            time: 0.0,
            rss: Some(0)
        }
    }
}

/// Run through the timings for a date and construct the `by_crate` field of TestRun.
fn make_times(timings: &[Value], is_rustc: bool) -> HashMap<String, HashMap<String, Timing>> {
    let mut by_crate = HashMap::new();
    let mut totals = HashMap::new();

    for timing in timings {
        let mut times = HashMap::new();
        for (phase_name, phase) in timing.find("times").unwrap().as_object().unwrap() {
            times.insert(phase_name.to_string(), Timing {
                percent: phase.find("percent").unwrap().as_f64().unwrap(),
                time: phase.find("time").unwrap().as_f64().unwrap(),
                rss: timing.find("rss")
                    .and_then(|rss| rss.find(phase_name))
                    .and_then(|phase| phase.as_u64()),
            });
        }

        let mut mem_values = Vec::new();
        if let Some(obj) = timing.find("rss").unwrap().as_object() {
            for (_, value) in obj {
                mem_values.push(value.as_u64().unwrap());
            }
        }

        times.insert("total".into(), Timing {
            percent: 100.0,
            time: timing.find("total").unwrap().as_f64().unwrap(),
            rss: Some(mem_values.into_iter().max().unwrap_or(0))
        });

        for phase in times.keys() {
            let mut entry = totals.entry(phase.to_string()).or_insert_with(Timing::new);
            entry.time += times[phase].time;
            entry.rss = max(times[phase].rss, entry.rss);
        }

        by_crate.insert(timing.find("crate").unwrap().as_str().unwrap().to_string(), times);
    }

    if is_rustc {
        by_crate.insert("total".into(), totals);
    }
    // TODO: calculate percentages
    by_crate
}

#[derive(Debug)]
pub struct SummarizedWeek {
    pub date: NaiveDateTime,

    /// Maps crate names to a map of phases to each phase's percent change
    /// from the previous date.
    pub by_crate: HashMap<String, HashMap<String, f64>>
}

pub struct Summary {
    pub total: SummarizedWeek,
    pub summary: Vec<SummarizedWeek>,
}

impl Summary {
    // Compute summary data. For each week, we find the last 3 weeks, and use
    // the median timing as the basis of the current week's summary.
    fn new(data: &[TestRun], last_date: NaiveDateTime) -> Summary {
        // 13 week long mapping of crate names to by-phase medians.
        // We steal using summarized week type here even though we're not
        // storing the percent changes yet.
        let mut medians: Vec<SummarizedWeek> = Vec::new();

        // In order to compute summaries for the previous 12 weeks, we need 13
        // weeks, using the 0th week to compute difference with first week and
        // totals.
        for i in 0..(WEEKS_IN_SUMMARY + 1) {
            let date = last_date - Duration::weeks(i as i64);

            // For a given date we'll get the three most recent sets of TestRun
            // and take the the median for each value.
            let start_idx = start_idx(data, date);
            assert!(start_idx >= 3, "Less than 3 days of data");
            let mut weeks = Vec::new();
            for idx in 0..3 {
                weeks.push(&data[start_idx - idx].by_crate);
            }

            medians.push(SummarizedWeek {
                date: date,
                by_crate: HashMap::new(),
            });
            for crate_name in weeks[0].keys() {
                if !weeks[1].contains_key(crate_name) || !weeks[2].contains_key(crate_name) {
                    continue;
                }

                let mut crate_medians = HashMap::new();
                for phase in weeks[0][crate_name].keys() {
                    if !weeks[1][crate_name].contains_key(phase) ||
                        !weeks[2][crate_name].contains_key(phase) {
                        continue;
                    }
                    // Find the median value.
                    let mut values = [
                        weeks[0][crate_name][phase].time,
                        weeks[1][crate_name][phase].time,
                        weeks[2][crate_name][phase].time
                    ];
                    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    let median = values[1];

                    crate_medians.insert(phase.to_string(), median);
                }

                medians.last_mut().unwrap().by_crate.insert(crate_name.to_string(), crate_medians);
            }
        }

        /// Compute the percent change for a given number from the week N-1 to
        /// week N, where week N-1 is the previous and week N is the current
        /// week.
        fn get_percent_change(previous: f64, current: f64) -> f64 {
            ((current - previous) / previous) * 100.0
        }

        fn compare_weeks(week0: &SummarizedWeek, week1: &SummarizedWeek) -> SummarizedWeek {
            let mut current_week = HashMap::new();
            for crate_name in week0.by_crate.keys() {
                if !week1.by_crate.contains_key(crate_name) {
                    continue;
                }

                let mut current_crate = HashMap::new();
                for phase in week0.by_crate[crate_name].keys() {
                    if !week1.by_crate[crate_name].contains_key(phase) {
                        continue;
                    }

                    // TODO: Some form of warning is a good idea?
                    // If this is triggered for both the 0th overall week and
                    // the last week, then that phase won't be recorded in
                    // totals no matter what happens in between.
                    if week0.by_crate[crate_name][phase] > 0.0 &&
                        week1.by_crate[crate_name][phase] > 0.0 {
                        current_crate.insert(phase.to_string(),
                            get_percent_change(week0.by_crate[crate_name][phase],
                                week1.by_crate[crate_name][phase]));
                    }
                }
                current_week.insert(crate_name.to_string(), current_crate);
            }
            SummarizedWeek {
                date: week1.date,
                by_crate: current_week,
            }
        }

        // 12 week long mapping of crate names to by-phase percent changes with
        // the previous week.
        let mut weeks: Vec<SummarizedWeek> = Vec::new();
        for window in medians.windows(2) {
            // We want to compare week 1 to week 0; since we want the change from
            // week 1 to week 0.
            weeks.push(compare_weeks(&window[1], &window[0]));
        }

        assert_eq!(weeks.len(), 12);

        // See window comparison; compare from last week (which is the oldest)
        // to the first week.
        let totals = compare_weeks(medians.last().unwrap(), &medians[0]);

        for week in &mut weeks {
            for crate_name in totals.by_crate.keys() {
                if !week.by_crate.contains_key(crate_name) {
                    week.by_crate.insert(crate_name.to_string(), HashMap::new());
                }
            }
        }

        Summary {
            total: totals,
            summary: weeks
        }
    }
}
