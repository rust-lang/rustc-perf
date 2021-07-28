# Glossary

The following is a glossary of domain specific terminology. Although benchmarks are a seemingly simple domain, they have a surprising amount of complexity. It is therefore useful to ensure that the vocabulary used to describe the domain is consistent and precise to avoid confusion. 

## Basic terms

* **benchmark**: the source of a crate which will be used to benchmark rustc. For example, ["hello world"](https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/helloworld).
* **profile**: a [cargo profile](https://doc.rust-lang.org/cargo/reference/profiles.html). Note: the database uses "opt" whereas cargo uses "release". 
* **scenario**: The scenario under which a user is compiling their code. Currently, this is the incremental cache state and a change in the source since last compilation (e.g., full incremental cache and a `println!` statement is added).  
* **metric**: a name of quantifiable metric being measured (e.g., instruction count)
* **artifact**: a specific version of rustc (usually a commit sha or some sort of human readable "tag" like 1.51.0)

## Benchmarks

* **stress test benchmark**: a benchmark that is specifically designed to stress a certain part of the compiler (e.g., [deeply-nested-async](https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/deeply-nested-async) is meant to stress parts of the compiler used in async code)
* **real world benchmark**: a benchmark based on a real world crate. These are typically copied as-is from crates.io. (e.g., [serde](https://github.com/rust-lang/rustc-perf/tree/master/collector/benchmarks/serde) is a popular crate and the benchmark has not been altered from a release of serde on crates.io). 

## Artifacts

* **artifact tag**: an identifier for a particular artifact (e.g., the tag "1.51.0" would presumably point to the 1.51.0 release of rustc).

## Testing 

* **test case**: a combination of a benchmark, a profile, and a scenario.
* **test**: the act of running an artifact under a test case. Each test result is composed of many iterations.
* **test iteration**: a single iteration that makes up a test. Note: we currently normally run 2 test iterations for each test. 
* **test result**: the collection of all metrics as a result of running a test. 
* **test case metric**: the combination of a test case and a particular metric.
* **statistic**: a single value of a metric in a test result
* **statistic description**: the combination of a metric and a test result which describes a statistic.
* **statistic series**: statistics for the same statistic description over time.
* **run**: a collection of test results for all currently available test cases run on a given artifact. 
* **test result delta**: the delta between two test results for the same test case but (optionally) different artifacts. The [comparison page](https://perf.rust-lang.org/compare.html) lists all the test result deltas as percentages comparing two runs.  

## Analysis

* **significant test result delta**: a test result delta that is above some threshold that we have determined to be an actual change in performance and not noise. 
* **highly sensitive benchmark**: a benchmark that tends to produce large (over some threshold) significant test result deltas. For example, some of the stress tests are highly sensitive; when they do produce significant test result deltas, the percentage change is typically quite large. 
* **highly variable benchmark**: a benchmark that frequently produces (over some threshold) significant test result deltas. This differs from sensitive benchmarks in that the deltas aren't necessarily large, they just happen more frequently than one would expect.  Note: highly variable benchmarks can be classified as noisy. They are different from classically noisy benchmarks in that they often have produce deltas that are above the significance threshold. We cannot therefore be 100% if they are variable because they are noisy or because parts of the compiler being exercised by these benchmarks are just being changed very often. 

## Other 

* **bootstrap**: the process of building the compiler from a previous version of the compiler
* **compiler query**: a query used inside the [compiler query system](https://rustc-dev-guide.rust-lang.org/overview.html#queries).