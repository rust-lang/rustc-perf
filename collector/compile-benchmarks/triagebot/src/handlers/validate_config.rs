//! For pull requests that have changed the triagebot.toml, validate that the
//! changes are a valid configuration file.
//! It won't validate anything unless the PR is open and has changed.

use crate::{
    config::{ValidateConfig, CONFIG_FILE_NAME},
    github::IssuesAction,
    handlers::{Context, IssuesEvent},
};
use tracing as log;

pub(super) async fn parse_input(
    ctx: &Context,
    event: &IssuesEvent,
    _config: Option<&ValidateConfig>,
) -> Result<Option<()>, String> {
    if !matches!(
        event.action,
        IssuesAction::Opened | IssuesAction::Reopened | IssuesAction::Synchronize
    ) {
        return Ok(None);
    }
    // All processing needs to be done in parse_input (instead of
    // handle_input) because we want this to *always* run. handle_input
    // requires the config to exist in triagebot.toml, but we want this to run
    // even if it isn't configured. As a consequence, error handling needs to
    // be a little more cautious here, since we don't want to relay
    // un-actionable errors to the user.
    let diff = match event.issue.diff(&ctx.github).await {
        Ok(Some(diff)) => diff,
        Ok(None) => return Ok(None),
        Err(e) => {
            log::error!("failed to get diff {e}");
            return Ok(None);
        }
    };
    if !diff.iter().any(|diff| diff.filename == CONFIG_FILE_NAME) {
        return Ok(None);
    }

    let Some(pr_source) = &event.issue.head else {
        log::error!("expected head commit in {event:?}");
        return Ok(None);
    };
    let Some(repo) = &pr_source.repo else {
        log::warn!("repo is not available in {event:?}");
        return Ok(None);
    };
    let triagebot_content = match ctx
        .github
        .raw_file(&repo.full_name, &pr_source.sha, CONFIG_FILE_NAME)
        .await
    {
        Ok(Some(c)) => c,
        Ok(None) => {
            log::error!("{CONFIG_FILE_NAME} modified, but failed to get content");
            return Ok(None);
        }
        Err(e) => {
            log::error!("failed to get {CONFIG_FILE_NAME}: {e}");
            return Ok(None);
        }
    };

    let triagebot_content = String::from_utf8_lossy(&*triagebot_content);
    if let Err(e) = toml::from_str::<crate::handlers::Config>(&triagebot_content) {
        let position = match e.span() {
            // toml sometimes gives bad spans, see https://github.com/toml-rs/toml/issues/589
            Some(span) if span != (0..0) => {
                let (line, col) = translate_position(&triagebot_content, span.start);
                let url = format!(
                    "https://github.com/{}/blob/{}/{CONFIG_FILE_NAME}#L{line}",
                    repo.full_name, pr_source.sha
                );
                format!(" at position [{line}:{col}]({url})",)
            }
            Some(_) | None => String::new(),
        };

        return Err(format!(
            "Invalid `triagebot.toml`{position}:\n\
            `````\n\
            {e}\n\
            `````",
        ));
    }
    Ok(None)
}

pub(super) async fn handle_input(
    _ctx: &Context,
    _config: &ValidateConfig,
    _event: &IssuesEvent,
    _input: (),
) -> anyhow::Result<()> {
    Ok(())
}

/// Helper to translate a toml span to a `(line_no, col_no)` (1-based).
fn translate_position(input: &str, index: usize) -> (usize, usize) {
    if input.is_empty() {
        return (0, index);
    }

    let safe_index = index.min(input.len() - 1);
    let column_offset = index - safe_index;

    let nl = input[0..safe_index]
        .as_bytes()
        .iter()
        .rev()
        .enumerate()
        .find(|(_, b)| **b == b'\n')
        .map(|(nl, _)| safe_index - nl - 1);
    let line_start = match nl {
        Some(nl) => nl + 1,
        None => 0,
    };
    let line = input[0..line_start]
        .as_bytes()
        .iter()
        .filter(|c| **c == b'\n')
        .count();
    let column = input[line_start..=safe_index].chars().count() - 1;
    let column = column + column_offset;

    (line + 1, column + 1)
}
