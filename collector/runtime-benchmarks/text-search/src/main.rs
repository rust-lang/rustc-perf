use regex::Regex;

use benchlib::benchmark::run_benchmark_group;

const TEXT_SHERLOCK: &str = include_str!("data/sherlock.txt");

fn main() {
    let regex1 = Regex::new(r"[a-zA-Z]+ing").unwrap();
    let regex2 = Regex::new(r"(Sherlock|Holmes|Watson|Irene|Adler|John|Baker)").unwrap();

    run_benchmark_group(|group| {
        group.register_benchmark("regex-search-1", || {
            || regex1.find_iter(TEXT_SHERLOCK).count()
        });
        group.register_benchmark("regex-capture-1", || {
            || regex2.captures_iter(TEXT_SHERLOCK).count()
        });
    });
}
