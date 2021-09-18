# Schema

Below is an explanation of the current database schema. This schema is duplicated across the (currently) two database backends we support: sqlite and postgres.


## Overview

In general, the database is used to track three groups of things:
* Performance run statistics (e.g., instruction count) on a per benchmark, profile, and cache-state basis.
* Self profile data gathered with `-Zself-profile`.
* State when running GitHub bots and the performance runs (e.g., how long it took for a performance suite to run, errors encountered a long the way, etc.)

Below are some diagrams showing the basic layout of the database schema for these three uses:

### Performance run statistics

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
│ ┌───────────────┐  ┌──────────┐   |               │
│ │ pstat_series  │  │ pstat    │   │               │
│ ├───────────────┤  ├──────────┤   │               │ 
│ │ id *          │◄┐│ id *     │   │               │
└─┤ crate         │ └┤ series   │   │               │ 
  │ profile       │  │ aid      ├───┼───────────────┘
  │ cache         │  │ cid      ├───┘
  │ statistic     │  │ value    │
  └───────────────┘  └──────────┘
```

### Self profile data

**TODO**

### Miscellaneous State

**TODO**

## Tables

### benchmark

The different types of benchmarks that are run. 

The table stores the name of the benchmark as well as whether it is capable of being run using the stable compiler.  The benchmark name is used as a foreign key in many of the other tables. 

```
sqlite> select * from benchmark limit 1;
name        stabilized
----------  ----------
helloworld  0   
```

### artifact

A description of a rustc compiler artifact being benchmarked. 

This description includes:
* name: usually a commit sha or a tag like "1.51.0" but is free-form text so can be anything.
* date: the date associated with this compiler artifact (usually only when the name is a commit) 
* type: currently one of "master" (i.e., we're testing a merge commit), "try" (someone is testing a PR), and "release" (usually a release candidate - though local compilers also get labeled like this).

```
sqlite> select * from artifact limit 1;
id          name        date        type      
----------  ----------  ----------  ----------
1           LOCAL_TEST              release  
```

### collection

A "collection" of benchmarks tied only differing by the statistic collected.

This is a way to collect statistics together signifying that they belong to the same logical benchmark run. 

Currently the collection also marks the git sha of the currently running collector binary.

```
sqlite> select * from collection limit 1;
id          perf_commit                              
----------  -----------------------------------------
1           d9fd96f409a15429757030f225b082744a72516c
```

### pstat_series

A unique collection of crate, profile, cache and statistic.

* crate: the benchmarked crate which might be a crate from crates.io or a crate made specifically to stress some part of the compiler.
* profile: what type of compilation is happening - check build, optimized build (a.k.a. release build), debug build, or doc build.
* cache: how much of the incremental cache is full. An empty incremental cache means that the compiler must do a full build.
* statistic: the type of stat being collected

```
sqlite> select * from pstat_series limit 1;
id          crate       profile     cache       statistic   
----------  ----------  ----------  ----------  ------------
1           helloworld  check       full        task-clock:u
```

### pstat

A statistic that is unique to a pstat_series, artifact and collection.

This stat is unique across a benchmarked crate, profile, cache state, statistic, rustc artifact, and benchmarks "collection".

```
sqlite> select * from pstat limit 1;
series      aid         cid         value     
----------  ----------  ----------  ----------
1           1           1           24.93   
```


### self_profile_query_series

**TODO**

### self_profile_query

**TODO**

### pull_request_build

**TODO**

### artifact_collection_duration

Records how long benchmarking takes in seconds.

```
sqlite> select * from artifact_collection_duration limit 1;
aid         date_recorded  duration  
----------  -------------  ----------
1           1625829965     4 
```

### collector_progress

Keeps track of the collector's start and finish time as well as which step it's currently on.

```
sqlite> select * from collector_progress limit 1;
aid         step        start       end       
----------  ----------  ----------  ----------
1           helloworld  1625829961  1625829965
```

### rustc_compilation

**TODO**

### error_series

**TODO**

### error

**TODO**
