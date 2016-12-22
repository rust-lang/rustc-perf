#[macro_use]
extern crate lazy_static;
extern crate rustc_perf;
extern crate serde_json;
extern crate iron_test;
extern crate serde;
extern crate iron;

use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;

use iron::prelude::*;
use iron::status::Status;
use iron::{Headers, headers};
use iron::response::ResponseBody;
use serde_json::Value;
use serde::Serialize;

use rustc_perf::server::{self, GroupBy};
use rustc_perf::load::{Kind, InputData};
use rustc_perf::date::{OptionalDate, Date};
use rustc_perf::api::{data, tabular, days, stats};

lazy_static! {
    static ref CHAIN: Chain = server::chain(InputData::from_fs("tests/data").unwrap());
}

fn request(path: &str) -> Response {
    iron_test::request::get(&format!("http://perf.rust-lang.org/{}", path),
                            Headers::new(),
                            &*CHAIN)
        .unwrap()
}

fn post_request<T: Serialize>(path: &str, body: T) -> Response {
    iron_test::request::post(&format!("http://perf.rust-lang.org/{}", path),
                             Headers::new(),
                             &serde_json::to_string(&body).unwrap(),
                             &*CHAIN)
        .unwrap()
}

fn response_body(response: Response) -> Value {
    if let Some(mut body) = response.body {
        let mut buf = Vec::new();
        body.write_body(&mut ResponseBody::new(&mut buf)).unwrap();
        let s = String::from_utf8(buf).unwrap();
        serde_json::from_str(&s).unwrap()
    } else {
        Value::Null
    }
}

fn round_trip_value(v: &Value) -> Value {
    let s = pretty_json(v);
    serde_json::from_str(&s).unwrap()
}

fn from_file<P: AsRef<Path>>(path: P) -> Value {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    serde_json::from_str(&s).unwrap()
}

#[allow(dead_code)]
fn to_file<P: AsRef<Path>>(path: P, value: &Value) {
    let mut file = File::create(path).unwrap();
    file.write_all(pretty_json(&value).as_bytes()).unwrap();
}

fn pretty_json(value: &Value) -> String {
    serde_json::to_string_pretty(&value).unwrap()
}

fn check_response(response: Response, expected_file: &str) {
    assert_eq!(response.status, Some(Status::Ok));
    assert_eq!(response.headers.get(),
               Some(&headers::AccessControlAllowOrigin::Any));
    assert_eq!(response.headers.get(),
               Some(&headers::ContentType("application/json".parse().unwrap())));

    // The deserialized value from wire.
    // If new values are to be recorded to the files, this is the value that
    // should be recorded.
    // This has undergone: server serialize -> tests deserialize
    let raw_received_value = response_body(response);

    // Uncomment this line to refresh the expected results.
    // to_file(expected_file, &raw_received_value);

    // The deserialized value from the file.
    // This has undergone:
    //     server serialize -> tests deserialize ->
    //     to_file serialize (pretty) -> tests deserialize
    let expected_value = from_file(expected_file);

    // This value has now undergone the same amount of round trips through
    // Serde as the one in the file.
    let received_value = round_trip_value(&raw_received_value);

    if received_value != expected_value {
        // Note: This value's float fields should not be used for direct
        // comparison with the expected results, due to the loss of precision in Serde
        println!("{}", serde_json::to_string(&raw_received_value).unwrap());
        panic!("Compared {} body with server result, results not equal.",
               expected_file);
    }
}

#[test]
fn summary() {
    let response = request("/summary");
    check_response(response, "tests/expected_results/summary.json");
}

#[test]
fn info() {
    let response = request("/info");
    check_response(response, "tests/expected_results/info.json");
}

#[test]
fn data_crate_benchmarks() {
    let response = post_request("/data",
                                data::Request {
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    group_by: GroupBy::Crate,
                                    kind: Kind::Benchmarks,
                                    phases: vec!["total".into()],
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                });
    check_response(response,
                   "tests/expected_results/data_crate_benchmarks.json");
}

#[test]
fn data_crate_rustc_total() {
    let response = post_request("/data",
                                data::Request {
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    group_by: GroupBy::Crate,
                                    kind: Kind::Rustc,
                                    phases: vec!["total".into()],
                                    crates: vec!["total".into()],
                                });
    check_response(response,
                   "tests/expected_results/data_crate_rustc_total.json");
}

#[test]
fn tabular_rustc() {
    let response = post_request("/get_tabular",
                                tabular::Request {
                                    kind: Kind::Rustc,
                                    date: OptionalDate::CouldNotParse("".into()),
                                });
    check_response(response, "tests/expected_results/tabular_rustc.json");
}

#[test]
fn tabular_benchmarks() {
    let response = post_request("/get_tabular",
                                tabular::Request {
                                    kind: Kind::Benchmarks,
                                    date: OptionalDate::CouldNotParse("".into()),
                                });
    check_response(response, "tests/expected_results/tabular_benchmarks.json");
}

fn ymd_date(year: i32, month: u32, day: u32) -> OptionalDate {
    OptionalDate::Date(Date::ymd_hms(year, month, day, 0, 0, 0))
}

#[test]
fn days_benchmarks() {
    let response = post_request("/get",
                                days::Request {
                                    kind: Kind::Benchmarks,
                                    date_a: ymd_date(2016, 02, 21),
                                    date_b: ymd_date(2016, 03, 22),
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                    phases: vec!["total".into()],
                                    group_by: GroupBy::Crate,
                                });
    check_response(response, "tests/expected_results/days_benchmarks.json");
}

#[test]
fn days_rustc() {
    let response = post_request("/get",
                                days::Request {
                                    kind: Kind::Rustc,
                                    date_a: ymd_date(2016, 02, 21),
                                    date_b: ymd_date(2016, 03, 22),
                                    crates: vec!["total".into()],
                                    phases: vec!["total".into()],
                                    group_by: GroupBy::Crate,
                                });
    check_response(response, "tests/expected_results/days_rustc.json");
}

#[test]
fn stats_benchmarks() {
    let response = post_request("/stats",
                                stats::Request {
                                    kind: Kind::Benchmarks,
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                    phases: vec!["total".into()],
                                });
    check_response(response, "tests/expected_results/stats_benchmarks.json");
}

#[test]
fn stats_rustc() {
    let response = post_request("/stats",
                                stats::Request {
                                    kind: Kind::Rustc,
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    crates: vec!["total".into()],
                                    phases: vec!["total".into()],
                                });
    check_response(response, "tests/expected_results/stats_rustc.json");
}
