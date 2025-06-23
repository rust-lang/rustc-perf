use std::iter::FromIterator;

use cargo::core::Workspace;
use cargo::ops::{self, MessageFormat, Packages};
use cargo::util::{CliResult, CliError, Config, CargoErrorKind};
use cargo::util::important_paths::{find_root_manifest_for_wd};

#[derive(Deserialize)]
pub struct Options {
    flag_bin: Option<String>,
    flag_example: Option<String>,
    flag_package: Option<String>,
    flag_jobs: Option<u32>,
    flag_features: Vec<String>,
    flag_all_features: bool,
    flag_no_default_features: bool,
    flag_target: Option<String>,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_message_format: MessageFormat,
    flag_release: bool,
    flag_frozen: bool,
    flag_locked: bool,
    arg_args: Vec<String>,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

pub const USAGE: &'static str = "
Run the main binary of the local package (src/main.rs)

Usage:
    cargo run [options] [--] [<args>...]

Options:
    -h, --help                   Print this message
    --bin NAME                   Name of the bin target to run
    --example NAME               Name of the example target to run
    -p SPEC, --package SPEC      Package with the target to run
    -j N, --jobs N               Number of parallel jobs, defaults to # of CPUs
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --target TRIPLE              Build for the target triple
    --manifest-path PATH         Path to the manifest to execute
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date
    -Z FLAG ...                  Unstable (nightly-only) flags to Cargo

If neither `--bin` nor `--example` are given, then if the project only has one
bin target it will be run. Otherwise `--bin` specifies the bin target to run,
and `--example` specifies the example target to run. At most one of `--bin` or
`--example` can be provided.

All of the trailing arguments are passed to the binary to run. If you're passing
arguments to both Cargo and the binary, the ones after `--` go to the binary,
the ones before go to Cargo.
";

pub fn execute(options: Options, config: &mut Config) -> CliResult {
    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked,
                     &options.flag_z)?;

    let root = find_root_manifest_for_wd(options.flag_manifest_path, config.cwd())?;

    let (mut examples, mut bins) = (Vec::new(), Vec::new());
    if let Some(s) = options.flag_bin {
        bins.push(s);
    }
    if let Some(s) = options.flag_example {
        examples.push(s);
    }

    let packages = Vec::from_iter(options.flag_package.iter().cloned());
    let spec = Packages::Packages(&packages);

    let compile_opts = ops::CompileOptions {
        config: config,
        jobs: options.flag_jobs,
        target: options.flag_target.as_ref().map(|t| &t[..]),
        features: &options.flag_features,
        all_features: options.flag_all_features,
        no_default_features: options.flag_no_default_features,
        spec: spec,
        release: options.flag_release,
        mode: ops::CompileMode::Build,
        filter: if examples.is_empty() && bins.is_empty() {
            ops::CompileFilter::Default { required_features_filterable: false, }
        } else {
            ops::CompileFilter::new(false,
                                    &bins, false,
                                    &[], false,
                                    &examples, false,
                                    &[], false,
                                    false)
        },
        message_format: options.flag_message_format,
        target_rustdoc_args: None,
        target_rustc_args: None,
    };

    let ws = Workspace::new(&root, config)?;
    match ops::run(&ws, &compile_opts, &options.arg_args)? {
        None => Ok(()),
        Some(err) => {
            // If we never actually spawned the process then that sounds pretty
            // bad and we always want to forward that up.
            let exit = match err.exit {
                Some(exit) => exit,
                None => return Err(
                    CliError::new(CargoErrorKind::ProcessErrorKind(err).into(), 101)),
            };

            // If `-q` was passed then we suppress extra error information about
            // a failed process, we assume the process itself printed out enough
            // information about why it failed so we don't do so as well
            let exit_code = exit.code().unwrap_or(101);
            Err(if options.flag_quiet == Some(true) {
                CliError::code(exit_code)
            } else {
                CliError::new(CargoErrorKind::ProcessErrorKind(err).into(), exit_code)
            })
        }
    }
}
