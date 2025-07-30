# Schema

Below is an explanation of the current database schema. This schema is duplicated across the (currently) two database backends we support: sqlite and postgres.

## Overview

In general, the database is used to track four groups of things:
* Performance run statistics (e.g., instruction count) for compile time benchmarks on a per benchmark, profile, and scenario basis.
* Performance run statistics (e.g., instruction count) for runtime benchmarks on a per benchmark basis.
* Self profile data gathered with `-Zself-profile`.
* State when running GitHub bots and the performance runs (e.g., how long it took for a performance suite to run, errors encountered along the way, etc.)

Below are some diagrams showing the basic layout of the database schema for these three uses:

### Performance run statistics

Here is the diagram for compile-time benchmarks:
```
  ┌────────────┐  ┌───────────────┐  ┌────────────┐   
  │ benchmark  │  │ collection    │  │ artifact   │
  ├────────────┤  ├───────────────┤  ├────────────┤
┌►│ name *     │  │ id *          │◄┐│ id *       │◄┐
│ │ stabilized │  │ perf_commit   │ ││ name       │ │
│ │            │  │               │ ││ date       │ │
│ │            │  │               │ ││ type       │ │
│ └────────────┘  └───────────────┘ │└────────────┘ │
│                                   │               │
│                                   │               │
│ ┌───────────────┐  ┌──────────┐   │               │
│ │ pstat_series  │  │ pstat    │   │               │
│ ├───────────────┤  ├──────────┤   │               │ 
│ │ id *          │◄┐│ id *     │   │               │
└─┤ crate         │ └┤ series   │   │               │ 
  │ profile       │  │ aid      ├───┼───────────────┘
  │ scenario      │  │ cid      │   │
  │ backend       │  │ value    ├───┘
  │ metric        │  └──────────┘
  │ target        │
  └───────────────┘
```

For runtime benchmarks the schema very similar, but there are different table names:
- `pstat` => `runtime_pstat`
- `pstat_series` => `runtime_pstat_series`
  - There are different attributes here, `benchmark` and `metric`.

## Tables

### artifact

A description of a rustc compiler artifact being benchmarked.

