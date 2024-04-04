use clap::{CommandFactory, FromArgMatches};

#[derive(clap::Parser, Debug)]
pub enum Args {
    /// Benchmark all benchmarks in this benchmark group and print the results as JSON.
    Run(BenchmarkArgs),
    /// Profile a single benchmark execution.
    Profile(ProfileArgs),
    /// List benchmarks that are defined in the current group as a JSON array.
    List,
}

#[derive(clap::Parser, Debug)]
pub struct BenchmarkArgs {
    /// How many times should each benchmark be repeated.
    #[arg(long, default_value = "5")]
    pub iterations: u32,

    /// Exclude all benchmarks matching a prefix in this comma-separated list
    #[arg(long, value_delimiter = ',')]
    pub exclude: Vec<String>,

    /// Include only benchmarks matching a prefix in this comma-separated list
    #[arg(long, value_delimiter = ',')]
    pub include: Vec<String>,
}

#[derive(clap::Parser, Debug)]
pub struct ProfileArgs {
    /// Name of the benchmark that should be profiled.
    pub benchmark: String,
}

#[test]
fn verify_cli() {
    // By default, clap lazily checks subcommands. This provides eager testing
    // without having to run the binary for each subcommand.
    use clap::CommandFactory;
    Args::command().debug_assert()
}

pub fn parse_cli() -> anyhow::Result<Args> {
    let app = Args::command();

    // Set the name of the help to the current binary name
    let app = app.name(
        std::env::current_exe()?
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_owned())
            .expect("Binary name not found"),
    );

    Ok(Args::from_arg_matches(&app.get_matches())?)
}
