#![feature(question_mark, custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate persistent;
extern crate serde;
extern crate serde_json;

pub const SERVER_ADDRESS: &'static str = "0.0.0.0:2346";

mod util;
mod load;
mod server;
mod errors;

fn main() {
    let data = load::InputData::from_fs().unwrap();
    server::start(data);
}
