use crate::github::client::{Client, GraphQLClient, ResponseComment};
use crate::request_handlers::parse_unrolled_build_message;
use anyhow::bail;
use database::QueuedCommit;

pub struct TriageBuild {
    pub rollup_pr_number: u32,
    pub triage_comment: ResponseComment,
}

pub const TRIAGE_MARKER: &str = "<!-- rust-timer: triage -->";

pub fn triage_body_start_marker(pr: u32) -> String {
    format!("<!-- rust-timer:triage-body-start-pr-{pr} -->\n")
}
pub fn triage_body_end_marker(pr: u32) -> String {
    format!("<!-- rust-timer:triage-body-end-pr-{pr} -->\n")
}

/// Returns `Some` if this commit is part of a triage run.
pub async fn is_triage_run(
    commit: &QueuedCommit,
    client: &mut Client,
    graph_qlclient: &mut GraphQLClient,
) -> anyhow::Result<Option<TriageBuild>> {
    // Find the rollup PR
    let commit_title = client.get_commit(&commit.sha).await?;
    let Ok(unrolled_build) = parse_unrolled_build_message(&commit_title.commit.message) else {
        // The commit title parsed successfully during the `@rust-timer triage` command, so if it does not parse here it's not a triage run
        return Ok(None);
    };

    // Find the triage run on the rollup PR
    let rollup_comments = graph_qlclient
        .get_comments(unrolled_build.rollup_pr_number)
        .await?;
    let Some(triage_comment) = rollup_comments.into_iter().rev().find(|c| {
        c.viewer_did_author && c.body.contains(TRIAGE_MARKER) && c.body.contains(&commit.sha)
    }) else {
        // This was a try job on the unrolled build that did not originate from a triage command
        return Ok(None);
    };

    Ok(Some(TriageBuild {
        rollup_pr_number: unrolled_build.rollup_pr_number,
        triage_comment,
    }))
}

pub fn update_triage_body(
    body: &mut String,
    pr: u32,
    triage_summary: String,
) -> anyhow::Result<()> {
    let Some(body_start) = body.find(&triage_body_start_marker(pr)) else {
        bail!("Failed to triage body start")
    };
    let body_start = body_start + triage_body_start_marker(pr).len();
    let Some(body_end) = body.find(&triage_body_end_marker(pr)) else {
        bail!("Failed to triage body end")
    };

    body.replace_range(body_start..body_end, &triage_summary);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::github::triage::update_triage_body;

    const BEFORE_BODY: &str = "### #157428 364c9dea926885d60daef4bf1a15fa03efa35a84 allocator: refactor for stabilisation
<!-- rust-timer:triage-body-start-pr-157428 -->
Queued 364c9dea926885d60daef4bf1a15fa03efa35a84 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=364c9dea926885d60daef4bf1a15fa03efa35a84).
There are currently 0 preceding artifacts in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~1.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-157428 -->

### #160077 a96bde1411c3b0fe68417c04e85624f33b645516 Don't compute FnAbi for LLVM intrinsics
<!-- rust-timer:triage-body-start-pr-160077 -->
Queued a96bde1411c3b0fe68417c04e85624f33b645516 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=a96bde1411c3b0fe68417c04e85624f33b645516).
There is currently 1 preceding artifact in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~2.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-160077 -->

### #160288 77cb04ee49e4f29833262112d5abf705c96faa98 rustdoc: use anonymous constant for primitives/keywords/attribute docs
<!-- rust-timer:triage-body-start-pr-160288 -->
Queued 77cb04ee49e4f29833262112d5abf705c96faa98 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=77cb04ee49e4f29833262112d5abf705c96faa98).
There are currently 2 preceding artifacts in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~3.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-160288 -->

<!-- rust-timer: triage -->";

    #[test]
    fn test_update_first() {
        let mut body = BEFORE_BODY.to_string();
        update_triage_body(
            &mut body,
            157428,
            "NEW BODY1\nNEW BODY2\nNEW BODY3\n".to_string(),
        )
        .unwrap();

        const EXPECTED_RESULT: &str = "### #157428 364c9dea926885d60daef4bf1a15fa03efa35a84 allocator: refactor for stabilisation
<!-- rust-timer:triage-body-start-pr-157428 -->
NEW BODY1
NEW BODY2
NEW BODY3
<!-- rust-timer:triage-body-end-pr-157428 -->

### #160077 a96bde1411c3b0fe68417c04e85624f33b645516 Don't compute FnAbi for LLVM intrinsics
<!-- rust-timer:triage-body-start-pr-160077 -->
Queued a96bde1411c3b0fe68417c04e85624f33b645516 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=a96bde1411c3b0fe68417c04e85624f33b645516).
There is currently 1 preceding artifact in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~2.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-160077 -->

