use anyhow::Context;
use chrono::{DateTime, Utc};
use http::header;
use http::header::USER_AGENT;
use serde::de::DeserializeOwned;

use crate::load::SiteCtxt;

const BOT_USER_AGENT: &str = "perf-rust-lang-org-server";

/// A client for interacting with the GitHub API
pub struct Client {
    repository_url: String,
    token: String,
    inner: reqwest::Client,
}

impl Client {
    /// Create a new client from a URL and token (useful for testing)
    pub fn new(repository_url: String, token: String) -> Self {
        Self {
            repository_url,
            token,
            inner: reqwest::Client::new(),
        }
    }

    /// Create a client from a `SiteCtxt` and a URL
    pub fn from_ctxt(ctxt: &SiteCtxt, repository_url: String) -> Self {
        let token = ctxt
            .config
            .keys
            .github_api_token
            .clone()
            .expect("needs github API token");
        Self::new(repository_url, token)
    }

    pub async fn get_commit(&self, sha: &str) -> anyhow::Result<Commit> {
        let url = format!("{}/commits/{}", self.repository_url, sha);
        let req = self.inner.get(&url);
        let response = self.send(req).await.context("cannot get commit")?;
        if !response.status().is_success() {
            anyhow::bail!("{:?} != 200 OK", response.status());
        }
        response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("cannot deserialize commit: {:?}", e))
    }

    pub async fn post_comment<B>(&self, pr_number: u32, body: B)
    where
        B: Into<String>,
    {
        #[derive(Debug, Clone, serde::Serialize)]
        pub struct PostComment {
            pub body: String,
        }
        let body = body.into();
        let req = self
            .inner
            .post(format!(
                "{}/issues/{}/comments",
                self.repository_url, pr_number
            ))
            .json(&PostComment {
                body: body.to_owned(),
            });
        let resp = self.send(req).await;

        if let Err(e) = resp {
            log::error!("failed to post GitHub comment: {e:?}");
        }
    }

    async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, reqwest::Error> {
        request
            .header(USER_AGENT, BOT_USER_AGENT)
            .basic_auth("rust-timer", Some(&self.token))
            .send()
            .await
    }
}

const GRAPHQL_API_BASE: &str = "https://api.github.com/graphql";

/// A client for interacting with the GraphQL GitHub API.
pub struct GraphQLClient {
    inner: reqwest::Client,
    repo: &'static str,
}

impl GraphQLClient {
    /// Create a GraphQL client from a `SiteCtxt`.
    pub fn from_ctxt(ctxt: &SiteCtxt) -> Self {
        let token = ctxt
            .config
            .keys
            .github_api_token
            .as_ref()
            .expect("needs github API token");
        Self::from_token(token, "rust")
    }

    fn from_token(token: &str, repo: &'static str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(USER_AGENT, header::HeaderValue::from_static(BOT_USER_AGENT));
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {token}")).unwrap(),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            inner: client,
            repo,
        }
    }

    pub async fn get_comments(&self, pull_request: u32) -> anyhow::Result<Vec<ResponseComment>> {
        const QUERY: &str = "query($owner: String!, $repo: String!, $pr: Int!, $cursor: String) {
                repository(owner: $owner, name: $repo) {
                    pullRequest(number: $pr) {
                        comments(first: 100, after: $cursor) {
                            nodes {
                                id
                                fullDatabaseId
                                body
                                isMinimized
                                viewerDidAuthor
                            }
                            pageInfo {
                                endCursor
                            }
                        }
                    }
                }
            }";

        #[derive(Debug, serde::Deserialize)]
        struct Response {
            repository: ResponseRepo,
        }
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ResponseRepo {
            pull_request: ResponsePR,
        }
        #[derive(Debug, serde::Deserialize)]
        struct ResponsePR {
            comments: ResponseComments,
        }
        #[derive(Debug, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ResponseComments {
            nodes: Vec<ResponseComment>,
            page_info: GraphPageInfo,
        }

        let owner = "rust-lang";

        let mut comments = Vec::new();
        let mut cursor = None;
        loop {
            let mut resp: Response = self
                .send(
                    QUERY,
                    serde_json::json!({
                        "owner": owner,
                        "repo": self.repo,
                        "pr": pull_request,
                        "cursor": cursor,
                    }),
                )
                .await?;
            cursor = resp.repository.pull_request.comments.page_info.end_cursor;
            comments.append(&mut resp.repository.pull_request.comments.nodes);

            if cursor.is_none() {
                break;
            }
        }
        Ok(comments)
    }

    pub async fn update_comment_content(
        &self,
        comment_id: &str,
        new_body: &str,
    ) -> anyhow::Result<()> {
        #[derive(serde::Deserialize)]
        struct UpdateCommentData {}

        const UPDATE_COMMENT: &str = "mutation($id: ID!, $body: String!) {
            updateIssueComment(input: {id: $id, body: $body}) {
                issueComment {
                    id
                }
            }
        }";

        self.send::<UpdateCommentData, _>(
            UPDATE_COMMENT,
            serde_json::json!({
                "id": comment_id,
                "body": new_body,
            }),
        )
        .await?;
        Ok(())
    }

    pub async fn hide_comment(&self, comment_id: &str, reason: &str) -> anyhow::Result<()> {
        #[derive(serde::Deserialize)]
        struct MinimizeData {}

        const MINIMIZE: &str = "mutation($node_id: ID!, $reason: ReportedContentClassifiers!) {
            minimizeComment(input: {subjectId: $node_id, classifier: $reason}) {
                __typename
            }
        }";

        self.send::<Option<MinimizeData>, _>(
            MINIMIZE,
            serde_json::json!({
                "node_id": comment_id,
                "reason": reason,
            }),
        )
        .await?;
        Ok(())
    }

    async fn send<T: DeserializeOwned, V: serde::Serialize>(
        &self,
        query: &str,
        variables: V,
    ) -> anyhow::Result<T> {
        #[derive(serde::Serialize)]
        struct GraphPayload<'a, V> {
            query: &'a str,
            variables: V,
        }

        let response: GraphResponse<T> = self
            .inner
            .post(GRAPHQL_API_BASE)
            .json(&GraphPayload { query, variables })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if response.errors.is_empty() {
            Ok(response.data)
        } else {
            Err(anyhow::anyhow!(
                "GraphQL query failed: {}",
                response.errors[0].message
            ))
        }
    }
}

#[derive(serde::Deserialize)]
struct GraphResponse<T> {
    data: T,
    #[serde(default)]
    errors: Vec<GraphError>,
}

#[derive(Debug, serde::Deserialize)]
struct GraphError {
    message: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct GraphPageInfo {
    end_cursor: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseComment {
    pub id: String,
    pub full_database_id: String,
    pub body: String,
    pub is_minimized: bool,
    // Is the account that fetches this comment also the original author of the comment?
    pub viewer_did_author: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreatePrResponse {
    pub number: u32,
    pub html_url: String,
    pub comments_url: String,
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
    pub committer: Committer,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Committer {
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitTree {
    pub sha: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitParent {
    pub sha: String,
}
