// This test is currently out-of-date. Disabled. See issue #165.
#![cfg(any())]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pretty_assertions;
extern crate rustc_perf;
extern crate serde;
extern crate serde_json;

use std::cmp::PartialEq;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use rustc_perf::api::{data, days, stats, tabular};
use rustc_perf::date::{Bound, Date, OptionalDate};
use rustc_perf::load::InputData;
use rustc_perf::server::{self, GroupBy};

lazy_static! {
    static ref INPUT_DATA: InputData = InputData::from_fs("tests/data").unwrap();
}

fn round_trip<T: Serialize + DeserializeOwned + ::std::fmt::Debug>(value: &T) -> T {
    let serialized = serde_json::to_string_pretty(value).unwrap();
    serde_json::from_str(&serialized).unwrap()
}

fn from_file<P: AsRef<Path>, D: DeserializeOwned>(path: P) -> D {
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
where
    S: Serialize + DeserializeOwned + PartialEq + ::std::fmt::Debug,
{
    // Some types aren't equivalent after a round trip to their actual values.
    // This means we need to round trip through Serde to get saved representation on disk.
    let received_value = round_trip(&received_value);

    // Uncomment this line to refresh the expected results.
    // to_file(expected_file, &received_value);

    // The deserialized value from the file.
    let expected_value = from_file(expected_file);

    if received_value != expected_value {
        assert_eq!(
            expected_value, received_value,
            "Compared {} body with server result, results not equal.",
            expected_file
        );
    }
}

#[test]
fn summary() {
    check_response(
        server::handle_summary(&INPUT_DATA),
        "tests/expected_results/summary.json",
    );
}

#[test]
fn info() {
    check_response(
        server::handle_info(&INPUT_DATA),
        "tests/expected_results/info.json",
    );
}

#[test]
fn data_phase() {
    check_response(
        server::handle_data(
            data::Request {
                start_date: OptionalDate::CouldNotParse("".into()),
                end_date: OptionalDate::CouldNotParse("".into()),
                group_by: GroupBy::Phase,
                phases: vec!["total".into()].into(),
                crates: vec!["helloworld".into(), "regex.0.1.30".into()].into(),
            },
            &INPUT_DATA,
        ),
        "tests/expected_results/data_phase.json",
    );
}

#[test]
fn data_crate() {
    check_response(
        server::handle_data(
            data::Request {
                start_date: OptionalDate::CouldNotParse("".into()),
                end_date: OptionalDate::CouldNotParse("".into()),
                group_by: GroupBy::Crate,
                phases: vec!["total".into()].into(),
                crates: vec!["helloworld".into(), "regex.0.1.30".into()].into(),
            },
            &INPUT_DATA,
        ),
        "tests/expected_results/data_crate_benchmarks.json",
    );
}

#[test]
fn tabular() {
    check_response(
        server::handle_tabular(
            tabular::Request {
                date: OptionalDate::CouldNotParse("".into()),
            },
            &INPUT_DATA,
        ),
        "tests/expected_results/tabular.json",
    );
}

fn ymd_date<B: Bound>(year: i32, month: u32, day: u32) -> OptionalDate<B> {
    OptionalDate::Date(
        Date::ymd_hms(year, month, day, 0, 0, 0),
        std::marker::PhantomData,
    )
}

#[test]
fn days() {
    check_response(
        server::handle_days(
            days::Request {
                date_a: ymd_date(2016, 02, 21),
                date_b: ymd_date(2016, 03, 22),
                crates: vec!["helloworld".into(), "regex.0.1.30".into()].into(),
                phases: vec!["total".into()].into(),
                group_by: GroupBy::Crate,
            },
            &INPUT_DATA,
        ),
        "tests/expected_results/days.json",
    );
}

#[test]
fn stats() {
    check_response(
        server::handle_stats(
            stats::Request {
                start_date: OptionalDate::CouldNotParse("".into()),
                end_date: OptionalDate::CouldNotParse("".into()),
                crates: vec!["helloworld".into(), "regex.0.1.30".into()].into(),
                phases: vec!["total".into()].into(),
                group_by: GroupBy::Crate,
            },
            &INPUT_DATA,
        ),
        "tests/expected_results/stats.json",
    );
}
