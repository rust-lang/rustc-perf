use regex::Regex;

use benchlib::benchmark::run_benchmark_group;

const TEXT_SHERLOCK: &str = include_str!("data/sherlock.txt");

fn main() {
    run_benchmark_group(|group| {
        group.register_benchmark("regex-search-1", || {
            let regex = Regex::new(r"[a-zA-Z]+ing").unwrap();
            move || regex.find_iter(TEXT_SHERLOCK).count()
        });
        group.register_benchmark("regex-capture-1", || {
            let regex = Regex::new(r"(Sherlock|Holmes|Watson|Irene|Adler|John|Baker)").unwrap();
            move || regex.captures_iter(TEXT_SHERLOCK).count()
        });
    });
}
