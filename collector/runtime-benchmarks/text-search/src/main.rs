use regex::Regex;

use benchlib::benchmark::run_benchmark_group;
use benchlib::decompress_file;

const TEXT_SHERLOCK: &[u8] = include_bytes!("../../data/sherlock.txt.gz");

fn main() {
    let sherlock_text = String::from_utf8(decompress_file(TEXT_SHERLOCK)).expect("Invalid UTF-8");

    let regex1 = Regex::new(r"[a-zA-Z]+ing").unwrap();
    let regex2 = Regex::new(r"(Sherlock|Holmes|Watson|Irene|Adler|John|Baker)").unwrap();

    run_benchmark_group(|group| {
        group.register_benchmark("regex-search-1", || {
            || regex1.find_iter(&sherlock_text).count()
        });
        group.register_benchmark("regex-capture-1", || {
            || regex2.captures_iter(&sherlock_text).count()
        });
    });
}
