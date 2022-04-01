# Glossary

The following is a glossary of domain specific terminology. Although benchmarks are a seemingly simple domain, they have a surprising amount of complexity. It is therefore useful to ensure that the vocabulary used to describe the domain is consistent and precise to avoid confusion. 

## Basic terms

* **benchmark**: the source of a crate which will be used to benchmark rustc. For example, ["hello world"](https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/helloworld).
* **profile**: a [cargo profile](https://doc.rust-lang.org/cargo/reference/profiles.html). Note: the database uses "opt" whereas cargo uses "release". 
* **scenario**: The scenario under which a user is compiling their code. Currently, this is the incremental cache state and an optional change in the source since last compilation (e.g., full incremental cache and a `println!` statement is added).  
* **metric**: a name of a quantifiable metric being measured (e.g., instruction count)
* **artifact**: a specific version of rustc (usually a commit sha or some sort of human readable "tag" like 1.51.0)
* **category**: a high-level group of benchmarks. Currently, there are three categories, primary (mostly real-world crates), secondary (mostly stress tests), and stable (old real-world crates, only used for the dashboard).

## Benchmarks

* **stress test benchmark**: a benchmark that is specifically designed to stress a certain part of the compiler. For example, [projection-caching](https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/projection-caching) stresses the compiler's projection caching mechanisms.
* **real world benchmark**: a benchmark based on a real world crate. These are typically copied as-is from crates.io. For example, [serde](https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/serde-1.0.136) is a popular crate and the benchmark has not been altered from a release of serde on crates.io. 

## Artifacts

* **artifact tag**: an identifier for a particular artifact (e.g., the tag "1.51.0" would presumably point to the 1.51.0 release of rustc).

## Testing 

* **test case**: a combination of a benchmark, a profile, and a scenario.
* **test**: the act of running an artifact under a test case. Each test result is composed of many iterations.
* **test iteration**: a single iteration that makes up a test. Note: we currently normally run 2 test iterations for each test. 
* **test result**: the result of the collection of all statistics from running a test. Currently the minimum of the statistics.
* **statistic**: a single value of a metric in a test result
* **statistic description**: the combination of a metric and a test case which describes a statistic.
* **statistic series**: statistics for the same statistic description over time.
* **run**: a collection of test results for all currently available test cases run on a given artifact. 
* **test result delta**: the delta between two test results for the same test case but (optionally) different artifacts. The [comparison page](https://perf.rust-lang.org/compare.html) lists all the test result deltas as percentages comparing two runs.  

## Analysis

* **test result delta**: the difference between two test results for the same metric and test case.
* **significance threshold**: the threshold at which a test result delta is considered "significant" (i.e., a real change in performance and not just noise). This is calculated using some statistical measure (see the code for how this is currently being done).
* **significant test result delta**: a test result delta above the significance threshold.
* **dodgy test case**: a test case for which the significance threshold is significantly large indicating a high amount of variability in the test and thus making it necessary to be somewhat skeptical of any results too close to the significance threshold.
* **relevant test result delta**: a synonym for *significant test result delta* in situations where the term "significant" might be ambiguous and readers may potentially interpret *significant* as "large" or "statistically significant". For example, in try run results, we use the term relevant instead of significant.

## Other 

* **bootstrap**: the process of building the compiler from a previous version of the compiler
* **compiler query**: a query used inside the [compiler query system](https://rustc-dev-guide.rust-lang.org/overview.html#queries).
