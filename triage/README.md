# Rust Compiler Performance Triage

We regularly triage the effects of merged PRs on rustc's speed and memory
usage.

## Roster

- ecstatic-morse
- Mark-Simulacrum
- rylev
- pnkfelix

Monday evening to Tuesday afternoon in North America is a good time to do it 
because This Week in Rust (see below) is usually published on Wednesday, US time, 
and so it means the PR to include the triage details in TWiR can be merged shortly 
before publication. This time is also shortly before the weekly Rust compiler 
meeting, where the results are looked at.

## Generating the report 

First, check the previous triage log entry. Look for responses in PRs, and
follow up on any promised actions. (i.e. nag people!)

While looking at the previous triage log entry, take note of the final commit in
the revision range from that triage period. The noted final commit `$PARENT` will
be the parent commit that we use for the current round of triage.

Use the API endpoint to automate building the file:

```
% curl https://perf.rust-lang.org/perf/triage -d "{\"start\": \"$PARENT\"}" > YYYY-MM-DD.md
```

## Analysis

The following is a list of items you should look for when interpreting performance results. 

Go through each entry in the report and verify that it is properly labeled as a regression, 
improvement, or a mix of the two. For instance, some entries that are labeled as regressions, 
are actually not regressions and have only been labeled as such due to noise.

### Viewing results

Look for significant changes (regressions or improvements) in the following:
- instruction count
- max rss

When working with graphs: 
- Click and drag a region of a graph to zoom in on it. This is useful when data
  points are close together.
- Click on a data point to open the "compare" page for that merge. This opens the comparison pages that are linked to in the generated triage report.

To view the code changes:
- Click on the "compare" link at the top of the measurements on that page to
  open the page of commits in the merge.

Understanding the comparison page:
- Each benchmark is listed with the min, max and the avg percentage change 
  (red indicates regressions, green indicates improvements) across the various 
  benchmarks run (e.g., full, incremental-full, incremental-unchanged, etc.).
- Clicking on a specific benchmark will show the results for each of the various
  benchmarks. Clicking on the percentages will open a more specific detail view 
  of timing for queries run during compilation.

### Interpreting results

*Warning*: max rss is much more variable than instruction count. Recheck the "Absolute
data" checkbox otherwise the noise becomes unmanageable.

- A change isn't significant unless one or more of the benchmarks changed by at
  least 1%.

#### Dealing with noise

Some benchmarks are noisy. Those that are known to be noisy typically have their results 
labeled as such with a `?`. These results can usually be dismissed if they are below 2.5% and are not accompanied by other regressions in other benchmarks. 

## Action

### Ping PR Author/Reviewer 

Single PR in Merge:
- Add a comment to the PR pointing to the "compare" page (unless someone else
  has already done that).
- In the case of a regression, ask the author for a response. If it's a big
  regression, consider requesting the author revert their changes. It may 
  be worth looking through the comments to see if any perf CI runs were done, 
  and whether the regression was expected.

Difficult cases: the merge was a rollup of multiple PRs.
- Look through the PRs and try to determine which was the cause. Often you
  can easily tell that one or more PRs could not have caused the change, e.g.
  because they made trivial changes, documentation-only changes, etc.
- If there are still PRs left over, look at the 'detailed-query' page on perf.rlo: often, there is a single timing pass that improved significantly, and the name may give you a hint. You can find the page by expanding the dropdown for the build with the greatest change, then clicking on the percent change. Note that this does not work for `-doc` builds.
- If you can't narrow it down to a single PR, in the rollup PR ask all the
  authors who might be responsible.
- Once you have narrowed it down to a single PR, treat it like an easy case,
  above.
- You might want to remind the author to use "@bors rollup=never" for PRs
  that are likely to affect performance.
- Add an entry to the triage log, as for the easy cases.

### Add analysis and follow-ups to report

- For each entry in the report, include useful details, such as the size of the regression/improvement, and any promises of follow-up action 
  from authors in the case of a regression.

### This Week in Rust 

Once finished, file a PR adding a link to the log entry in [This Week in
Rust](https://github.com/emberian/this-week-in-rust/).
- See the previous This Week in Rust edition for how the log entry should be formatted.

If you have any questions, the `t-compiler/performance` stream on Zulip is the
best place to ask.

