# Glossary

The following is a glossary of domain specific terminology. Although benchmarks are a seemingly simple domain, they have a surprising amount of complexity. It is therefore useful to ensure that the vocabulary used to describe the domain is consistent and precise to avoid confusion. 

## Common terms

* **metric**: a name of a quantifiable metric being measured (e.g., instruction count).
* **artifact**: a specific rustc binary labeled by some identifier tag (usually a commit sha or some sort of human readable id like "1.51.0" or "test").
* **benchmark suite**: an entire collection of benchmarks, either compile-time or runtime.

## Compile-time benchmark terms

* **benchmark**: the source of a crate which will be used to benchmark rustc. For example, ["hello world"](https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/helloworld).
* **profile**: a compilation configuration.
  - `check` corresponds to running `cargo check`.
  - `debug` corresponds to running `cargo build`.
  - `opt` corresponds to running `cargo build --release`.
  - `doc` corresponds to running rustdoc. 
* **scenario**: describes the incremental cache state and an optional change in the source since last compilation.
  - `full`: incremental compilation is not used.
  - `incr-full`: incremental compilation is used, with an empty incremental cache.
  - `incr-unchanged`: incremental compilation is used, with a full incremental cache and no code changes made.
  - `incr-patched`: incremental compilation is used, with a full incremental cache and some code changes made.
* **backend**: the codegen backend used for compiling Rust code.
  - `llvm`: the default codegen backend
* **category**: a high-level group of benchmarks. Currently, there are three categories, primary (mostly real-world crates), secondary (mostly stress tests), and stable (old real-world crates, only used for the dashboard).
* **artifact type**: describes what kind of artifact does the benchmark build. Either `library` or `binary`.

### Types of compile-time benchmarks

* **stress test benchmark**: a benchmark that is specifically designed to stress a certain part of the compiler. For example, [projection-caching](https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/projection-caching) stresses the compiler's projection caching mechanisms. Corresponds to the `secondary` category.
* **real world benchmark**: a benchmark based on a real world crate. These are typically copied as-is from crates.io. For example, [serde](https://github.com/rust-lang/rustc-perf/tree/master/collector/compile-benchmarks/serde-1.0.136) is a popular crate and the benchmark has not been altered from a release of serde on crates.io. Corresponds to the `primary` or `stable` categories.

## Runtime benchmark terms

* **benchmark**: a function compiled by rustc, which function will be benchmarked.
* **benchmark group**: a crate that contains a set of runtime benchmarks.

## Testing

* **test case**: a combination of parameters that describe the measurement of a single (compile-time or runtime) benchmark - a single `test`
    - For compile-time benchmarks, it is a combination of a benchmark, a profile, and a scenario.
    - For runtime benchmarks, it is currently only the benchmark name.
* **test**: the act of running an artifact under a test case. Each test is composed of many iterations.
* **test iteration**: a single iteration that makes up a test. Note: we currently normally run 3 test iterations for each test. 
* **test result**: the result of the collection of all statistics from running a test. Currently, the minimum value of a statistic from all the test iterations is used for analysis calculations and the website.
* **statistic**: a single measured value of a metric in a test result
* **statistic description**: the combination of a metric and a test case which describes a statistic.
* **statistic series**: statistics for the same statistic description over time.
* **run**: a set of tests for all currently available test cases measured on a given artifact. 

## Analysis

* **artifact comparisons**: the comparison of two artifacts. This is composed of many test result comparisons. The [comparison page](https://perf.rust-lang.org/compare.html) shows a single artifact comparison between two artifacts.
* **test result comparison**: the delta between two test results for the same test case but different artifacts. The [comparison page](https://perf.rust-lang.org/compare.html) lists all the test result comparisons as percentages between two runs.  
* **significance threshold**: the threshold at which a test result comparison is considered "significant" (i.e., a real change in performance and not just noise). You can see how this is calculated [here](https://github.com/rust-lang/rustc-perf/blob/master/docs/comparison-analysis.md#what-makes-a-test-result-significant).
* **significant test result comparison**: a test result comparison above the significance threshold. Significant test result comparisons can be thought of as being "statistically significant".
* **relevant test result comparison**: a test result comparison can be significant but still not be relevant (i.e., worth paying attention to). Relevance is a factor of the test result comparison's significance and magnitude. Comparisons are considered relevant if they are significant and have at least a small magnitude .
* **test result comparison magnitude**: how "large" the delta is between the two test result's under comparison. This is determined by the average of two factors: the absolute size of the change (i.e., a change of 5% is larger than a change of 1%) and the amount above the significance threshold (i.e., a change that is 5x the significance threshold is larger than a change 1.5x the significance threshold).

## Other 

* **bootstrap**: the process of building the compiler from a previous version of the compiler
* **compiler query**: a query used inside the [compiler query system](https://rustc-dev-guide.rust-lang.org/overview.html#queries).
