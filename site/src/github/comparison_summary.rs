use crate::comparison::{
    deserves_attention_icount, write_summary_table, ArtifactComparison, ArtifactComparisonSummary,
    Direction, Metric,
};
use crate::load::SiteCtxt;

use database::{ArtifactId, QueuedCommit};

use std::collections::HashSet;
use std::fmt::Write;

/// Post messages to GitHub for all queued commits that have
/// not yet been marked as completed.
pub async fn post_finished(ctxt: &SiteCtxt) {
    // If the github token is not configured, do not run this -- we don't want
    // to mark things as complete without posting the comment.
    if ctxt.config.keys.github_api_token.is_none() {
        return;
    }
    let conn = ctxt.conn().await;
    let index = ctxt.index.load();
    let mut known_commits = index
        .commits()
        .into_iter()
        .map(|c| c.sha.to_string())
        .collect::<HashSet<_>>();
    let (master_commits, queued_pr_commits, in_progress_artifacts) = futures::join!(
        collector::master_commits(),
        conn.queued_commits(),
        conn.in_progress_artifacts()
    );
    let master_commits = match master_commits {
        Ok(mcs) => mcs.into_iter().map(|c| c.sha).collect::<HashSet<_>>(),
        Err(e) => {
            log::error!("posting finished did not load master commits: {:?}", e);
            // If we can't fetch master commits, return.
            // We'll eventually try again later
            return;
        }
    };

    for aid in in_progress_artifacts {
        match aid {
            ArtifactId::Commit(c) => {
                known_commits.remove(&c.sha);
            }
            ArtifactId::Tag(_) => {
                // do nothing, for now, though eventually we'll want an artifact queue
            }
        }
    }
    for queued_commit in queued_pr_commits
        .into_iter()
        .filter(|c| known_commits.contains(&c.sha))
    {
        if let Some(completed) = conn.mark_complete(&queued_commit.sha).await {
            assert_eq!(completed, queued_commit);

            let is_master_commit = master_commits.contains(&queued_commit.sha);
            post_comparison_comment(ctxt, queued_commit, is_master_commit).await;
        }
    }
}

/// Posts a comment to GitHub summarizing the comparison of the queued commit with its parent
///
/// `is_master_commit` is used to differentiate messages for try runs and post-merge runs.
async fn post_comparison_comment(ctxt: &SiteCtxt, commit: QueuedCommit, is_master_commit: bool) {
    let client = super::client::Client::from_ctxt(
        ctxt,
        "https://api.github.com/repos/rust-lang/rust".to_owned(),
    );
    let pr = commit.pr;
    let body = match summarize_run(ctxt, commit, is_master_commit).await {
        Ok(message) => message,
        Err(error) => error,
    };

    client.post_comment(pr, body).await;
}

fn make_comparison_url(commit: &QueuedCommit, stat: Metric) -> String {
    format!(
        "https://perf.rust-lang.org/compare.html?start={}&end={}&stat={}",
        commit.parent_sha,
        commit.sha,
        stat.as_str()
    )
}

async fn calculate_metric_comparison(
    ctxt: &SiteCtxt,
    commit: &QueuedCommit,
    metric: Metric,
) -> Result<ArtifactComparison, String> {
    match crate::comparison::compare(
        collector::Bound::Commit(commit.parent_sha.clone()),
        collector::Bound::Commit(commit.sha.clone()),
        metric,
        ctxt,
    )
    .await
    {
        Ok(Some(c)) => Ok(c),
        _ => Err("ERROR categorizing benchmark run!".to_owned()),
    }
}

