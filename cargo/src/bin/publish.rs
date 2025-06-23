use cargo::core::Workspace;
use cargo::ops;
use cargo::util::{CliResult, Config};
use cargo::util::important_paths::find_root_manifest_for_wd;

#[derive(Deserialize)]
pub struct Options {
    flag_index: Option<String>,
    flag_host: Option<String>,  // TODO: Deprecated, remove
    flag_token: Option<String>,
    flag_target: Option<String>,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_no_verify: bool,
    flag_allow_dirty: bool,
    flag_jobs: Option<u32>,
    flag_dry_run: bool,
    flag_frozen: bool,
    flag_locked: bool,
    #[serde(rename = "flag_Z")]
    flag_z: Vec<String>,
}

pub const USAGE: &'static str = "
Upload a package to the registry

Usage:
    cargo publish [options]

Options:
    -h, --help               Print this message
    --index INDEX            Registry index to upload the package to
    --host HOST              DEPRECATED, renamed to '--index'
    --token TOKEN            Token to use when uploading
    --no-verify              Don't verify package tarball before publish
    --allow-dirty            Allow publishing with a dirty source directory
    --target TRIPLE          Build for the target triple
    --manifest-path PATH     Path to the manifest of the package to publish
    -j N, --jobs N           Number of parallel jobs, defaults to # of CPUs
    --dry-run                Perform all checks without uploading
    -v, --verbose ...        Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never
    --frozen                 Require Cargo.lock and cache are up to date
    --locked                 Require Cargo.lock is up to date
    -Z FLAG ...              Unstable (nightly-only) flags to Cargo

";

pub fn execute(options: Options, config: &mut Config) -> CliResult {
    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked,
                     &options.flag_z)?;

    let Options {
        flag_token: token,
        flag_index: index,
        flag_host: host,    // TODO: Deprecated, remove
        flag_manifest_path,
        flag_no_verify: no_verify,
        flag_allow_dirty: allow_dirty,
        flag_jobs: jobs,
        flag_dry_run: dry_run,
        flag_target: target,
        ..
    } = options;


    // TODO: Deprecated
    // remove once it has been decided --host can be removed
    // We may instead want to repurpose the host flag, as
    // mentioned in this issue
    // https://github.com/rust-lang/cargo/issues/4208
    let msg = "The flag '--host' is no longer valid.

Previous versions of Cargo accepted this flag, but it is being
deprecated. The flag is being renamed to 'index', as the flag
wants the location of the index to which to publish. Please
use '--index' instead.

This will soon become a hard error, so it's either recommended
to update to a fixed version or contact the upstream maintainer
about this warning.";

    let root = find_root_manifest_for_wd(flag_manifest_path.clone(), config.cwd())?;
    let ws = Workspace::new(&root, config)?;
    ops::publish(&ws, &ops::PublishOpts {
        config: config,
        token: token,
        index:
            if host.clone().is_none() || host.clone().unwrap().is_empty() { index }
            else { config.shell().warn(&msg)?; host },  // TODO: Deprecated, remove
        verify: !no_verify,
        allow_dirty: allow_dirty,
        target: target.as_ref().map(|t| &t[..]),
        jobs: jobs,
        dry_run: dry_run,
    })?;
    Ok(())
}
