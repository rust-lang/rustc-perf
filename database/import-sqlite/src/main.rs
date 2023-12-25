use database::{CompileBenchmark, Pool};
use hashbrown::HashMap;
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let sqlite = std::env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("needs sqlite database as 1st argument"));
    let postgres = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("needs postgres database as 2nd argument"));

    let sqlite = Pool::open(&sqlite);
    let mut sqlite_conn = sqlite.connection().await;
    let postgres = Pool::open(&postgres);
    let postgres_conn = postgres.connection().await;

    let sqlite_idx = sqlite_conn.load_index().await;

    let cid_name = format!("imported-{}", chrono::Utc::now().timestamp());
    println!("Collection ID for import is {}", cid_name);
    let cid = postgres_conn.collection_id(&cid_name).await;

    let mut benchmarks = HashSet::new();
    let benchmark_data: HashMap<String, CompileBenchmark> = sqlite_conn
        .get_compile_benchmarks()
        .await
        .into_iter()
        .map(|benchmark| (benchmark.name.clone(), benchmark))
        .collect();

    // Starting after the sqlite and postgres db args, the rest are artifact
    // names to import.
    for artifact in std::env::args().skip(3) {
        let aid = sqlite_conn
            .artifact_by_name(&artifact)
            .await
            .unwrap_or_else(|| {
                panic!("{} not found in sqlite db", artifact);
            });

        let sqlite_aid = sqlite_conn.artifact_id(&aid).await;
        let postgres_aid = postgres_conn.artifact_id(&aid).await;

        for (&(benchmark, profile, scenario, backend, metric), id) in
            sqlite_idx.compile_statistic_descriptions()
        {
            if benchmarks.insert(benchmark) {
                postgres_conn
                    .record_compile_benchmark(
                        benchmark.as_str(),
                        None,
                        benchmark_data[benchmark.as_str()].category.clone(),
                    )
                    .await;
            }

            let stat = sqlite_conn
                .get_pstats(&[id], &[Some(sqlite_aid)])
                .await
                .pop()
                .unwrap()
                .pop()
                .unwrap();
            if let Some(stat) = stat {
                postgres_conn
                    .record_statistic(
                        cid,
                        postgres_aid,
                        &benchmark,
                        profile,
                        scenario,
                        backend,
                        metric.as_str(),
                        stat,
                    )
                    .await;
            }
        }

        for (&(benchmark, metric), id) in sqlite_idx.runtime_statistic_descriptions() {
            let stat = sqlite_conn
                .get_runtime_pstats(&[id], &[Some(sqlite_aid)])
                .await
                .pop()
                .unwrap()
                .pop()
                .unwrap();
            if let Some(stat) = stat {
                postgres_conn
                    .record_runtime_statistic(cid, postgres_aid, &benchmark, metric.as_str(), stat)
                    .await;
            }
        }
    }
}
