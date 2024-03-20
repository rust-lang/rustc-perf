use benchlib::benchmark::run_benchmark_group;
use std::time::Instant;
use tokio::runtime::Runtime;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

async fn read_the_textfile(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut total_characters = 0;
    for line in reader.lines() {
        let line = line?;
        total_characters += line.len();
    }

    Ok(total_characters)
}

async fn async_operation() {
    let f1 = read_the_textfile("./collector/runtime-benchmarks/Async2/src/poem.txt").await.expect("Can't read file");
    let f2 = read_the_textfile("./collector/runtime-benchmarks/Async2/src/poem2.txt").await.expect("Can't read file");
    let total_char_count =f1+f2;
    
}

fn main() {
    run_benchmark_group(|group| {
        group.register_benchmark("Async", || {
            // This closure should prepare data that will be needed for the benchmark (if any),
            // and then return a closure that will actually be benchmarked/profiled.
            
            // Create a Tokio runtime
            let rt = Runtime::new().unwrap();
            let start_time = Instant::now();

            for _ in 0..1000 {
            rt.block_on(async_operation());
            }

            let end_time = Instant::now();

            let duration = end_time - start_time;
            move || {
                duration
            }
        });
    });
}


