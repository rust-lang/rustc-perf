# Rust Compiler Performance Triage

We regularly triage the effects of merged PRs on rustc's speed and memory
usage.

## Roster

- ecstatic-morse
- Mark-Simulacrum
- rylev
- pnkfelix

Monday evening in North America is a good time to do it because This Week in Rust (see below) is usually
published on Tuesday, US time, and so it means the PR to include the triage
details in TWiR can be merged shortly before publication. This time is also
shortly before the weekly Rust compiler meeting, where the results are looked
at.

## Instructions

First, check the previous triage log entry. Look for responses in PRs, and
follow up on any promised actions. (i.e. nag people!)

While looking at the previous triage log entry, take note of the final commit in
the revision range from that triage period. The noted final commit `$PARENT` will
be the parent commit that we use for the current round of triage.

Start the new triage log entry in a new file using a `YYYY-MM-DD.md`-form name.
Follow the format of the previous entries.

Use the provided script to automate building the file:

```
% ./weekly-report.py PARENT > YYYY-MM-DD.md
```

You can also do it manually, starting by viewing the [perf website](https://perf.rust-lang.org).

- Determine the revision range. The start revision corresponds to the end
  revision from the previous log entry. The end revision is the latest measured
  revision on perf.rust-lang.org.
- Set the range on the graphs by setting the "start" and "end" revisions at
  the top of the page. Full SHAs must be used.
- Uncheck the "Absolute data" checkbox, because that makes changes easier to
  see.
- Record the revision range, with a link, in the log entry.

Look for significant changes (regressions or improvements) in the instruction
count graphs.
- Click and drag a region of a graph to zoom in on it. This is useful when data
  points are close together.
- Click on a data point to open the "compare" page for that merge.
- Click on the "compare" link at the top of the measurements on that page to
  open the page of commits in the merge.
- Alternatively, it may be easier to simply click through to the "compare" page
  for every revision in the range (there often aren't that many), rather than
  looking at the graphs.
- A change isn't significant unless one or more of the benchmarks changed by at
  least 1%.

Easy cases: there is only a single PR in the merge.
- Add a comment to the PR pointing to the "compare" page (unless someone else
  has already done that).
- In the case of a regression, ask the author for a response. If it's a big
  regression, consider requesting a backout. It may be worth looking through
  the comments to see if any perf CI runs were done, and whether the
  regression was expected.
- Add an entry to the triage log. Include the PR title and number, a link to
  the PR comment you added mentioning the performance effect, and a link to the
  performance results. Include useful details, such as the size of the
  regression/improvement, and any promises of follow-up action from authors in
  the case of a regression.

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

Repeat with the `max-rss` graphs. These measurements are much noisier than
`instructions:u`, so only larger changes will be clear; recheck the "Absolute
data" checkbox otherwise the noise becomes unmanageable.

Once finished, file a PR adding a link to the log entry in [This Week in
Rust](https://github.com/emberian/this-week-in-rust/).
- Add it within the "Updates from Rust Core" section.
- Add a "Rust Compiler Performance Triage" subsection immediately after the
  list of notable merged PRs.
- Within that subsection, add a list containing a single item.
- That item should be a link to the triage log entry with the form
  "YYYY-MM-DD", possibly with some brief text about notable things.

If you have any questions, the `t-compiler/performance` stream on Zulip is the
best place to ask.

