use std::env;

use cargo::core::Workspace;
use cargo::ops::{self, MessageFormat, Packages};
use cargo::util::{CliResult, CliError, Config, CargoErrorKind};
use cargo::util::important_paths::find_root_manifest_for_wd;

#[derive(Deserialize)]
pub struct Options {
    arg_args: Vec<String>,
    flag_features: Vec<String>,
    flag_all_features: bool,
    flag_jobs: Option<u32>,
    flag_manifest_path: Option<String>,
    flag_no_default_features: bool,
    flag_no_run: bool,
    flag_package: Vec<String>,
    flag_target: Option<String>,
    flag_lib: bool,
    flag_doc: bool,
    flag_bin: Vec<String>,
    flag_bins: bool,
    flag_example: Vec<String>,
    flag_examples: bool,
    flag_test: Vec<String>,
    flag_tests: bool,
    flag_bench: Vec<String>,
    flag_benches: bool,
    flag_all_targets: bool,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_message_format: MessageFormat,
    flag_release: bool,
    flag_no_fail_fast: bool,
    flag_frozen: bool,
    flag_locked: bool,
    flag_all: bool,
    flag_exclude: Vec<String>,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

pub const USAGE: &'static str = "
Execute all unit and integration tests of a local package

Usage:
    cargo test [options] [--] [<args>...]

Options:
    -h, --help                   Print this message
    --lib                        Test only this package's library
    --doc                        Test only this library's documentation
    --bin NAME ...               Test only the specified binary
    --bins                       Test all binaries
    --example NAME ...           Check that the specified examples compile
    --examples                   Check that all examples compile
    --test NAME ...              Test only the specified test target
    --tests                      Test all tests
    --bench NAME ...             Test only the specified bench target
    --benches                    Test all benches
    --all-targets                Test all targets (default)
    --no-run                     Compile, but don't run tests
    -p SPEC, --package SPEC ...  Package to run tests for
    --all                        Test all packages in the workspace
    --exclude SPEC ...           Exclude packages from the test
    -j N, --jobs N               Number of parallel builds, see below for details
    --release                    Build artifacts in release mode, with optimizations
    --features FEATURES          Space-separated list of features to also build
    --all-features               Build all available features
    --no-default-features        Do not build the `default` feature
    --target TRIPLE              Build for the target triple
    --manifest-path PATH         Path to the manifest to build tests for
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --message-format FMT         Error format: human, json [default: human]
    --no-fail-fast               Run all tests regardless of failure
    --frozen                     Require Cargo.lock and cache are up to date
    --locked                     Require Cargo.lock is up to date
    -Z FLAG ...                  Unstable (nightly-only) flags to Cargo

All of the trailing arguments are passed to the test binaries generated for
filtering tests and generally providing options configuring how they run. For
example, this will run all tests with the name `foo` in their name:

    cargo test foo

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be tested. If it is not given, then the
current package is tested. For more information on SPEC and its format, see the
`cargo help pkgid` command.

All packages in the workspace are tested if the `--all` flag is supplied. The
`--all` flag is automatically assumed for a virtual manifest.
Note that `--exclude` has to be specified in conjunction with the `--all` flag.

The --jobs argument affects the building of the test executable but does
not affect how many jobs are used when running the tests. The default value
for the --jobs argument is the number of CPUs. If you want to control the
number of simultaneous running test cases, pass the `--test-threads` option
to the test binaries:

  cargo test -- --test-threads=1

Compilation can be configured via the `test` profile in the manifest.

By default the rust test harness hides output from test execution to
keep results readable. Test output can be recovered (e.g. for debugging)
by passing `--nocapture` to the test binaries:

  cargo test -- --nocapture

To get the list of all options available for the test binaries use this:

  cargo test -- --help
";

pub fn execute(options: Options, config: &mut Config) -> CliResult {
    debug!("executing; cmd=cargo-test; args={:?}",
           env::args().collect::<Vec<_>>());

    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked,
                     &options.flag_z)?;

    let root = find_root_manifest_for_wd(options.flag_manifest_path, config.cwd())?;
    let ws = Workspace::new(&root, config)?;

    let empty = Vec::new();
    let (mode, filter);
    if options.flag_doc {
        mode = ops::CompileMode::Doctest;
        filter = ops::CompileFilter::new(true, &empty, false, &empty, false,
                                               &empty, false, &empty, false,
                                         false);
    } else {
        mode = ops::CompileMode::Test;
        filter = ops::CompileFilter::new(options.flag_lib,
                                         &options.flag_bin, options.flag_bins,
                                         &options.flag_test, options.flag_tests,
                                         &options.flag_example, options.flag_examples,
                                         &options.flag_bench, options.flag_benches,
                                         options.flag_all_targets);
    }

    let spec = Packages::from_flags(ws.is_virtual(),
                                    options.flag_all,
                                    &options.flag_exclude,
                                    &options.flag_package)?;

    let ops = ops::TestOptions {
        no_run: options.flag_no_run,
        no_fail_fast: options.flag_no_fail_fast,
        only_doc: options.flag_doc,
        compile_opts: ops::CompileOptions {
            config: config,
            jobs: options.flag_jobs,
            target: options.flag_target.as_ref().map(|s| &s[..]),
            features: &options.flag_features,
            all_features: options.flag_all_features,
            no_default_features: options.flag_no_default_features,
            spec: spec,
            release: options.flag_release,
            mode: mode,
            filter: filter,
            message_format: options.flag_message_format,
            target_rustdoc_args: None,
            target_rustc_args: None,
        },
    };

    let err = ops::run_tests(&ws, &ops, &options.arg_args)?;
    match err {
        None => Ok(()),
        Some(err) => {
            Err(match err.exit.as_ref().and_then(|e| e.code()) {
                Some(i) => CliError::new(err.hint().into(), i),
                None => CliError::new(CargoErrorKind::CargoTestErrorKind(err).into(), 101),
            })
        }
    }
}
