# Rust Compiler Performance Triage

We regularly triage the effects of merged PRs on rustc's speed and memory
usage.

## Roster

- Mark-Simulacrum
- rylev
- pnkfelix
- kobzol

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
% curl "https://perf.rust-lang.org/perf/triage?start=$PARENT" > YYYY-MM-DD.md
```

You can also analyze binary size regressions/improvements using the following command:
```
% curl "https://perf.rust-lang.org/perf/triage?start=$PARENT&metric=size:linked_artifact" > binary-size.md
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
- binary size

When working with graphs: 
- Click and drag a region of a graph to zoom in on it. This is useful when data
  points are close together.
- Click on a data point to open the "compare" page for that merge. This opens the comparison pages that are linked to in the generated triage report.

To view the code changes:
- Click on the "compare" link at the top of the measurements on that page to
  open the page of commits in the merge.

Understanding the comparison page:
- Each benchmark is listed with the percentage change 
  (red indicates regressions, green indicates improvements) across the various 
  benchmarks run (e.g., full, incremental-full, incremental-unchanged, etc.).
- Clicking on a specific benchmark run will show a detailed view of the results, including
  history chart and links to self-profile query timings.

### Interpreting results

*Warning*: max rss is much more variable than instruction count. Recheck the "Absolute
data" checkbox otherwise the noise becomes unmanageable.

For help understanding how to interpret results, consult the [comparison analysis documentation](../docs/comparison-analysis.md).

## Action

### Ping PR Author/Reviewer 

Single PR in Merge:
- In the case of a regression, ask the author for a response. If it's a big
  regression, consider requesting the author revert their changes. It may 
  be worth looking through the comments to see if the regression was expected.

Difficult cases: the merge was a rollup of multiple PRs.
- Look through the PRs and try to determine which was the cause. You can start
  a perf. run for a single PR merged in the rollup using the "unrolled build"
  table (see e.g. [here](https://github.com/rust-lang/rust/pull/119313#issuecomment-1869441617)) with
  the `@rust-timer build $SHA` command.
- Often you can easily tell that one or more PRs could not have caused the change, e.g.
  because they made trivial changes, documentation-only changes, etc., so start with the
  perf. runs for the most "suspicious" PRs.
- Once you have narrowed it down to a single PR, treat it like a single PR case, see above.
- You might want to remind the author to use "@bors rollup=never" for PRs
  that are likely to affect performance.
- Add an entry to the triage log, as for the single PR cases.

### Add analysis and follow-ups to report

- For each entry in the report, include useful details, such as the size of the regression/improvement,
  and any promises of follow-up action from authors in the case of a regression.

### This Week in Rust 

Once finished, file a PR adding a link to the log entry in [This Week in
Rust](https://github.com/emberian/this-week-in-rust/).
- See the previous This Week in Rust edition for how the log entry should be formatted.

After you have finished the triage, also post a short summary to the
[`t-compiler/performance`](https://rust-lang.zulipchat.com/#narrow/stream/247081-t-compiler.2Fperformance)
stream on Zulip. If you have any questions, you can ask around in that stream.
