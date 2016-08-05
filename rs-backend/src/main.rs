#![feature(question_mark)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate json;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate persistent;

pub const SERVER_ADDRESS: &'static str = "0.0.0.0:2346";

mod util;
mod load;
mod server;
mod errors;

fn main() {
    let data = load::InputData::from_fs().unwrap();
    server::start(data);
}
