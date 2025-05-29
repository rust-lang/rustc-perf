//! Purpose: Allow any user to modify issue labels on GitHub via comments.
//!
//! Labels are checked against the labels in the project; the bot does not support creating new
//! labels.
//!
//! Parsing is done in the `parser::command::relabel` module.
//!
//! If the command was successful, there will be no feedback beyond the label change to reduce
//! notification noise.

use crate::{
    config::RelabelConfig,
    github::UnknownLabels,
    github::{self, Event, GithubClient},
    handlers::Context,
    interactions::ErrorComment,
};
use parser::command::relabel::{LabelDelta, RelabelCommand};

pub(super) async fn handle_command(
    ctx: &Context,
    config: &RelabelConfig,
    event: &Event,
    input: RelabelCommand,
) -> anyhow::Result<()> {
    let mut results = vec![];
    let mut to_add = vec![];
    for delta in &input.0 {
        let name = delta.label().as_str();
        let err = match check_filter(name, config, is_member(&event.user(), &ctx.github).await) {
            Ok(CheckFilterResult::Allow) => None,
            Ok(CheckFilterResult::Deny) => Some(format!(
                "Label {} can only be set by Rust team members",
                name
            )),
            Ok(CheckFilterResult::DenyUnknown) => Some(format!(
                "Label {} can only be set by Rust team members;\
                 we were unable to check if you are a team member.",
                name
            )),
            Err(err) => Some(err),
        };
        if let Some(msg) = err {
            let cmnt = ErrorComment::new(&event.issue().unwrap(), msg);
            cmnt.post(&ctx.github).await?;
            return Ok(());
        }
        match delta {
            LabelDelta::Add(label) => {
                to_add.push(github::Label {
                    name: label.to_string(),
                });
            }
            LabelDelta::Remove(label) => {
                results.push((
                    label,
                    event.issue().unwrap().remove_label(&ctx.github, &label),
                ));
            }
        }
    }

    if let Err(e) = event
        .issue()
        .unwrap()
        .add_labels(&ctx.github, to_add.clone())
        .await
    {
        tracing::error!(
            "failed to add {:?} from issue {}: {:?}",
            to_add,
            event.issue().unwrap().global_id(),
            e
        );
        if let Some(err @ UnknownLabels { .. }) = e.downcast_ref() {
            event
                .issue()
                .unwrap()
                .post_comment(&ctx.github, &err.to_string())
                .await?;
        }

        return Err(e);
    }

    for (label, res) in results {
        if let Err(e) = res.await {
            tracing::error!(
                "failed to remove {:?} from issue {}: {:?}",
                label,
                event.issue().unwrap().global_id(),
                e
            );
            return Err(e);
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum TeamMembership {
    Member,
    Outsider,
    Unknown,
}

async fn is_member(user: &github::User, client: &GithubClient) -> TeamMembership {
    match user.is_team_member(client).await {
        Ok(true) => TeamMembership::Member,
        Ok(false) => TeamMembership::Outsider,
        Err(err) => {
            eprintln!("failed to check team membership: {:?}", err);
            TeamMembership::Unknown
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
enum CheckFilterResult {
    Allow,
    Deny,
    DenyUnknown,
}

fn check_filter(
    label: &str,
    config: &RelabelConfig,
    is_member: TeamMembership,
) -> Result<CheckFilterResult, String> {
    if is_member == TeamMembership::Member {
        return Ok(CheckFilterResult::Allow);
    }
    let mut matched = false;
    for pattern in &config.allow_unauthenticated {
        match match_pattern(pattern, label) {
            Ok(MatchPatternResult::Allow) => matched = true,
            Ok(MatchPatternResult::Deny) => {
                // An explicit deny overrides any allowed pattern
                matched = false;
                break;
            }
            Ok(MatchPatternResult::NoMatch) => {}
            Err(err) => {
                eprintln!("failed to match pattern {}: {}", pattern, err);
                return Err(format!("failed to match pattern {}", pattern));
            }
        }
    }
    if matched {
        return Ok(CheckFilterResult::Allow);
    } else if is_member == TeamMembership::Outsider {
        return Ok(CheckFilterResult::Deny);
    } else {
        return Ok(CheckFilterResult::DenyUnknown);
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
enum MatchPatternResult {
    Allow,
    Deny,
    NoMatch,
}

fn match_pattern(pattern: &str, label: &str) -> anyhow::Result<MatchPatternResult> {
    let (pattern, inverse) = if pattern.starts_with('!') {
        (&pattern[1..], true)
    } else {
        (pattern, false)
    };

    let glob = glob::Pattern::new(pattern)?;
    let mut matchopts = glob::MatchOptions::default();
    matchopts.case_sensitive = false;

    Ok(match (glob.matches_with(label, matchopts), inverse) {
        (true, false) => MatchPatternResult::Allow,
        (true, true) => MatchPatternResult::Deny,
        (false, _) => MatchPatternResult::NoMatch,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        check_filter, match_pattern, CheckFilterResult, MatchPatternResult, TeamMembership,
    };
    use crate::config::RelabelConfig;

    #[test]
    fn test_match_pattern() -> anyhow::Result<()> {
        assert_eq!(
            match_pattern("I-*", "I-nominated")?,
            MatchPatternResult::Allow
        );
        assert_eq!(
            match_pattern("i-*", "I-nominated")?,
            MatchPatternResult::Allow
        );
        assert_eq!(
            match_pattern("!I-no*", "I-nominated")?,
            MatchPatternResult::Deny
        );
        assert_eq!(
            match_pattern("I-*", "T-infra")?,
            MatchPatternResult::NoMatch
        );
        assert_eq!(
            match_pattern("!I-no*", "T-infra")?,
            MatchPatternResult::NoMatch
        );
        Ok(())
    }

    #[test]
    fn test_check_filter() -> anyhow::Result<()> {
        macro_rules! t {
            ($($member:ident { $($label:expr => $res:ident,)* })*) => {
                let config = RelabelConfig {
                    allow_unauthenticated: vec!["T-*".into(), "I-*".into(), "!I-*nominated".into()],
                };
                $($(assert_eq!(
                    check_filter($label, &config, TeamMembership::$member),
                    Ok(CheckFilterResult::$res)
                );)*)*
            }
        }
        t! {
            Member {
                "T-release" => Allow,
                "I-slow" => Allow,
                "I-lang-nominated" => Allow,
                "I-nominated" => Allow,
                "A-spurious" => Allow,
            }
            Outsider {
                "T-release" => Allow,
                "I-slow" => Allow,
                "I-lang-nominated" => Deny,
                "I-nominated" => Deny,
                "A-spurious" => Deny,
            }
            Unknown {
                "T-release" => Allow,
                "I-slow" => Allow,
                "I-lang-nominated" => DenyUnknown,
                "I-nominated" => DenyUnknown,
                "A-spurious" => DenyUnknown,
            }
        }
        Ok(())
    }
}
