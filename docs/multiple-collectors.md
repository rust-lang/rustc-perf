# rustc-perf - Multiple Collectors Documentation

rustc-perf has been enhanced to support parallel benchmarking and execution across various architectures. While this enables a distributed architecture in deployment environments, local benchmarking continues to operate as before.

The previous documentation for the rustc-benchmark-suite can be found [here](https://kobzol.github.io/rust/rustc/2023/08/18/rustc-benchmark-suite.html). The major difference is the section "Performance measurement workflow" which is documented here.

The table below details a set of keywords, or a glossary of terms, that appear throughout this doc and the codebase. The naming aims to minimally identify the constituent parts of the system.

## Keywords
| Term | Meaning |
|------|---------|
| **artifact** | A single Rust compiler toolchain built from a specific commit SHA. |
| **metric** | A quantifiable metric gathered during the execution of the compiler (e.g. instruction count). |
| **benchmark** | A Rust crate that will be used for benchmarking the performance of `rustc` (a compile-time benchmark) or its codegen quality (a runtime benchmark) |
| **profile** | Describes how to run the compiler (e.g. `cargo build/check`). A profile is a **benchmark parameter**. |
| **scenario** | Further specifies how to invoke the compiler (e.g. incremental rebuild/full build). A scenario is a **benchmark parameter**. |
| **backend** | Codegen backend used when invoking `rustc`. A backend is a **benchmark parameter**. |
| **target** | Roughly the Rust target triple, e.g. `aarch64-unknown-linux-gnu`. A target is a **benchmark parameter**. |
| **benchmark suite** | A set of *benchmarks*. We have two suites - compile-time and runtime. |
| **test case** | A combination of a *benchmark* and its *benchmark parameters* that uniquely identifies a single *test*. For compile-time benchmarks, it's *benchmark* + *profile* + *scenario* + *backend* + *target*, for runtime benchmarks it's just *benchmark*. Unique instance of compile-time/run-time benchmark parameters. |
| **test** | Identifies the act of benchmarking an *artifact* under a specific *test case*. Each test consists of several *test iterations*. |
| **test iteration** | A single actual execution of a *test*. |
| **collection** | A set of all *statistics* for a single *test iteration*. |
| **test result** | The result of gathering all *statistics* from a single *test*. Aggregates results from all *test iterations* of that *test*, so a *test result* is essentially the union of *collections*. Usually we just take the minimum of each statistic out of all its *collections*. | 
| **statistic** | A single measured value of a *metric* in a *test result*. |
| **run** | A set of all *test results* for a set of *test cases* measured on a single *artifact*. |
| **benchmark request** | A request for a benchmarking a *run* for a given *artifact*. Can be either created from a try build on a PR, or it is automatically determined from merged master/release *artifacts*. |
| **benchmark set** | A selection of benchmarks that a collector will run. A collector is assigned a benchmark_set id |
| **collector** | A physical runner for benchmarking the compiler. |
| **cluster** | One or more collectors of the same target, for benchmarking the compiler. |
| **collector_id** | A unique identifier of a *collector* (hard-coded at first for simplicity). |
| **job** | High-level "work item" that defines a set of *test cases* that should be benchmarked on a specific collector. |
| **job_queue** | Queue of *jobs*. |
| **MAX_JOB_FAILS** | Maximum number of failures before a job is marked as a failed. |
| **Assigning a job** | The act of allocating one or more *jobs* to a collector. |
| **website** | A standalone server responsible for inserting work into the queue. |

### Master artifacts

The website will go through all recent master commits, and check if they are done by looking at the `sha` and `status` column in the `benchmark_request` table.

- If the request is already marked as `completed`, nothing happens.
- If the request is `in progress`, check [request completion](#Checking-request-completion).
- If the request is `waiting for parent` commit benchmark to be completed, nothing happens.
- If the request is missing, we will recursively find a set of parent master commits that are missing data (by looking at their status in `benchmark_request`).
    - If the set is non-empty, these commits will be handled recursively with the same logic as this commit.
    - If the set is empty, the request will be *enqueued*.

### Try artifacts

> The logic for try artifacts can either happen both in cron and in the GH webhook listener (that receives `@rust-timer queue/build` notifications), or only in cron.

The website will go through all try artifacts in `benchmark_request` that are not yet marked as `completed`.

- If the request is `waiting for artifacts`, do nothing (sometime later a GH notification will switch the status to `waiting for parent` once the artifacts are ready).
- If the request is `waiting for parent`:
    - Recursively find a set of **grandparent** master commits that are missing data (by looking at their status in `benchmark_request`). This could happen on the edge switch from `waiting for artifacts` to `waiting for parent` in the GH webhook handler, or it could happen in each cron invocation.
    - If that set is empty, generate all necessary **parent** jobs and check if they are all completed in the `job_queue`.
        - If yes, *enqueue* the request.
        - If not, insert these jobs into the jobqueue. This is where backfilling happens, as we can backfill e.g. new backends for a parent master commit that was only benchmarked for LLVM before.
- If the request is `in progress`, check [request completion](#Checking-request-completion).


## Programs that need to be available

`perf` with `/proc/sys/kernel/perf_event_paranoid` set to -1 else the collector will panic. Setting this to 4 is a convenient way for testing error cases however.

## Database schema

For a complete overview of the database structure, refer to the [schema documentation](https://github.com/rust-lang/rustc-perf/blob/master/database/schema.md). Only the most relevant tables are discussed below to prevent this document from becoming overly verbose.

## How The Flow Works 

There are two major components in the new system; the website (CRON) and the collectors.

### CRON Lifecycle

It's simplest to show how the new system works by walking through it step by step. We will start with the website, which accepts requests as a web server and also has a cron job for managing the queue. This is the entry point for how work is queued.

Step 1:

The website also runs a CRON that will split a pending `benchmark_request` (request) into `benchmark_job`'s (jobs) and mark `in_progress` jobs as complete.

When a request is created through a web hook there will be a period of time where the artifact is not ready for benchmarking and will be in the state `waiting_for_artifacts`. Once the artifact is ready the request will move to `artifacts_ready`, indicating that the request is ready for benchmarking. This is updated through an endpoint on the webserver.

The states go as follows;

`waiting_for_artifacts` -> `artifacts_ready` -> `in_progress` -> `completed`

Only one request can presently be `in_progress` at any one time. If a request is in progress the CRON does not start splitting up other requests into jobs.

Step 2:

From here if a request is marked as `completed` then the next request that is in the state `artifacts_ready` will be expanded into the jobs needed to fulfil the request. This will be all the combinations of target, profile,

### Collector Lifecycle

The collectors are registered through configuration in the `collector_config` table. The configuration includes the architecture of the collector and a `benchmark_set` id. The `benchmark_set` is used to lookup what benchmarks the collector should run. If there is only one collector then the set would contain all items. Presently this is hardcoded in the github rustc-perf repository and altered through pull requests.

The collectors run in a loop polling the Postgres database for work and exiting if there is no work for it to do.

Step 1:

Determine if the code the collector is running is out of date, if it is the collector will exit. The collector is run through a bash script which will pull down the latest code from github.

Step 2:

Collector pulls down it's configuration from the database. If there is no configuration matching what the collector should have, the collector will panic and exit the loop. Otherwise the collector will try and dequeue a job, if there is no job it will exit gracefully or go to Step 3.

Step 3:

Once the collector has dequeued the job, the collector will proceed to lookup what benchmarks need to be done by looking them up using the benchmark set id. The collector will then loop over the items in the set executing the benchmarks and recording the results in the `pstat` and `pstat_runtime` tables.

Step 4:

The collectors health is monitored by updating a heartbeat column in the `collector_config` table. The UI will indicate the collector as offline if it is inactive for a specified period of time. This should be caught either by error logs or someone viewing the page and subsequently reporting the collector as offline in Zulip.

## Queue ordering
The ordering of the queue is by priority, we assume that there is a collector online that is currently looking for work.
- In progress requests, if there is a request that's state is `in_progress` the collector will take this request, for this to happen it presumably errored at some point and is restarting.
- Release requests, sorted by date and then name
- Requests whose parents are ready.
  - Do a topological sort (topological index = transitive number of parents that are not finished yet)
  - Order by topological index, type (master before try), then PR number, then `created_at`
- Requests that are waiting for artifacts
  - Order by PR number, then `created_at`

## `benchmark_request` table

This table stores permanent benchmark requests for try builds on PRs and for master and published artifacts. If any benchmarking happens (through the website), there has to be a record of it in benchmark_request.

- `waiting_for_artifacts`: a try build is waiting until CI produces the artifacts needed for benchmarking. At this point in time it is possible for a `request` to have not corresponding commit sha (stored in the tag column)
  - master artifact waits for all its (grand)parent benchmark requests to be completed
  - try artifact waits for all its (grand)parent benchmark requests to be completed, plus optionally for all its direct parent jobs to be completed (due to backfilling)
- `artifacts_ready`: artifact is ready for benchmarking
- `in_progress`: jobs for this request are currently in job_queue, waiting to be benchmarked
- `completed`: all jobs have been completed; either through `success` or a `failure`, and a GH PR comment was sent for try/master builds

## `job_queue` table

This table stores benchmark jobs, which specifically tell the collector which benchmarks it should execute. The jobs after being completed, so that we can quickly figure out what master parent jobs we need to backfill when handling try builds.

If you request backfill of data after and the jobs do not exist in the database, new jobs will be created, but that shouldn't matter, because the collector will pick them up, do essentially a no-op (because the test results will be already in the DB), and then mark the job as finished.

The table keeps the following invariant: each job stored into it has all its corresponding parent test cases benchmarked and stored in the DB.

## Limitations
A lot of what has described required manual intervention or codechanges in the repository. For example registering a new collector or configuring a current one is all done through code changes in the repository or manually updating the database.

Aside from the obvious shortcomings, due to resources, there are some edge-cases that are worth documenting.

### One request at a time

Even if one of the collectors is finished with all of the jobs allocated to it for a request it will effectively spin until the request is fully complete. This helps us to synchronise the workload making it easier to keep track of what is going on.

### Deactivating a collector which has a job in progress

Marking a collector's status from `is_active = true` to `false` in the database does not immediately take the collector offline. Instead it will finish the job that is currently assigned to it and then on the next iteration exit.

### Max retries

The system will try to run a request three times before bailing and moving on to the next request. This does not take into account nuances like the database being unreachable for example.

### Static dependencies

The division of the benchmark sets are statically divided by the collectors, if there are multiple collectors and one went offline then a request would hang. This would require manual intervention to resolve.
