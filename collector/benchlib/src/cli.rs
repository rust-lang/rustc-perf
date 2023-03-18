use clap::{CommandFactory, FromArgMatches};

#[derive(clap::Parser, Debug)]
pub enum Args {
    /// Benchmark all benchmarks in this benchmark group and print the results as JSON.
    Run(BenchmarkArgs),
    /// List benchmarks that are defined in the current group as a JSON array.
    List,
}

#[derive(clap::Parser, Debug)]
pub struct BenchmarkArgs {
    /// How many times should each benchmark be repeated.
    #[clap(long, default_value = "5")]
    pub iterations: u32,

    /// Exclude all benchmarks matching a prefix in this comma-separated list
    #[clap(long)]
    pub exclude: Option<String>,

    /// Include only benchmarks matching a prefix in this comma-separated list
    #[clap(long)]
    pub include: Option<String>,
}

pub fn parse_cli() -> anyhow::Result<Args> {
    let app = Args::command();

    // Set the name of the help to the current binary name
    let app = app.name(
        std::env::current_exe()?
            .file_name()
            .and_then(|s| s.to_str())
            .expect("Binary name not found"),
    );

    Ok(Args::from_arg_matches(&app.get_matches())?)
}
