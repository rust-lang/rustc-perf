#[macro_use]
extern crate itertools;

pub mod api;
pub mod github;
pub mod load;
pub mod server;

mod average;
mod benchmark_metadata;
mod comparison;
mod request_handlers;
mod resources;
mod selector;
mod self_profile;
