#[macro_use]
extern crate lazy_static;
extern crate rustc_perf;
extern crate serde_json;
extern crate serde;

use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use std::cmp::PartialEq;

use serde::{Deserialize, Serialize};

use rustc_perf::server::{self, GroupBy};
use rustc_perf::load::{Kind, InputData};
use rustc_perf::date::{OptionalDate, Date};
use rustc_perf::api::{data, tabular, days, stats};

lazy_static! {
    static ref INPUT_DATA: InputData = InputData::from_fs("tests/data").unwrap();
}

fn round_trip<T: Serialize + Deserialize>(value: &T) -> T {
    serde_json::from_str(&serde_json::to_string(value).unwrap()).unwrap()
}

fn from_file<P: AsRef<Path>, D: Deserialize>(path: P) -> D {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}

#[allow(dead_code)]
fn to_file<P: AsRef<Path>, S: Serialize>(path: P, value: S) {
    let mut file = File::create(path).unwrap();
    file.write_all(pretty_json(&value).as_bytes()).unwrap();
}

fn pretty_json<S: Serialize>(value: &S) -> String {
    serde_json::to_string_pretty(value).unwrap()
}

fn check_response<S>(received_value: S, expected_file: &str)
    where S: Serialize + Deserialize + PartialEq + ::std::fmt::Debug
{
    // Some types aren't equivalent after a round trip to their actual values.
    // This means we need to round trip through Serde to get saved representation on disk.
    let received_value = round_trip(&received_value);

    // Uncomment this line to refresh the expected results.
    // to_file(expected_file, &received_value);

    // The deserialized value from the file.
    let expected_value = from_file(expected_file);

    if received_value != expected_value {
        panic!("Compared {} body with server result, results not equal.", expected_file);
    }
}

#[test]
fn summary() {
    check_response(server::handle_summary(&INPUT_DATA), "tests/expected_results/summary.json");
}


#[test]
fn info() {
    check_response(server::handle_info(&INPUT_DATA), "tests/expected_results/info.json");
}

#[test]
fn data_crate_benchmarks() {
    check_response(server::handle_data(data::Request {
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    group_by: GroupBy::Crate,
                                    kind: Kind::Benchmarks,
                                    phases: vec!["total".into()],
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                }, &INPUT_DATA),
                                "tests/expected_results/data_crate_benchmarks.json");
}

#[test]
fn data_crate_rustc_total() {
    check_response(server::handle_data(data::Request {
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    group_by: GroupBy::Crate,
                                    kind: Kind::Rustc,
                                    phases: vec!["total".into()],
                                    crates: vec!["total".into()],
                                }, &INPUT_DATA),
                                "tests/expected_results/data_crate_rustc_total.json");
}

#[test]
fn tabular_rustc() {
    check_response(server::handle_tabular(tabular::Request {
                                    kind: Kind::Rustc,
                                    date: OptionalDate::CouldNotParse("".into()),
                                }, &INPUT_DATA), "tests/expected_results/tabular_rustc.json");
}

#[test]
fn tabular_benchmarks() {
    check_response(server::handle_tabular(tabular::Request {
                                    kind: Kind::Benchmarks,
                                    date: OptionalDate::CouldNotParse("".into()),
                                }, &INPUT_DATA), "tests/expected_results/tabular_benchmarks.json");
}

fn ymd_date(year: i32, month: u32, day: u32) -> OptionalDate {
    OptionalDate::Date(Date::ymd_hms(year, month, day, 0, 0, 0))
}

#[test]
fn days_benchmarks() {
    check_response(server::handle_days(days::Request {
                                    kind: Kind::Benchmarks,
                                    date_a: ymd_date(2016, 02, 21),
                                    date_b: ymd_date(2016, 03, 22),
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                    phases: vec!["total".into()],
                                    group_by: GroupBy::Crate,
                                }, &INPUT_DATA), "tests/expected_results/days_benchmarks.json");
}

#[test]
fn days_rustc() {
    check_response(server::handle_days(days::Request {
                                    kind: Kind::Rustc,
                                    date_a: ymd_date(2016, 02, 21),
                                    date_b: ymd_date(2016, 03, 22),
                                    crates: vec!["total".into()],
                                    phases: vec!["total".into()],
                                    group_by: GroupBy::Crate,
                                }, &INPUT_DATA), "tests/expected_results/days_rustc.json");
}

#[test]
fn stats_benchmarks() {
    check_response(server::handle_stats(stats::Request {
                                    kind: Kind::Benchmarks,
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                    phases: vec!["total".into()],
                                }, &INPUT_DATA), "tests/expected_results/stats_benchmarks.json");
}

#[test]
fn stats_rustc() {
    check_response(server::handle_stats(stats::Request {
                                    kind: Kind::Rustc,
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    crates: vec!["total".into()],
                                    phases: vec!["total".into()],
                                }, &INPUT_DATA), "tests/expected_results/stats_rustc.json");
}
