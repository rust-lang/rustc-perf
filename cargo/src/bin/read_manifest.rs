use std::env;

use cargo;
use cargo::core::Package;
use cargo::util::{CliResult, Config};
use cargo::util::important_paths::{find_root_manifest_for_wd};

#[derive(Deserialize)]
pub struct Options {
    flag_manifest_path: Option<String>,
    flag_color: Option<String>,
}

pub const USAGE: &'static str = "
Deprecated, use `cargo metadata --no-deps` instead.
Print a JSON representation of a Cargo.toml manifest.

Usage:
    cargo read-manifest [options]
    cargo read-manifest -h | --help

Options:
    -h, --help               Print this message
    -v, --verbose ...        Use verbose output (-vv very verbose/build.rs output)
    --manifest-path PATH     Path to the manifest
    --color WHEN             Coloring: auto, always, never
";

pub fn execute(options: Options, config: &mut Config) -> CliResult {
    debug!("executing; cmd=cargo-read-manifest; args={:?}",
           env::args().collect::<Vec<_>>());
    config.shell().set_color_choice(options.flag_color.as_ref().map(|s| &s[..]))?;

    let root = find_root_manifest_for_wd(options.flag_manifest_path, config.cwd())?;

    let pkg = Package::for_path(&root, config)?;
    cargo::print_json(&pkg);
    Ok(())
}
