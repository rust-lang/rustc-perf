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

Columns:

* **name** (`text`): Usually a commit sha "fefce3cecd69cebf2d7c9aa3dd90a84379f4201a" or a tag like "1.51.0" but is free-form text so could be anything.
* **date** (`timestamptz`: The date associated with this compiler artifact (usually only when the name is a commit)
* **type** (`text`): Currently one of "master" (i.e., we're testing a merge commit), "try" (someone is testing a PR), and "release" (usually a release candidate - though local compilers also get labeled like this).

### artifact size

Records the size of individual components (like `librustc_driver.so` or `libLLVM.so`) of a single
artifact.

Columns:

* **aid** (`integer`): Artifact id, references the id in the artifact table
* **component** (`text`): Normalized name of the component (hashes are removed)
* **size** (`integer`): Size of the component in bytes

### collection

A "collection" of benchmarks tied only differing by the statistic collected.

This corresponds to a [`test result`](../docs/glossary.md#testing).

This is a way to collect statistics together signifying that they belong to the same logical benchmark run.

Currently, the collection also marks the git sha of the currently running collector binary.

Columns:

* **id** (`integer`): Unique identifier
* **perf_commit** (`text`): Commit sha / tag

### collector_progress

Keeps track of the collector's start and finish time as well as which step it's currently on.

Columns:

* **aid** (`integer`): Artifact id, references the id in the artifact table
* **step** (`text`): The step the collector is currently benchmarking
* **start** (`timestamptz`): When the collector started
* **end** (`timestamptz`): When the collector finished

### artifact_collection_duration

Records how long benchmarking takes in seconds.

Columns:

* **aid** (`integer`): Artifact id, references the id in the artifact table
* **date_recorded** (`timestamptz`): When this was recorded
* **duration** (`integer`): How long the benchmarking took in seconds

### benchmark

The different types of compile-time benchmarks that are run. 

The table stores the name of the benchmark, whether it is capable of being run using the stable compiler,
and its category. The benchmark name is used as a foreign key in many of the other tables.

Category is either `primary` (real-world benchmark) or `secondary` (stress test).
Stable benchmarks have `category` set to `primary` and `stabilized` set to `1`.

Columns:

* **name** (`text`): Name of the benchmark
* **stabilized** (`boolean`): Whether the benchmark supports stable
* **category** (`category`): `primary` if this is a 'real-world' benchmark or `secondary` if a 'stress test'

### pstat_series

Describes the parametrization of a compile-time benchmark. Contains a unique combination
of a crate, profile, scenario and the metric being collected.

Columns:

* **crate** (`text`) (aka `benchmark`): The benchmarked crate which might be a crate from crates.io or a crate made specifically to stress some part of the compiler.
* **profile** (`text`): What type of compilation is happening - check build, optimized build (a.k.a. release build), debug build, or doc build.
* **scenario** (`text`): Describes how much of the incremental cache is full. An empty incremental cache means that the compiler must do a full build.
* **backend** (`text`): Codegen backend used for compilation, for example 'llvm'
* **metric** (`text`): the type of metric being collected.

This corresponds to a [`statistic description`](../docs/glossary.md).

There is a separate table for this collection to avoid duplicating crates, profiles, scenarios etc... many times in the `pstat` table.

### pstat

A measured value of a compile-time metric that is unique to a `pstat_series`, `artifact` and a `collection`.

Each measured combination of a collection, rustc artifact, benchmarked crate, profile, scenario and a metric
has its own unique entry in this table.

Columns:

* **series** (`integer`): References `pstat_series` id
* **aid** (`integer`): Artifact id, references the id in the artifact table
* **cid** (`integer`): Collection id, references the id in the collection table
* **value** (`double precision`): The value of the metric that has been measured, for example time

### runtime_pstat_series

Describes the parametrization of a runtime benchmark. Contains a unique combination
of a benchmark and the metric being collected.

This table exists to avoid duplicating crates, profiles, scenarios etc. many times in the `runtime_pstat` table.

Columns:

* **id** (`integer`): Unique identifier
* **benchmark** (`text`): The name of the benchmark
* **metric** (`text`): The metric that was measured

### runtime_pstat

A measured value of a runtime metric that is unique to a `runtime_pstat_series`, `artifact` and a `collection`.

Each measured combination of a collection, rustc artifact, benchmark and a metric
has its own unique entry in this table.

Columns:

* **series** (`integer`): References `runtime_pstat_series` id
* **aid** (`integer`): Artifact id, references the id in the artifact table
* **cid** (`integer`): Collection id, references the id in the collection table
* **value** (`double precision`): The value of the metric that has been measured, for example time

### rustc_compilation

Records the duration of compiling a `rustc` crate for a given artifact and collection.

Columns:

* **aid** (`integer`): Artifact id, references the id in the artifact table
* **cid** (`integer`): Collection id, references the id in the collection table
* **crate** (`text`):  The name of the rustc crate
* **duration** (`big int`): How long compiling the rustc crate took

### raw_self_profile

Records that a given combination of artifact, collection, benchmark, profile and scenario
has a self profile archive available. This profile is then downloaded through an endpoint -
it is not stored in the database directly.

Columns:

* **aid** (`integer`): Artifact id, references the id in the artifact table
* **cid** (`integer`): Collection id, references the id in the collection table
* **crate** (`text`):  The name of the crate
* **profile** (`text`): What type of compilation is happening - check build, optimized build (a.k.a. release build), debug build, or doc build.
* **cache** (`text`): The incremental scenario used for the benchmark

### pull_request_build

Records a pull request commit that is waiting in a queue to be benchmarked.

First a merge commit is queued, then its artifacts are built by bors, and once the commit
is attached to the entry in this table, it can be benchmarked.

* **bors_sha** (`text`): SHA of the commit that should be benchmarked
* **pr** (`integer`): number of the PR
* **parent_sha** (`text`): SHA of the parent commit, to which will the PR be compared
* **complete** (`boolean`): Specifies whether this commit has been already benchmarked or not
* **requested** (`timestamptz`): When was the commit queued
* **include** (`text`): Which benchmarks should be included (corresponds to the `--include` benchmark parameter), comma separated strings
* **exclude** (`text`): Which benchmarks should be excluded (corresponds to the `--exclude` benchmark parameter), comma separated strings
* **runs** (`integer`): How many iterations should be used by default for the benchmark run
* **commit_date** (`timestamptz`): When was the commit created
* **backends** (`text`): The codegen backends to use for the benchmarks (corresponds to the `--backends` benchmark parameter)

### error

Records an error within the application namely a;
- compilation
- runtime
- error contextual to a benchmark job

Columns:

* **id** (`BIGINT` / `SERIAL`): Primary key identifier for the error row;
  auto increments with each new error.
* **aid** (`INTERGER`): References the artifact id column.
* **context** (`TEXT NOT NULL`): A little message to be able to understand a 
  bit more about why or where the error occured.
* **message** (`TEXT NOT NULL`): The error message.
* **job_id** (`INTEGER`): A nullable job_id which, if it exists it will inform
  us as to which job this error is part of.

### benchmark_request

Represents a single request for performing a benchmark run. Each request can be one of three types:

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
* **commit_date** (`timestamptz`): Datetime when the compiler artifact commit (not the request) was created.
  * Can be `NULL` for try requests without compiler artifacts.
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

This table stores benchmark jobs, which specifically tell the
collector which benchmarks it should execute.

Columns:

* **id** (`bigint` / `serial`): Primary key identifier for the job row;
  autoincrements with each new job.
* **request_tag** (`text`): References the parent benchmark request that
  spawned this job.
* **target** (`text NOT NULL`): Hardware/ISA the benchmarks must run on
  (e.g. AArch64, x86_64).
* **backend** (`text NOT NULL`): Code generation backend the collector should
  test (e.g. llvm, cranelift).
* **benchmark_set** (`int NOT NULL`): ID of the predefined benchmark suite to
  execute.
* **collector_name** (`text`): Name of the collector that claimed the job
  (populated once the job is started).
* **created_at** (`timestamptz NOT NULL`): Datetime when the job was queued.
* **started_at** (`timestamptz`): Datetime when the collector actually began
  running the benchmarks; NULL until the job is claimed.
* **completed_at** (`timestampt`): Datetime when the collector finished
  (successfully or otherwise); used to purge rows after ~30 days.
* **status** (`text NOT NULL`): Current job state. `queued`, `in_progress`,
  `success`, or `failure`.
* **retry** (`int NOT NULL`): Number of times the job has been re*queued after
  a failure; 0 on the first attempt.
* **kind** (`text NOT NULL`): What benchmark suite should be executed in the job (`compiletime`, `runtime` or `rustc`).
* **is_optional** (`boolean NOT NULL`): Whether a request should wait for this job to finish before it will become completed.
