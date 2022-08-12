use crate::comparison::{
    deserves_attention_icount, write_summary_table, write_summary_table_footer, ArtifactComparison,
    ArtifactComparisonSummary, Direction, Metric,
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
        "Finished benchmarking commit ({sha}): [comparison url]({comparison_url}).\n\n",
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

    let mut table_written = false;
    let metrics = vec![
        (
            "Instruction count",
            Metric::InstructionsUser,
            false,
            inst_comparison,
        ),
        (
            "Max RSS (memory usage)",
            Metric::MaxRSS,
            true,
            calculate_metric_comparison(ctxt, &commit, Metric::MaxRSS).await?,
        ),
        (
            "Cycles",
            Metric::CyclesUser,
            true,
            calculate_metric_comparison(ctxt, &commit, Metric::CyclesUser).await?,
        ),
    ];

    for (title, metric, hidden, comparison) in metrics {
        message.push_str(&format!(
            "\n### [{title}]({})\n",
            make_comparison_url(&commit, metric)
        ));

        let (primary, secondary) = comparison.summarize_by_category(&benchmark_map);
        table_written |= write_metric_summary(primary, secondary, hidden, &mut message);
    }

    if table_written {
        write_summary_table_footer(&mut message);
    }

    const DISAGREEMENT: &str = "If you disagree with this performance assessment, \
    please file an issue in [rust-lang/rustc-perf](https://github.com/rust-lang/rustc-perf/issues/new).";
    let footer = format!("{DISAGREEMENT}{errors}");

    let direction = Direction::join(inst_primary.direction(), inst_secondary.direction());
    let next_steps = next_steps(inst_primary, inst_secondary, direction, is_master_commit);

    write!(&mut message, "\n{footer}\n{next_steps}").unwrap();

    Ok(message)
}

/// Returns true if a summary table was written to `message`.
fn write_metric_summary(
    primary: ArtifactComparisonSummary,
    secondary: ArtifactComparisonSummary,
    hidden: bool,
    message: &mut String,
) -> bool {
    if !primary.is_relevant() && !secondary.is_relevant() {
        message
            .push_str("This benchmark run did not return any relevant results for this metric.\n");
        false
    } else {
        let primary_short_summary = generate_short_summary(&primary);
        let secondary_short_summary = generate_short_summary(&secondary);

        if hidden {
            message.push_str("<details>\n<summary>Results</summary>\n\n");
        }

        write!(
            message,
            r#"
- Primary benchmarks: {primary_short_summary}
- Secondary benchmarks: {secondary_short_summary}

"#
        )
        .unwrap();

        write_summary_table(&primary, &secondary, true, message);

        if hidden {
            message.push_str("</details>\n");
        }

        true
    }
}

fn next_steps(
    primary: ArtifactComparisonSummary,
    secondary: ArtifactComparisonSummary,
    direction: Direction,
    is_master_commit: bool,
) -> String {
    let deserves_attention = deserves_attention_icount(&primary, &secondary);
    let (is_regression, label) = match (deserves_attention, direction) {
        (true, Direction::Regression | Direction::Mixed) => (true, "+perf-regression"),
        _ => (false, "-perf-regression"),
    };

    if is_master_commit {
        master_run_body(is_regression)
    } else {
        try_run_body(label)
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

fn try_run_body(label: &str) -> String {
    let next_steps = if label.starts_with("+") {
        "\n\n**Next Steps**: If you can justify the regressions found in \
            this try perf run, please indicate this with \
            `@rustbot label: +perf-regression-triaged` along with \
            sufficient written justification. If you cannot justify the regressions \
            please fix the regressions and do another perf run. If the next run \
            shows neutral or positive results, the label will be automatically removed."
    } else {
        ""
    };

    format!(
        "
Benchmarking this pull request likely means that it is \
perf-sensitive, so we're automatically marking it as not fit \
for rolling up. While you can manually mark this PR as fit \
for rollup, we strongly recommend not doing so since this PR may lead to changes in \
compiler perf.{next_steps}

@bors rollup=never
@rustbot label: +S-waiting-on-review -S-waiting-on-perf {label}",
    )
}

fn generate_short_summary(summary: &ArtifactComparisonSummary) -> String {
    // Add an "s" to a word unless there's only one.
    fn ending(word: &'static str, count: usize) -> std::borrow::Cow<'static, str> {
        if count == 1 {
            return word.into();
        }
        format!("{}s", word).into()
    }

    let num_improvements = summary.number_of_improvements();
    let num_regressions = summary.number_of_regressions();

    match summary.direction() {
        Direction::Improvement => format!(
            "✅ relevant {} found",
            ending("improvement", num_improvements)
        ),
        Direction::Regression => format!(
            "❌ relevant {} found",
            ending("regression", num_regressions)
        ),
        Direction::Mixed => "mixed results".to_string(),
        Direction::None => "no relevant changes found".to_string(),
    }
}