### #160288 77cb04ee49e4f29833262112d5abf705c96faa98 rustdoc: use anonymous constant for primitives/keywords/attribute docs
<!-- rust-timer:triage-body-start-pr-160288 -->
Queued 77cb04ee49e4f29833262112d5abf705c96faa98 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=77cb04ee49e4f29833262112d5abf705c96faa98).
There are currently 2 preceding artifacts in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~3.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-160288 -->

<!-- rust-timer: triage -->";
        assert_eq!(body, EXPECTED_RESULT);
    }

    #[test]
    fn test_update_second() {
        let mut body = BEFORE_BODY.to_string();
        update_triage_body(
            &mut body,
            160077,
            "NEW BODY1\nNEW BODY2\nNEW BODY3\n".to_string(),
        )
        .unwrap();

        const EXPECTED_RESULT: &str = "### #157428 364c9dea926885d60daef4bf1a15fa03efa35a84 allocator: refactor for stabilisation
<!-- rust-timer:triage-body-start-pr-157428 -->
Queued 364c9dea926885d60daef4bf1a15fa03efa35a84 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=364c9dea926885d60daef4bf1a15fa03efa35a84).
There are currently 0 preceding artifacts in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~1.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-157428 -->

### #160077 a96bde1411c3b0fe68417c04e85624f33b645516 Don't compute FnAbi for LLVM intrinsics
<!-- rust-timer:triage-body-start-pr-160077 -->
NEW BODY1
NEW BODY2
NEW BODY3
<!-- rust-timer:triage-body-end-pr-160077 -->

### #160288 77cb04ee49e4f29833262112d5abf705c96faa98 rustdoc: use anonymous constant for primitives/keywords/attribute docs
<!-- rust-timer:triage-body-start-pr-160288 -->
Queued 77cb04ee49e4f29833262112d5abf705c96faa98 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=77cb04ee49e4f29833262112d5abf705c96faa98).
There are currently 2 preceding artifacts in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~3.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-160288 -->

<!-- rust-timer: triage -->";
        assert_eq!(body, EXPECTED_RESULT);
    }

    #[test]
    fn test_update_last() {
        let mut body = BEFORE_BODY.to_string();
        update_triage_body(
            &mut body,
            160288,
            "NEW BODY1\nNEW BODY2\nNEW BODY3\n".to_string(),
        )
        .unwrap();

        const EXPECTED_RESULT: &str = "### #157428 364c9dea926885d60daef4bf1a15fa03efa35a84 allocator: refactor for stabilisation
<!-- rust-timer:triage-body-start-pr-157428 -->
Queued 364c9dea926885d60daef4bf1a15fa03efa35a84 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=364c9dea926885d60daef4bf1a15fa03efa35a84).
There are currently 0 preceding artifacts in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~1.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-157428 -->

### #160077 a96bde1411c3b0fe68417c04e85624f33b645516 Don't compute FnAbi for LLVM intrinsics
<!-- rust-timer:triage-body-start-pr-160077 -->
Queued a96bde1411c3b0fe68417c04e85624f33b645516 with parent 93c9086fdd5b80d286480a19ac047746ecc5fa1f, future [comparison URL](https://perf.rust-lang.org/compare.html?start=93c9086fdd5b80d286480a19ac047746ecc5fa1f&end=a96bde1411c3b0fe68417c04e85624f33b645516).
There is currently 1 preceding artifact in the [queue](https://perf.rust-lang.org/status.html).
It will probably take at least ~2.0 hours until the benchmark run finishes.
<!-- rust-timer:triage-body-end-pr-160077 -->

### #160288 77cb04ee49e4f29833262112d5abf705c96faa98 rustdoc: use anonymous constant for primitives/keywords/attribute docs
<!-- rust-timer:triage-body-start-pr-160288 -->
NEW BODY1
NEW BODY2
NEW BODY3
<!-- rust-timer:triage-body-end-pr-160288 -->

<!-- rust-timer: triage -->";
        assert_eq!(body, EXPECTED_RESULT);
    }

    #[test]
    fn test_update_not_in_there() {
        let mut body = BEFORE_BODY.to_string();
        assert!(update_triage_body(
            &mut body,
            1234,
            "NEW BODY1\nNEW BODY2\nNEW BODY3\n".to_string()
        )
        .is_err());
    }
}
