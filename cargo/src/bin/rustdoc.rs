use cargo::core::Workspace;
use cargo::ops::{self, MessageFormat, Packages};
use cargo::util::{CliResult, Config};
use cargo::util::important_paths::{find_root_manifest_for_wd};

#[derive(Deserialize)]
pub struct Options {
    arg_opts: Vec<String>,
    flag_target: Option<String>,
    flag_features: Vec<String>,
    flag_all_features: bool,
    flag_jobs: Option<u32>,
    flag_manifest_path: Option<String>,
    flag_no_default_features: bool,
    flag_open: bool,
    flag_verbose: u32,
    flag_release: bool,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_message_format: MessageFormat,
    flag_package: Option<String>,
    flag_lib: bool,
    flag_bin: Vec<String>,
    flag_bins: bool,
    flag_example: Vec<String>,
    flag_examples: bool,
    flag_test: Vec<String>,
    flag_tests: bool,
    flag_bench: Vec<String>,
    flag_benches: bool,
    flag_all_targets: bool,
    flag_frozen: bool,
    flag_locked: bool,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

pub const USAGE: &'static str = "
Build a package's documentation, using specified custom flags.

Usage:
    cargo rustdoc [options] [--] [<opts>...]

Options:
    -h, --help               Print this message
    --open                   Opens the docs in a browser after the operation
    -p SPEC, --package SPEC  Package to document
    -j N, --jobs N           Number of parallel jobs, defaults to # of CPUs
    --lib                    Build only this package's library
    --bin NAME               Build only the specified binary
    --bins                   Build all binaries
    --example NAME           Build only the specified example
    --examples               Build all examples
    --test NAME              Build only the specified test target
    --tests                  Build all tests
    --bench NAME             Build only the specified bench target
    --benches                Build all benches
    --all-targets            Build all targets (default)
    --release                Build artifacts in release mode, with optimizations
    --features FEATURES      Space-separated list of features to also build
    --all-features           Build all available features
    --no-default-features    Do not build the `default` feature
    --target TRIPLE          Build for the target triple
    --manifest-path PATH     Path to the manifest to document
    -v, --verbose ...        Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
    --message-format FMT     Error format: human, json [default: human]
    --frozen                 Require Cargo.lock and cache are up to date
    --locked                 Require Cargo.lock is up to date
    -Z FLAG ...              Unstable (nightly-only) flags to Cargo

The specified target for the current package (or package specified by SPEC if
provided) will be documented with the specified <opts>... being passed to the
final rustdoc invocation. Dependencies will not be documented as part of this
command.  Note that rustdoc will still unconditionally receive arguments such
as -L, --extern, and --crate-type, and the specified <opts>...  will simply be
added to the rustdoc invocation.

If the --package argument is given, then SPEC is a package id specification
which indicates which package should be documented. If it is not given, then the
current package is documented. For more information on SPEC and its format, see
the `cargo help pkgid` command.
";

pub fn execute(options: Options, config: &mut Config) -> CliResult {
    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked,
                     &options.flag_z)?;

    let root = find_root_manifest_for_wd(options.flag_manifest_path,
                                         config.cwd())?;

    let spec = options.flag_package.map_or_else(Vec::new, |s| vec![s]);

    let doc_opts = ops::DocOptions {
        open_result: options.flag_open,
        compile_opts: ops::CompileOptions {
            config: config,
            jobs: options.flag_jobs,
            target: options.flag_target.as_ref().map(|t| &t[..]),
            features: &options.flag_features,
            all_features: options.flag_all_features,
            no_default_features: options.flag_no_default_features,
            spec: Packages::Packages(&spec),
            release: options.flag_release,
            filter: ops::CompileFilter::new(options.flag_lib,
                                            &options.flag_bin, options.flag_bins,
                                            &options.flag_test, options.flag_tests,
                                            &options.flag_example, options.flag_examples,
                                            &options.flag_bench, options.flag_benches,
                                            options.flag_all_targets),
            message_format: options.flag_message_format,
            mode: ops::CompileMode::Doc { deps: false },
            target_rustdoc_args: Some(&options.arg_opts),
            target_rustc_args: None,
        },
    };

    let ws = Workspace::new(&root, config)?;
    ops::doc(&ws, &doc_opts)?;

    Ok(())
}
