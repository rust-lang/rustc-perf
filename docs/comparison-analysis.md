# Comparison Analysis

The following is a detailed explanation of the process undertaken to automate the analysis of test results for two artifacts of interest (artifact A and B). 

This analysis can be done by hand, by using the [comparison page](https://perf.rust-lang.org/compare.html) and entering the two artifacts of interest in the form at the top.

## The goal

The goal of the analysis is to determine whether artifact B represents a performance improvement, regression, or mixed result from artifact A. Typically artifact B will be based on artifact A with a certain pull requests changes applied to artifact A to get artifact B, but this is not required.

Performance analysis is typically used to determine whether a pull request has introduced performance regressions or improvements.

## What is being compared?

At the core of comparison analysis are the collection of test results for the two artifacts being compared. For each test case, the statistics for the two artifacts are compared and a relative change percentage is obtained using the following formula:

```
100 * ((statisticForArtifactB - statisticForArtifactA) / statisticForArtifactA)
```

## High-level analysis description 

Analysis of the changes is performed in order to determine whether artifact B represents a performance change over artifact A. At a high level the analysis performed takes the following form:

How many _significant_ test results indicate performance changes? If all significant test results indicate regressions (i.e., all percent relative changes are positive), then artifact B represents a performance regression over artifact A. If all significant test results indicate improvements (i.e., all percent relative changes are negative), then artifact B represents a performance improvement over artifact B. If some significant test results indicate improvement and others indicate regressions, then the performance change is mixed.

Whether we actually _report_ an analysis or not depends on the context and how _confident_ we are in the summary of the results (see below for an explanation of how confidence is derived). For example, in pull request performance "try" runs, we report a performance change if we are at least confident that the results are "probably relevant", while for the triage report, we only report if the we are confident the results are "definitely relevant".

### What makes a test result significant?

A test result is significant if the relative change percentage meets some threshold. What the threshold is depends of whether the test case is "dodgy" or not (see below for an examination of "dodginess"). For dodgy test cases, the threshold is set at 1%. For non-dodgy test cases, the threshold is set to 0.1%.

### What makes a test case "dodgy"?

A test case is "dodgy" if it shows signs of either being noisy or highly variable.

To determine noise and high variability, the previous 100 test results for the test case in question are examined by calculating relative delta changes between adjacent test results. This is done with the following formula (where `testResult1` is the test result immediately proceeding `testResult2`):

```
testResult2 - testResult1 / testResult1
```

Any relative delta change that is above a threshold (currently 0.1) is considered "significant" for the purposes of dodginess detection.

A highly variable test case is one where a certain percentage (currently 5%) of relative delta changes are significant. The logic being that test cases should only display significant relative delta changes a small percentage of the time.

A noisy test case is one where of all the non-significant relative delta changes, the average delta change is still above some threshold (0.001). The logic being that non-significant changes should, on average, being very close to 0. If they are not close to zero, then they are noisy.

### How is confidence in whether a test analysis is "relevant" determined?

The confidence in whether a test analysis is relevant depends on the number of significant test results. Depending on that number a confidence level is reached:

* Maybe relevant: 0-3 changes 
* Probably relevant: 4-6 changes 
* Definitely relevant: >6 changes 

Note: changes can be any combination of positive or negative changes.