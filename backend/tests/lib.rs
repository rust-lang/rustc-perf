#[macro_use]
extern crate lazy_static;
extern crate rustc_perf;
extern crate serde_json;
#[macro_use]
extern crate difference;
extern crate iron_test;
extern crate serde;
extern crate iron;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use iron::prelude::*;
use iron::{Handler, Headers, headers};
use iron::response::ResponseBody;
use difference::Difference;
use iron::status::Status;
use serde_json::Value;
use serde::Serialize;

use rustc_perf::server::{self, GroupBy};
use rustc_perf::load::{Kind, InputData};
use rustc_perf::date::{OptionalDate, Date};
use rustc_perf::api::{data, tabular, days, stats};

fn request<H: Handler>(path: &str, handler: &H) -> IronResult<Response> {
    iron_test::request::get(&format!("http://perf.rust-lang.org/{}", path),
                            Headers::new(),
                            handler)
}

fn post_request<H: Handler, T: Serialize>(path: &str,
                                          handler: &H,
                                          body: T)
                                          -> IronResult<Response> {
    iron_test::request::post(&format!("http://perf.rust-lang.org/{}", path),
                             Headers::new(),
                             &serde_json::to_string(&body).unwrap(),
                             handler)
}

fn response_body(response: Response) -> String {
    if let Some(mut body) = response.body {
        let mut buf = Vec::new();
        body.write_body(&mut ResponseBody::new(&mut buf)).unwrap();
        String::from_utf8(buf).unwrap()
    } else {
        "".to_string()
    }
}

fn pretty_json(s: &str) -> String {
    let value: Value = serde_json::from_str(s).unwrap();
    serde_json::to_string_pretty(&value).unwrap()
}

lazy_static! {
    static ref CHAIN: Chain = server::chain(InputData::from_fs("tests/data").unwrap());
}

fn check_response(response: Response, expected_file: &str) {
    let expected_body = &from_file(expected_file);

    assert_eq!(response.status, Some(Status::Ok));
    assert_eq!(response.headers.get(),
               Some(&headers::AccessControlAllowOrigin::Any));
    assert_eq!(response.headers.get(),
               Some(&headers::ContentType("application/json".parse().unwrap())));
    let n = response_body(response);
    let body = pretty_json(&n);
    let expected = pretty_json(expected_body);

    if body != expected {
        let (_, changeset) = difference::diff(&expected, &body, "\n");

        println!("Diff:");
        for difference in changeset {
            match difference {
                Difference::Same(_) => {},
                Difference::Add(ref x) => {
                    for line in x.lines() {
                        println!("+{}", line);
                    }
                }
                Difference::Rem(ref x) => {
                    for line in x.lines() {
                        println!("-{}", line);
                    }
                }
            }
        }
        println!("");
        println!("got: {}", n);
        panic!("Bodies did not match.");
    }
}

fn from_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

#[test]
fn summary() {
    let response = request("/summary", &*CHAIN).unwrap();
    check_response(response, "tests/expected_results/summary.json");
}

#[test]
fn info() {
    let response = request("/info", &*CHAIN).unwrap();
    check_response(response, "tests/expected_results/info.json");
}

#[test]
fn data_crate_benchmarks() {
    let response = post_request("/data",
                                &*CHAIN,
                                data::Request {
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    group_by: GroupBy::Crate,
                                    kind: Kind::Benchmarks,
                                    phases: vec!["total".into()],
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                })
        .unwrap();
    check_response(response, "tests/expected_results/data_crate_benchmarks.json");
}

#[test]
fn data_crate_rustc_total() {
    let response = post_request("/data",
                                &*CHAIN,
                                data::Request {
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    group_by: GroupBy::Crate,
                                    kind: Kind::Rustc,
                                    phases: vec!["total".into()],
                                    crates: vec!["total".into()],
                                })
        .unwrap();
    check_response(response, "tests/expected_results/data_crate_rustc_total.json");
}

#[test]
fn tabular_rustc() {
    let response = post_request("/get_tabular",
                                &*CHAIN,
                                tabular::Request {
                                    kind: Kind::Rustc,
                                    date: OptionalDate::CouldNotParse("".into()),
                                })
        .unwrap();
    check_response(response, "tests/expected_results/tabular_rustc.json");
}

#[test]
fn tabular_benchmarks() {
    let response = post_request("/get_tabular",
                                &*CHAIN,
                                tabular::Request {
                                    kind: Kind::Benchmarks,
                                    date: OptionalDate::CouldNotParse("".into()),
                                })
        .unwrap();
    check_response(response, "tests/expected_results/tabular_benchmarks.json");
}

fn ymd_date(year: i32, month: u32, day: u32) -> OptionalDate {
    OptionalDate::Date(Date::ymd_hms(year, month, day, 0, 0, 0))
}

#[test]
fn days_benchmarks() {
    let response = post_request("/get",
                                &*CHAIN,
                                days::Request {
                                    kind: Kind::Benchmarks,
                                    date_a: ymd_date(2016, 02, 21),
                                    date_b: ymd_date(2016, 03, 22),
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                    phases: vec!["total".into()],
                                    group_by: GroupBy::Crate,
                                })
        .unwrap();
    check_response(response, "tests/expected_results/days_benchmarks.json");
}

#[test]
fn days_rustc() {
    let response = post_request("/get",
                                &*CHAIN,
                                days::Request {
                                    kind: Kind::Rustc,
                                    date_a: ymd_date(2016, 02, 21),
                                    date_b: ymd_date(2016, 03, 22),
                                    crates: vec!["total".into()],
                                    phases: vec!["total".into()],
                                    group_by: GroupBy::Crate,
                                })
        .unwrap();
    check_response(response, "tests/expected_results/days_rustc.json");
}

#[test]
fn stats_benchmarks() {
    let response = post_request("/stats",
                                &*CHAIN,
                                stats::Request {
                                    kind: Kind::Benchmarks,
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    crates: vec!["helloworld".into(), "regex.0.1.30".into()],
                                    phases: vec!["total".into()],
                                })
        .unwrap();
    check_response(response, "tests/expected_results/stats_benchmarks.json");
}

#[test]
fn stats_rustc() {
    let response = post_request("/stats",
                                &*CHAIN,
                                stats::Request {
                                    kind: Kind::Rustc,
                                    start_date: OptionalDate::CouldNotParse("".into()),
                                    end_date: OptionalDate::CouldNotParse("".into()),
                                    crates: vec!["total".into()],
                                    phases: vec!["total".into()],
                                })
        .unwrap();
    check_response(response, "tests/expected_results/stats_rustc.json");
}
