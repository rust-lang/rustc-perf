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

How many _significant_ test results indicate performance changes and what is the magnitude of the changes (i.e., how large are the changes regardless of the direction of change)?

* If there are improvements and regressions with magnitude of medium or above then the comparison is mixed.
* If there are only either improvements or regressions then the comparison is labeled with that kind.
* If one kind of changes are of medium or above magnitude (and the other kind are not), then the comparison is mixed if 15% or more of the total changes are the other (small magnitude) kind. For example:
  * given 20 regressions (with at least 1 of medium magnitude) and all improvements of low magnitude, the comparison is only mixed if there are 4 or more improvements.
  * given 5 regressions (with at least 1 of medium magnitude) and all improvements of low magnitude, the comparison is only mixed if there 1 or more improvements.
* If both kinds of changes are all low magnitude changes, then the comparison is mixed unless 90% or more of total changes are of one kind. For example:
  * given 20 changes of different kinds all of low magnitude, the result is mixed unless only 2 or fewer of the changes are of one kind.
  * given 5 changes of different kinds all of low magnitude, the result is always mixed.

Whether we actually _report_ an analysis or not depends on the context and how _confident_ we are in the summary of the results (see below for an explanation of how confidence is derived). For example, in pull request performance "try" runs, we report a performance change if we are at least confident that the results are "probably relevant", while for the triage report, we only report if the we are confident the results are "definitely relevant".

### What makes a test result significant?

A test result is significant if the relative change percentage is considered an outlier against historical data. Determining whether a value is an outlier is done through interquartile range "fencing" (i.e., whether a value exceeds a threshold equal to the third quartile plus 1.5 times the interquartile range):

```
interquartile_range = Q3 - Q1
result > Q3 + (interquartile_range * 3)
```

(Assuming the data is ordered, Q3 is the median of the upper half of the data while Q1 is the median of the lower half.)

We ignore the lower fence, because result data is bounded by 0.

This upper fence is often called the "significance threshold".

### How is confidence in whether a test analysis is "relevant" determined?

The confidence in whether a test analysis is relevant depends on the number of significant test results and their magnitude.

#### Magnitude

Magnitude is a combination of two factors:
* how large a change is regardless of the direction of the change
* how much that change went over the significance threshold

If a large change only happens to go over the significance threshold by a small factor, then the over magnitude of the change is considered small.

#### Confidence algorithm

The actual algorithm for determining confidence may change, but in general the following rules apply:
* Definitely relevant: any number of very large or large changes, a small amount of medium changes, or a large amount of small or very small changes.
* Probably relevant: any number of very large or large changes, any medium change, or smaller but still substantial amount of small or very small changes.
* Maybe relevant: if it doesn't fit into the above two categories, it ends in this category.

### "Dodgy" Test Cases

"Dodgy" test cases are test cases that tend to produce unreliable results (i.e., noise). A test case is considered "dodgy" if its significance threshold is sufficiently far enough away from 0.
