use anyhow::Context;
use http::header::USER_AGENT;

use crate::{
    api::github::Issue,
    load::{Config, SiteCtxt},
};

#[derive(serde::Serialize)]
struct CreateRefRequest<'a> {
    // Must start with `refs/` and have at least two slashes.
    // e.g. `refs/heads/master`.
    #[serde(rename = "ref")]
    ref_: &'a str,
    sha: &'a str,
}

pub async fn create_ref(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    ref_: &str,
    sha: &str,
) -> anyhow::Result<()> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/git/refs", repository_url);
    let response = client
        .post(&url)
        .json(&CreateRefRequest { ref_, sha })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("POST git/refs failed")?;
    if response.status() != reqwest::StatusCode::CREATED {
        anyhow::bail!("{:?} != 201 CREATED", response.status());
    }

    Ok(())
}

#[derive(serde::Serialize)]
struct CreatePrRequest<'a> {
    title: &'a str,
    // username:branch if cross-repo
    head: &'a str,
    // branch to pull into (e.g, master)
    base: &'a str,
    #[serde(rename = "body")]
    description: &'a str,
    draft: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePrResponse {
    pub number: u32,
    pub html_url: String,
    pub comments_url: String,
}

pub async fn create_pr(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    title: &str,
    head: &str,
    base: &str,
    description: &str,
    draft: bool,
) -> anyhow::Result<CreatePrResponse> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/pulls", repository_url);
    let response = client
        .post(&url)
        .json(&CreatePrRequest {
            title,
            head,
            base,
            description,
            draft,
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("POST pulls failed")?;
    if response.status() != reqwest::StatusCode::CREATED {
        anyhow::bail!("{:?} != 201 CREATED", response.status());
    }

    Ok(response.json().await.context("deserializing failed")?)
}

#[derive(serde::Serialize)]
struct UpdateBranchRequest<'a> {
    sha: &'a str,
    force: bool,
}

pub async fn update_branch(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    branch: &str,
    sha: &str,
) -> anyhow::Result<()> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/git/refs/{}", repository_url, branch);
    let commit_response = client
        .patch(&url)
        .json(&UpdateBranchRequest { sha, force: true })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("PATCH git/refs failed")?;
    if commit_response.status() != reqwest::StatusCode::OK {
        anyhow::bail!("{:?} != 200 OK", commit_response.status());
    }

    Ok(())
}

#[derive(serde::Serialize)]
struct MergeBranchRequest<'a> {
    base: &'a str,
    head: &'a str,
    commit_message: &'a str,
}
#[derive(serde::Deserialize)]
struct MergeBranchResponse {
    sha: String,
}

pub async fn merge_branch(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    branch: &str,
    sha: &str,
    commit_message: &str,
) -> anyhow::Result<String> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/merges", repository_url);
    let response = client
        .patch(&url)
        .json(&MergeBranchRequest {
            base: branch,
            head: sha,
            commit_message,
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("PATCH /merges failed")?;
    if !response.status().is_success() {
        anyhow::bail!("{:?} != 201 CREATED", response.status());
    }

    Ok(response.json::<MergeBranchResponse>().await?.sha)
}

#[derive(serde::Serialize)]
struct CreateCommitRequest<'a> {
    message: &'a str,
    tree: &'a str,
    parents: &'a [&'a str],
}

#[derive(serde::Deserialize)]
struct CreateCommitResponse {
    sha: String,
}

pub async fn create_commit(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    message: &str,
    tree: &str,
    parents: &[&str],
) -> anyhow::Result<String> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/git/commits", repository_url);
    let commit_response = client
        .post(&url)
        .json(&CreateCommitRequest {
            message,
            tree,
            parents,
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("POST git/commits failed")?;
    if commit_response.status() != reqwest::StatusCode::CREATED {
        anyhow::bail!("{:?} != 201 CREATED", commit_response.status());
    }

    Ok(commit_response
        .json::<CreateCommitResponse>()
        .await
        .context("deserializing failed")?
        .sha)
}

pub async fn get_issue(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    number: u64,
) -> anyhow::Result<Issue> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/issues/{}", repository_url, number);
    let response = client
        .get(&url)
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("cannot get issue")?;
    if !response.status().is_success() {
        anyhow::bail!("{:?} != 200 OK", response.status());
    }

    Ok(response.json().await?)
}

pub async fn get_commit(
    client: &reqwest::Client,
    ctxt: &SiteCtxt,
    repository_url: &str,
    sha: &str,
) -> anyhow::Result<Commit> {
    let timer_token = ctxt
        .config
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let url = format!("{}/commits/{}", repository_url, sha);
    let commit_response = client
        .get(&url)
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token))
        .send()
        .await
        .context("cannot get commit")?;
    let commit_response = match commit_response.text().await {
        Ok(c) => c,
        Err(err) => {
            anyhow::bail!("Failed to decode response for {}: {:?}", url, err);
        }
    };
    match serde_json::from_str(&commit_response) {
        Ok(c) => Ok(c),
        Err(e) => Err(anyhow::anyhow!(
            "cannot deserialize commit ({}): {:?}",
            commit_response,
            e
        )),
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: InnerCommit,
    pub parents: Vec<CommitParent>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct InnerCommit {
    #[serde(default)]
    pub message: String,
    pub tree: CommitTree,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitTree {
    pub sha: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitParent {
    pub sha: String,
}

pub async fn post_comment<B>(cfg: &Config, pr: u32, body: B)
where
    B: Into<String>,
{
    let body = body.into();
    let timer_token = cfg
        .keys
        .github_api_token
        .clone()
        .expect("needs github API token");
    let client = reqwest::Client::new();
    let req = client
        .post(&format!(
            "https://api.github.com/repos/rust-lang/rust/issues/{}/comments",
            pr
        ))
        .json(&PostComment {
            body: body.to_owned(),
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token));

    if let Err(e) = req.send().await {
        eprintln!("failed to post comment: {:?}", e);
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PostComment {
    pub body: String,
}
