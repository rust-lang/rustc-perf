use benchlib::benchmark::run_benchmark_group;
use tokio::runtime::Builder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use flate2::read::GzDecoder;

async fn total_char_count(reader: BufReader<GzDecoder<File>>,x: &mut usize) {
    for line in reader.lines() {
        *x += line_char_count(line.expect("invalid character")).await;
    }
}

async fn line_char_count(line: String) -> usize {
    let line_count = line.chars().count();
    line_count
}

async fn async_operation() -> usize {
    let reader2 = BufReader::new(GzDecoder::new(File::open("./collector/runtime-benchmarks/data/sherlock.txt.gz").expect("can't read a file")));
    let mut total_char = 0;
    total_char_count(reader2, &mut total_char).await;
    total_char
}

fn main() {
    run_benchmark_group(|group| {
        group.register_benchmark("Async", || {
            // This closure should prepare data that will be needed for the benchmark (if any),
            // and then return a closure that will actually be benchmarked/profiled.
            // Create a Tokio runtime
            let rt = Builder::new_multi_thread()
                .worker_threads(1) 
                .enable_all()     
                .build()
                .unwrap();
            move || {
                rt.block_on(async_operation());
            }
        });
    });
}


