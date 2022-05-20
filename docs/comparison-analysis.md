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

Whether we actually _report_ an analysis or not depends on the context and how relevant we find the summary of the results over all (see below for an explanation of how the relevance of a summary is determined). For example, in pull request performance "try" runs, we report a performance change if results are "somewhat relevant", while for the triage report, we only report if the we are confident the results are "definitely relevant".

### What makes a test result significant?

A test result is significant if the relative change percentage is considered an outlier against historical data. Determining whether a value is an outlier is done through interquartile range ["fencing"](https://www.statisticshowto.com/upper-and-lower-fences/#:~:text=Upper%20and%20lower%20fences%20cordon,%E2%80%93%20(1.5%20*%20IQR)) (i.e., whether a value exceeds a threshold equal to the third quartile plus 1.5 times the interquartile range):

```
interquartile_range = Q3 - Q1
result > Q3 + (interquartile_range * 3)
```

(Assuming the data is ordered, Q3 is the median of the upper half of the data while Q1 is the median of the lower half.)

We ignore the lower fence, because result data is bounded by 0.

This upper fence is called the "significance threshold".

### How is relevance of a test run summary determined?

The relevance test run summary is determined by the number of significant and relevant test results and their magnitude.

#### Magnitude

Magnitude is a small set of discrete buckets describing how "big" a change is from "very small" to "very large". It is a combination of two factors:
* how much that change went over the significance threshold.
  * this criteria is the same regardless of which metric is being measured.
* how large percentage wise a change is regardless of the direction of the change.
  * which bucket a change falls into is metric dependent (i.e., changes of the same percent might fall into different buckets depending on the metric in question)

As an example, if a change that is large in absolute terms only exceeds the significance threshold by a small factor, then the overall magnitude of the change is considered small. On the other hand, if a change is small in absolute terms but exceeds the significance threshold by a very large amount, then the overall magnitude of the change is considered large.

#### Relevance algorithm

The actual algorithm for determining relevance of a comparison summary may change, but in general the following rules apply:
* High relevance: any number of very large or large changes, a small amount of medium changes, or a large number of small or very small changes.
* Medium relevance: any number of very large or large changes, any medium change, or smaller but still substantial number of small or very small changes.
* Low relevance: if it doesn't fit into the above two categories, it ends in this category.