async fn summarize_run(
    ctxt: &SiteCtxt,
    commit: QueuedCommit,
    is_master_commit: bool,
) -> Result<String, String> {
    let benchmark_map = ctxt.get_benchmark_category_map().await;

    let mut message = format!(
        "Finished benchmarking commit ({sha}): [comparison URL]({comparison_url}).\n\n",
        sha = commit.sha,
        comparison_url = make_comparison_url(&commit, Metric::InstructionsUser)
    );

    let inst_comparison =
        calculate_metric_comparison(ctxt, &commit, Metric::InstructionsUser).await?;

    let errors = if !inst_comparison.newly_failed_benchmarks.is_empty() {
        let benchmarks = inst_comparison
            .newly_failed_benchmarks
            .iter()
            .map(|(benchmark, _)| format!("- {benchmark}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("\n**Warning ⚠**: The following benchmark(s) failed to build:\n{benchmarks}\n")
    } else {
        String::new()
    };

    let (inst_primary, inst_secondary) = inst_comparison
        .clone()
        .summarize_by_category(&benchmark_map);

    let direction = inst_primary.direction().join(inst_secondary.direction());
    let overall_result = match direction {
        Direction::Improvement => "✅ improvements",
        Direction::Regression => "❌ regressions",
        Direction::Mixed => "❌✅ regressions and improvements",
        Direction::None => "no relevant changes",
    };
    let is_regression = deserves_attention_icount(&inst_primary, &inst_secondary)
        && matches!(direction, Direction::Regression | Direction::Mixed);

    writeln!(
        &mut message,
        "### Overall result: {}{}\n",
        overall_result,
        if is_regression {
            " - ACTION NEEDED"
        } else {
            " - no action needed"
        },
    )
    .unwrap();

    let next_steps = if is_master_commit {
        master_run_body(is_regression)
    } else {
        try_run_body(is_regression)
    };
    writeln!(&mut message, "{next_steps}\n").unwrap();

    if !errors.is_empty() {
        writeln!(&mut message, "\n{errors}").unwrap();
        if is_master_commit {
            writeln!(&mut message, "\ncc @rust-lang/wg-compiler-performance").unwrap();
        }
    }

    let metrics = vec![
        (
            "Instruction count",
            Metric::InstructionsUser,
            true, // highly reliable
            inst_comparison,
        ),
        (
            "Max RSS (memory usage)",
            Metric::MaxRSS,
            false, // not highly reliable
            calculate_metric_comparison(ctxt, &commit, Metric::MaxRSS).await?,
        ),
        (
            "Cycles",
            Metric::CyclesUser,
            false, // not highly reliable
            calculate_metric_comparison(ctxt, &commit, Metric::CyclesUser).await?,
        ),
    ];

    for (title, metric, highly_reliable, comparison) in metrics {
        message.push_str(&format!(
            "\n### [{title}]({})\n",
            make_comparison_url(&commit, metric)
        ));

        let (primary, secondary) = comparison.summarize_by_category(&benchmark_map);
        write_metric_summary(primary, secondary, highly_reliable, &mut message);
    }

    Ok(message)
}

fn write_metric_summary(
    primary: ArtifactComparisonSummary,
    secondary: ArtifactComparisonSummary,
    highly_reliable: bool,
    message: &mut String,
) {
    if !primary.is_relevant() && !secondary.is_relevant() {
        message
            .push_str("This benchmark run did not return any relevant results for this metric.\n");
    } else {
        if highly_reliable {
            message.push_str(
                "This is a highly reliable metric that was used to determine the \
                overall result at the top of this comment.\n\n",
            );
            write_summary_table(&primary, &secondary, false, message);
        } else {
            // `<details>` means it is hidden, requiring a click to reveal.
            message.push_str("<details>\n<summary>Results</summary>\n\n");
            message.push_str(
                "This is a less reliable metric that may be of interest but was not \
                used to determine the overall result at the top of this comment.\n\n",
            );
            write_summary_table(&primary, &secondary, false, message);
            message.push_str("</details>\n");
        }
    }
}

fn master_run_body(is_regression: bool) -> String {
    if is_regression {
        "
**Next Steps**: If you can justify the \
regressions found in this perf run, please indicate this with \
`@rustbot label: +perf-regression-triaged` along with \
sufficient written justification. If you cannot justify the regressions \
please open an issue or create a new PR that fixes the regressions, \
add a comment linking to the newly created issue or PR, \
and then add the `perf-regression-triaged` label to this PR.

@rustbot label: +perf-regression
cc @rust-lang/wg-compiler-performance
"
    } else {
        "
@rustbot label: -perf-regression
"
    }
    .to_string()
}

fn try_run_body(is_regression: bool) -> String {
    let next_steps = if is_regression {
        "\n\n**Next Steps**: If you can justify the regressions found in \
            this try perf run, please indicate this with \
            `@rustbot label: +perf-regression-triaged` along with \
            sufficient written justification. If you cannot justify the regressions \
            please fix the regressions and do another perf run. If the next run \
            shows neutral or positive results, the label will be automatically removed."
    } else {
        ""
    };

    let sign = if is_regression { "+" } else { "-" };
    format!(
        "
Benchmarking this pull request likely means that it is \
perf-sensitive, so we're automatically marking it as not fit \
for rolling up. While you can manually mark this PR as fit \
for rollup, we strongly recommend not doing so since this PR may lead to changes in \
compiler perf.{next_steps}

@bors rollup=never
@rustbot label: +S-waiting-on-review -S-waiting-on-perf {sign}perf-regression",
    )
}
