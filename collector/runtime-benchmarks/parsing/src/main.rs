use crate::json::parse_json;
use benchlib::benchmark::run_benchmark_group;

mod json;

// JSON describing GitHub events, taken from https://api.github.com/events.
const GITHUB_EVENTS: &str = include_str!("../data/github-events.json");

fn main() {
    // Inflate the data to make the benchmark run for longer
    let github_events = format!(
        "[{}]",
        std::iter::repeat(GITHUB_EVENTS)
            .take(100)
            .collect::<Vec<_>>()
            .join(",")
    );

    run_benchmark_group(|group| {
        group.register_benchmark("nom-json", || || parse_json(&github_events).unwrap());
    });
}
