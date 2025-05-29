use crate::github::GithubClient;
use anyhow::Context as _;
use rust_team_data::v1::{People, Teams, ZulipMapping, BASE_URL};
use serde::de::DeserializeOwned;

async fn by_url<T: DeserializeOwned>(client: &GithubClient, path: &str) -> anyhow::Result<T> {
    let base = std::env::var("TEAMS_API_URL").unwrap_or(BASE_URL.to_string());
    let url = format!("{}{}", base, path);
    for _ in 0i32..3 {
        let map: Result<T, _> = client.json(client.raw().get(&url)).await;
        match map {
            Ok(v) => return Ok(v),
            Err(e) => {
                if e.downcast_ref::<reqwest::Error>()
                    .map_or(false, |e| e.is_timeout())
                {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }

    Err(anyhow::anyhow!("Failed to retrieve {} in 3 requests", url))
}

pub async fn zulip_map(client: &GithubClient) -> anyhow::Result<ZulipMapping> {
    by_url(client, "/zulip-map.json")
        .await
        .context("team-api: zulip-map.json")
}

pub async fn teams(client: &GithubClient) -> anyhow::Result<Teams> {
    by_url(client, "/teams.json")
        .await
        .context("team-api: teams.json")
}

pub async fn people(client: &GithubClient) -> anyhow::Result<People> {
    by_url(client, "/people.json")
        .await
        .context("team-api: people.json")
}
