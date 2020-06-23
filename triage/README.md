# Rust Compiler Performance Triage

We regularly triage the effects of merged PRs on rustc's speed and memory
usage.

## Roster

- Tuesday morning (Melbourne time): njn

This is a good time to do it because This Week in Rust (see below) is usually
published on Tuesday, US time, and so it means the PR to include the triage
details in TWiR can be merged shortly before publication.

## Instructions

Check the most recent triage log entry. Look for responses in PRs, and follow
up on any promised actions. (I.e. nag people!)

Start a new triage log entry. Follow the format of the preceding entries.

Look through the [`instructions:u` graphs](https://perf.rust-lang.org) for each
benchmark.
- Record the hash of the latest measured revision in the log entry.
- Set the "start" revision to the previous log entry's latest revision, and the
  "end" revision to the latest revision. Full SHAs must be used.
- Uncheck the "Absolute data" checkbox, because that makes changes easier to
  see.

Look for significant changes (regressions or improvements) in the graphs.
- Click and drag a region of a graph to zoom in on it. This is useful when data
  points are close together.
- Click on a data point to open the "compare" page for that merge.
- Click on the "compare" link at the top of the measurements on that page to
  open the page of commits in the merge.

Easy cases: there is only a single PR in the merge.
- Add a comment to the PR pointing to the "compare" page (unless someone else
  has already done that).
- In the case of a regression, ask the author for a response. If it's a big
  regression, consider requesting a backout. It may be worth looking through
  the comments to see if any perf CI runs were done, and whether the
  regression was expected.
- Add an entry to the triage log. Include the PR title, a link to it, and a
  link to the performance results. Include useful details, such as the size of
  the regression/improvement, and any promises of follow-up action from authors
  in the case of a regression.

Difficult cases: the merge was a rollup of multiple PRs.
- Look through the PRs and try to determine which was the cause. Often you
  can easily tell that one or more PRs could not have caused the change, e.g.
  because they made trivial changes, documentation-only changes, etc.
- If you can't narrow it down to a single PR, in the rollup PR ask all the
  authors who might be responsible.
- Once you have narrowed it down to a single PR, treat it like an easy case,
  above.
- You might want to remind the author to use "@bors rollup=never" for PRs
  that are likely to affect performance.
- Add an entry to the triage log, as for the easy cases.

Repeat with the [`max-rss`
graphs](https://perf.rust-lang.org/?start=&end=&absolute=true&stat=max-rss).
These measurements are much noisier than `instructions:u`, so only larger
changes will be clear.

Once finished, file a PR adding a link to the log entry in [This Week in
Rust](https://github.com/emberian/this-week-in-rust/).
- Add it within the "Updates from Rust Core" section.
- Add a "Rust Compiler Performance Triage" subsection immediately after the
  list of notable merged PRs.
- Within that subsection, add a list containing a single item.
- That item should be a link to the triage log entry with the form
  "YYYY-MM-DD", possibly with some brief text about notable things.

If you have any questions, the #wg-compiler-performance channel on Discord and
the t-compiler stream on Zulip are good places to ask.

## Triage logs

- [2020](2020.md)

