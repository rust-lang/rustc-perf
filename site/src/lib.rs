use thiserror::Error;

#[macro_use]
extern crate itertools;

#[derive(Error, Debug)]
#[error("command failed: {command}")]
pub struct CommandFailed {
    command: String,
}

pub mod api;
pub mod db;
pub mod github;
pub mod load;
pub mod server;

mod average;
mod comparison;
mod interpolate;
mod request_handlers;
mod selector;
mod self_profile;
