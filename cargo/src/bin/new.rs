use std::env;

use cargo::ops;
use cargo::util::{CliResult, Config};

#[derive(Deserialize)]
pub struct Options {
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_bin: bool,
    flag_lib: bool,
    arg_path: String,
    flag_name: Option<String>,
    flag_vcs: Option<ops::VersionControl>,
    flag_frozen: bool,
    flag_locked: bool,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

pub const USAGE: &'static str = "
Create a new cargo package at <path>

Usage:
    cargo new [options] <path>
    cargo new -h | --help

Options:
    -h, --help          Print this message
    --vcs VCS           Initialize a new repository for the given version
                        control system (git, hg, pijul, or fossil) or do not
                        initialize any version control at all (none), overriding
                        a global configuration.
    --bin               Use a binary (application) template
    --lib               Use a library template [default]
    --name NAME         Set the resulting package name, defaults to the value of <path>
    -v, --verbose ...   Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet         No output printed to stdout
    --color WHEN        Coloring: auto, always, never
    --frozen            Require Cargo.lock and cache are up to date
    --locked            Require Cargo.lock is up to date
    -Z FLAG ...         Unstable (nightly-only) flags to Cargo
";

pub fn execute(options: Options, config: &mut Config) -> CliResult {
    debug!("executing; cmd=cargo-new; args={:?}", env::args().collect::<Vec<_>>());
    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked,
                     &options.flag_z)?;

    let Options { flag_bin, flag_lib, arg_path, flag_name, flag_vcs, .. } = options;

    let opts = ops::NewOptions::new(flag_vcs,
                                    flag_bin,
                                    flag_lib,
                                    &arg_path,
                                    flag_name.as_ref().map(|s| s.as_ref()));

    let opts_lib = opts.lib;
    ops::new(&opts, config)?;

    config.shell().status("Created", format!("{} `{}` project",
                                             if opts_lib { "library" }
                                             else {"binary (application)"},
                                             arg_path))?;

    Ok(())
}

