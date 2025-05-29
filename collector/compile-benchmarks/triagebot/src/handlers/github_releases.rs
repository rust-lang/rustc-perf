use crate::{
    changelogs::Changelog,
    config::GitHubReleasesConfig,
    github::{CreateEvent, CreateKind, Event},
    handlers::Context,
};
use anyhow::Context as _;
use octocrab::Page;
use std::{collections::HashMap, time::Duration};
use tracing as log;

pub(super) async fn handle(
    ctx: &Context,
    event: &Event,
    config: &GitHubReleasesConfig,
) -> anyhow::Result<()> {
    // Only allow commit pushed to the changelog branch or tags being created.
    match event {
        Event::Push(push) if push.git_ref == format!("refs/heads/{}", config.changelog_branch) => {}
        Event::Create(CreateEvent {
            ref_type: CreateKind::Tag,
            ..
        }) => {}
        _ => return Ok(()),
    }

    log::info!("handling github releases");

    log::debug!("loading the changelog");
    let content = load_changelog(ctx, event, config).await.with_context(|| {
        format!(
            "failed to load changelog file {} from repo {} in branch {}",
            config.changelog_path,
            event.repo().full_name,
            config.changelog_branch
        )
    })?;
    let changelog = Changelog::parse(config.format, &content)?;

    log::debug!("loading the git tags");
    let tags = load_paginated(
        ctx,
        &format!("/repos/{}/git/matching-refs/tags", event.repo().full_name),
        |git_ref: &GitRef| {
            git_ref
                .name
                .strip_prefix("refs/tags/")
                .unwrap_or(git_ref.name.as_str())
                .to_string()
        },
    )
    .await?;

    log::debug!("loading the existing releases");
    let releases = load_paginated(
        ctx,
        &format!("/repos/{}/releases", event.repo().full_name),
        |release: &Release| release.tag_name.clone(),
    )
    .await?;

    for tag in tags.keys() {
        if let Some(expected_body) = changelog.version(tag) {
            let expected_name = format!("{} {}", config.project_name, tag);

            if let Some(release) = releases.get(tag) {
                if release.name != expected_name || release.body != expected_body {
                    log::info!("updating release {} on {}", tag, event.repo().full_name);
                    let _: serde_json::Value = ctx
                        .octocrab
                        .patch(
                            &release.url,
                            Some(&serde_json::json!({
                                "name": expected_name,
                                "body": expected_body,
                            })),
                        )
                        .await?;
                } else {
                    // Avoid waiting for the delay below.
                    continue;
                }
            } else {
                log::info!("creating release {} on {}", tag, event.repo().full_name);
                let e: octocrab::Result<serde_json::Value> = ctx
                    .octocrab
                    .post(
                        format!("/repos/{}/releases", event.repo().full_name),
                        Some(&serde_json::json!({
                            "tag_name": tag,
                            "name": expected_name,
                            "body": expected_body,
                        })),
                    )
                    .await;
                match e {
                    Ok(v) => log::debug!("created release: {:?}", v),
                    Err(e) => {
                        log::error!("Failed to create release: {:?}", e);

                        // Don't stop creating future releases just because this
                        // one failed.
                    }
                }
            }

            log::debug!("sleeping for one second to avoid hitting any rate limit");
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            log::trace!(
                "skipping tag {} since it doesn't have a changelog entry",
                tag
            );
        }
    }

    Ok(())
}

async fn load_changelog(
    ctx: &Context,
    event: &Event,
    config: &GitHubReleasesConfig,
) -> anyhow::Result<String> {
    let resp = ctx
        .github
        .raw_file(
            &event.repo().full_name,
            &config.changelog_branch,
            &config.changelog_path,
        )
        .await?
        .ok_or_else(|| anyhow::Error::msg("missing file"))?;

    Ok(String::from_utf8(resp.to_vec())?)
}

async fn load_paginated<T, R, F>(ctx: &Context, url: &str, key: F) -> anyhow::Result<HashMap<R, T>>
where
    T: serde::de::DeserializeOwned,
    R: Eq + PartialEq + std::hash::Hash,
    F: Fn(&T) -> R,
{
    let mut current_page: Page<T> = ctx
        .octocrab
        .get::<Page<T>, _, ()>(url, None)
        .await
        .with_context(|| format!("failed to load {url}"))?;

    let mut items = current_page
        .take_items()
        .into_iter()
        .map(|val| (key(&val), val))
        .collect::<HashMap<R, T>>();

    while let Some(mut new_page) = ctx
        .octocrab
        .get_page::<T>(&current_page.next)
        .await
        .with_context(|| format!("failed to load next page {:?}", current_page.next))?
    {
        items.extend(
            new_page
                .take_items()
                .into_iter()
                .map(|val| (key(&val), val)),
        );
        current_page = new_page;
    }

    Ok(items)
}

#[derive(Debug, serde::Deserialize)]
struct GitRef {
    #[serde(rename = "ref")]
    name: String,
}

#[derive(Debug, serde::Deserialize)]
struct Release {
    url: String,
    tag_name: String,
    name: String,
    body: String,
}
