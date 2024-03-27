use benchlib::benchmark::run_benchmark_group;
use tokio::runtime::Builder;
use benchlib::decompress_file;

const TEXT_SHERLOCK: &[u8] = include_bytes!("../../data/sherlock.txt.gz");

async fn total_char_count(reader: String) -> usize{
    let mut total_char = 0;
    for line in reader.lines() {
        total_char += line_char_count(line).await;
    }
    total_char
}

async fn line_char_count(line: &str) -> usize {
    let char_count = line.chars().count();
    char_count
}

async fn async_operation() -> usize {
    let sherlock_text = String::from_utf8(decompress_file(TEXT_SHERLOCK)).expect("Invalid UTF-8");
    let total_char = total_char_count(sherlock_text).await;
    total_char
}

fn main() {
    run_benchmark_group(|group| {
        group.register_benchmark("async-word-count", || {
            // This closure should prepare data that will be needed for the benchmark (if any),
            // and then return a closure that will actually be benchmarked/profiled.
            // Create a Tokio runtime
            let rt = Builder::new_current_thread()
                .enable_all()     
                .build()
                .unwrap(); 
            move || {
                rt.block_on(async_operation())
            }
        });
    });
}