This description includes:
* name: usually a commit sha or a tag like "1.51.0" but is free-form text so can be anything.
* date: the date associated with this compiler artifact (usually only when the name is a commit)
* type: currently one of "master" (i.e., we're testing a merge commit), "try" (someone is testing a PR), and "release" (usually a release candidate - though local compilers also get labeled like this).

```
sqlite> select * from artifact limit 1;
id          name        date        type   
----------  ----------  ----------  -------
1           LOCAL_TEST              release
```

### artifact size

Records the size of individual components (like `librustc_driver.so` or `libLLVM.so`) of a single
artifact.

This description includes:
* component: normalized name of the component (hashes are removed)
* size: size of the component in bytes

```
sqlite> select * from artifact_size limit 1;
aid         component   size
----------  ----------  ----------
1           libLLVM.so  177892352
```

### collection

A "collection" of benchmarks tied only differing by the statistic collected.

This corresponds to a [`test result`](../docs/glossary.md#testing).

This is a way to collect statistics together signifying that they belong to the same logical benchmark run.

Currently, the collection also marks the git sha of the currently running collector binary.

```
sqlite> select * from collection limit 1;
id          perf_commit 
----------  ----------------------------------------
1           d9fd96f409a15429757030f225b082744a72516c
```

### collector_progress

Keeps track of the collector's start and finish time as well as which step it's currently on.

```
sqlite> select * from collector_progress limit 1;
aid         step        start       end
----------  ----------  ----------  ----------
1           helloworld  1625829961  1625829965
```

### artifact_collection_duration

Records how long benchmarking takes in seconds.

```
sqlite> select * from artifact_collection_duration limit 1;
aid         date_recorded  duration
----------  -------------  ----------
1           1625829965     4
```

### benchmark

The different types of compile-time benchmarks that are run. 

The table stores the name of the benchmark, whether it is capable of being run using the stable compiler,
and its category. The benchmark name is used as a foreign key in many of the other tables.

Category is either `primary` (real-world benchmark) or `secondary` (stress test).
Stable benchmarks have `category` set to `primary` and `stabilized` set to `1`.

```
sqlite> select * from benchmark limit 1;
name        stabilized  category
----------  ----------  ----------
helloworld  0           primary
```

### pstat_series

Describes the parametrization of a compile-time benchmark. Contains a unique combination
of a crate, profile, scenario and the metric being collected.

* crate (aka `benchmark`): the benchmarked crate which might be a crate from crates.io or a crate made specifically to stress some part of the compiler.
* profile: what type of compilation is happening - check build, optimized build (a.k.a. release build), debug build, or doc build.
* scenario: describes how much of the incremental cache is full. An empty incremental cache means that the compiler must do a full build.
* backend: codegen backend used for compilation.
* metric: the type of metric being collected.

This corresponds to a [`statistic description`](../docs/glossary.md).

There is a separate table for this collection to avoid duplicating crates, profiles, scenarios etc.
many times in the `pstat` table.

```
sqlite> select * from pstat_series limit 1;
id          crate       profile     scenario    backend  target                    metric      
----------  ----------  ----------  ----------  -------  ------------              ------------
1           helloworld  check       full        llvm     x86_64-linux-unknown-gnu  task-clock:u
```

### pstat

A measured value of a compile-time metric that is unique to a `pstat_series`, `artifact` and a `collection`.

Each measured combination of a collection, rustc artifact, benchmarked crate, profile, scenario and a metric
has its own unique entry in this table.

```
sqlite> select * from pstat limit 1;
series      aid         cid         value
----------  ----------  ----------  ----------
1           1           1           24.93
```

### runtime_pstat_series

Describes the parametrization of a runtime benchmark. Contains a unique combination
of a benchmark and the metric being collected.

This table exists to avoid duplicating crates, profiles, scenarios etc. many times in the `runtime_pstat` table.

```
sqlite> select * from runtime_pstat_series limit 1;
id          benchmark  metric
----------  ---------  --------------
1           nbody-10k  instructions:u
```

### runtime_pstat

A measured value of a runtime metric that is unique to a `runtime_pstat_series`, `artifact` and a `collection`.

Each measured combination of a collection, rustc artifact, benchmark and a metric
has its own unique entry in this table.

```
sqlite> select * from runtime_pstat limit 1;
series      aid         cid         value
----------  ----------  ----------  ----------
1           1           1           24.93
```

### rustc_compilation

Records the duration of compiling a `rustc` crate for a given artifact and collection.

```
sqlite> select * from rustc_compilation limit 1;
aid  cid  crate                duration
---  ---  ----------           --------
1    42   rustc_mir_transform  28.096
```

### raw_self_profile

Records that a given combination of artifact, collection, benchmark, profile and scenario
has a self profile archive available. This profile is then downloaded through an endpoint -
it is not stored in the database directly.

```
sqlite> select * from raw_self_profile limit 1;
aid  cid  crate        profile  cache
---  ---  -----        -------  -----
1    42   hello-world  debug    full
```

### pull_request_build

Records a pull request commit that is waiting in a queue to be benchmarked.

First a merge commit is queued, then its artifacts are built by bors, and once the commit
is attached to the entry in this table, it can be benchmarked.

* bors_sha: SHA of the commit that should be benchmarked
* pr: number of the PR
* parent_sha: SHA of the parent commit, to which will the PR be compared
* complete: bool specifying whether this commit has been already benchmarked or not
* requested: when was the commit queued
* include: which benchmarks should be included (corresponds to the `--include` benchmark parameter)
* exclude: which benchmarks should be excluded (corresponds to the `--exclude` benchmark parameter)
* runs: how many iterations should be used by default for the benchmark run
* commit_date: when was the commit created
* backends: the codegen backends to use for the benchmarks (corresponds to the `--backends` benchmark parameter)

```
sqlite> select * from pull_request_build limit 1;
bors_sha    pr  parent_sha  complete  requested    include  exclude  runs  commit_date  backends
----------  --  ----------  --------  ---------    -------  -------  ----  -----------  --------
1w0p83...   42  fq24xq...   true      <timestamp>                    3     <timestamp>
```

### error

Records a compilation or runtime error for an artifact and a benchmark.

```
sqlite> select * from error limit 1;
aid         benchmark   error
----------  ---         -----
1           syn-1.0.89  Failed to compile...
```

## New benchmarking design
We are currently implementing a new design for dispatching benchmarks to collector(s) and storing
them in the database. It will support new use-cases, like backfilling of new benchmarks into a parent
commit and primarily benchmarking with multiple collectors (and multiple hardware architectures) in
parallel.

The tables below are a part of the new scheme.

### benchmark_request

Represents a single request for performing a benchmark collection. Each request can be one of three types:

* Master: benchmark a merged master commit
* Release: benchmark a published stable or beta compiler toolchain
* Try: benchmark a try build on a PR

Columns:

* **id** (`int`): Unique ID.
* **tag** (`text`): Identifier of the compiler toolchain that should be benchmarked.
  * Commit SHA for master/try requests or release name (e.g. `1.80.0`) for release requests.
  * Can be `NULL` for try requests that were queued for a perf. run, but their compiler artifacts haven't been built yet.
* **parent_sha** (`text`): Parent SHA of the benchmarked commit.
  * Can be `NULL` for try requests without compiler artifacts.
* **commit_type** (`text NOT NULL`): One of `master`, `try` or `release`.
* **pr** (`int`): Pull request number associated with the master/try commit.
  * `NULL` for release requests.
* **created_at** (`timestamptz NOT NULL`): Datetime when the request was created.
* **completed_at** (`timestamptz`): Datetime when the request was completed.
* **status** (`text NOT NULL`): Current status of the benchmark request.
  * `waiting-for-artifacts`: A try request waiting for compiler artifacts.
  * `artifacts-ready`: Request that has compiler artifacts ready and can be benchmarked.
  * `in-progress`: Request that is currently being benchmarked.
  * `completed`: Completed request.
* **backends** (`text NOT NULL`): Comma-separated list of codegen backends to benchmark. If empty, the default set of codegen backends will be benchmarked.
* **profiles** (`text NOT NULL`): Comma-separated list of profiles to benchmark. If empty, the default set of profiles will be benchmarked.

### collector_config

Information about the collector; it's target architecture, when it was added,
whether it is active and when it last had activity denoted by `last_heartbeat_at`.

Columns:

* **id** (`id`): A unique identifier for the collector.
* **target** (`text NOT NULL`): The ISA of the collector for example; `aarch64-unknown-linux-gnu`.
* **name** (`text NOT NULL`): Unique name for the collector.
* **date_added** (`timestamptz NOT NULL`): When the collector was added
* **last_heartbeat_at** (`timestamptz`): When the collector last updated this
  column, a way to test if the collector is still alive.
* **benchmark_set** (`int NOT NULL`): ID of the predefined benchmark suite to
  execute.
* **is_active** (`boolean NOT NULL`): For controlling whether the collector is
  active for use. Useful for adding/removing collectors.

### job_queue

This table stores ephemeral benchmark jobs, which specifically tell the
collector which benchmarks it should execute. The jobs will be kept in the
table for ~30 days after being completed, so that we can quickly figure out
what master parent jobs we need to backfill when handling try builds.

Columns:

- **id** (`bigint` / `serial`): Primary-key identifier for the job row;
  auto-increments with each new job.
- **request_id** (`bigint`): References the parent benchmark request that
  spawned this job.
- **target** (`text NOT NULL`): Hardware/ISA the benchmarks must run on
  (e.g. AArch64, x86_64).
- **backend** (`text NOT NULL`): Code generation backend the collector should
  test (e.g. llvm, cranelift).
- **benchmark_set** (`int NOT NULL`): ID of the predefined benchmark suite to
  execute.
- **collector_id** (`text`): Id of the collector that claimed the job
  (populated once the job is started).
- **created_at** (`timestamptz NOT NULL`): Datetime when the job was queued.
- **started_at** (`timestamptz`): Datetime when the collector actually began
  running the benchmarks; NULL until the job is claimed.
- **completed_at** (`timestampt`): Datetime when the collector finished
  (successfully or otherwise); used to purge rows after ~30 days.
- **status** (`text NOT NULL`): Current job state. `queued`, `in_progress`,
  `success`, or `failure`.
- **retry** (`int NOT NULL`): Number of times the job has been re-queued after
  a failure; 0 on the first attempt.
- **error** (`text`): Optional error message or stack trace from the last
  failed run; NULL when the job succeeded.
