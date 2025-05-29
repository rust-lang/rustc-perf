//! Definitions for GitHub GraphQL.
//!
//! See <https://docs.github.com/en/graphql> for more GitHub's GraphQL API.

// This schema can be downloaded from https://docs.github.com/public/schema.docs.graphql
pub mod queries {
    use super::schema;

    pub type Date = chrono::NaiveDate;
    pub type DateTime = chrono::DateTime<chrono::Utc>;

    cynic::impl_scalar!(Date, schema::Date);
    cynic::impl_scalar!(DateTime, schema::DateTime);

    #[derive(cynic::QueryVariables, Debug, Clone)]
    pub struct LeastRecentlyReviewedPullRequestsArguments<'a> {
        pub repository_owner: &'a str,
        pub repository_name: &'a str,
        pub after: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(
        graphql_type = "Query",
        variables = "LeastRecentlyReviewedPullRequestsArguments"
    )]
    pub struct LeastRecentlyReviewedPullRequests {
        #[arguments(owner: $repository_owner, name: $repository_name)]
        pub repository: Option<Repository>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "LeastRecentlyReviewedPullRequestsArguments")]
    pub struct Repository {
        #[arguments(
            states: "OPEN",
            first: 100,
            after: $after,
            labels: ["S-waiting-on-review"],
            orderBy: {direction: "ASC", field: "UPDATED_AT"}
        )]
        pub pull_requests: PullRequestConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PullRequestConnection {
        pub total_count: i32,
        pub page_info: PageInfo,
        #[cynic(flatten)]
        pub nodes: Vec<PullRequest>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PullRequest {
        pub number: i32,
        pub author: Option<Actor>,
        pub created_at: DateTime,
        pub url: Uri,
        pub title: String,
        #[arguments(first = 100)]
        pub labels: Option<LabelConnection>,
        pub is_draft: bool,
        #[arguments(first = 100)]
        pub assignees: UserConnection,
        #[arguments(first = 100, orderBy: { direction: "DESC", field: "UPDATED_AT" })]
        pub comments: IssueCommentConnection,
        #[arguments(last = 20)]
        pub latest_reviews: Option<PullRequestReviewConnection>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PullRequestReviewConnection {
        pub total_count: i32,
        #[cynic(flatten)]
        pub nodes: Vec<PullRequestReview>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PullRequestReview {
        pub author: Option<Actor>,
        pub created_at: DateTime,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct UserConnection {
        #[cynic(flatten)]
        pub nodes: Vec<User>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        pub login: String,
        pub database_id: Option<i32>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PageInfo {
        pub has_next_page: bool,
        pub end_cursor: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct LabelConnection {
        #[cynic(flatten)]
        pub nodes: Vec<Label>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Label {
        pub name: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct IssueCommentConnection {
        pub total_count: i32,
        #[cynic(flatten)]
        pub nodes: Vec<IssueComment>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct IssueComment {
        pub author: Option<Actor>,
        pub created_at: DateTime,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Actor {
        pub login: String,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    #[cynic(graphql_type = "URI")]
    pub struct Uri(pub String);
}

pub mod docs_update_queries {
    use super::queries::{DateTime, PageInfo};
    use super::schema;

    #[derive(cynic::QueryVariables, Clone, Debug)]
    pub struct RecentCommitsArguments<'a> {
        pub branch: &'a str,
        pub name: &'a str,
        pub owner: &'a str,
        pub after: Option<String>,
    }

    /// Query for fetching recent commits and their associated PRs.
    ///
    /// This query is built from:
    ///
    /// ```text
    /// query RecentCommits($name: String!, $owner: String!, $branch: String!, $after: String) {
    ///   repository(name: $name, owner: $owner) {
    ///     ref(qualifiedName: $branch) {
    ///       target {
    ///         ... on Commit {
    ///           history(first: 100, after: $after) {
    ///             totalCount
    ///             pageInfo {
    ///               hasNextPage
    ///               endCursor
    ///             }
    ///             nodes {
    ///               oid
    ///               parents(first: 1) {
    ///                 nodes {
    ///                   oid
    ///                 }
    ///               }
    ///               committedDate
    ///               messageHeadline
    ///               associatedPullRequests(first: 1) {
    ///                 nodes {
    ///                   number
    ///                   title
    ///                 }
    ///               }
    ///             }
    ///           }
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    /// ```
    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "RecentCommitsArguments")]
    pub struct RecentCommits {
        #[arguments(name: $name, owner: $owner)]
        pub repository: Option<Repository>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "RecentCommitsArguments")]
    pub struct Repository {
        #[arguments(qualifiedName: $branch)]
        #[cynic(rename = "ref")]
        pub ref_: Option<Ref>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "RecentCommitsArguments")]
    pub struct Ref {
        pub target: Option<GitObject>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "RecentCommitsArguments")]
    pub struct Commit {
        #[arguments(first: 100, after: $after)]
        pub history: CommitHistoryConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct CommitHistoryConnection {
        pub total_count: i32,
        pub page_info: PageInfo,
        #[cynic(flatten)]
        pub nodes: Vec<Commit2>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Commit")]
    pub struct Commit2 {
        pub oid: GitObjectID,
        #[arguments(first = 1)]
        pub parents: CommitConnection,
        pub committed_date: DateTime,
        pub message_headline: String,
        #[arguments(first = 1)]
        pub associated_pull_requests: Option<PullRequestConnection>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PullRequestConnection {
        #[cynic(flatten)]
        pub nodes: Vec<PullRequest>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct PullRequest {
        pub number: i32,
        pub title: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct CommitConnection {
        #[cynic(flatten)]
        pub nodes: Vec<Commit3>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Commit")]
    pub struct Commit3 {
        pub oid: GitObjectID,
    }

    #[derive(cynic::InlineFragments, Debug)]
    #[cynic(variables = "RecentCommitsArguments")]
    pub enum GitObject {
        Commit(Commit),
        #[cynic(fallback)]
        Other,
    }

    #[derive(cynic::Scalar, Debug, Clone)]
    pub struct GitObjectID(pub String);
}

#[cynic::schema("github")]
mod schema {}

pub mod project_items {
    use super::queries::{Date, PageInfo, Uri};
    use super::schema;

    #[derive(cynic::QueryVariables, Debug, Clone)]
    pub struct Arguments {
        pub project_number: i32,
        pub after: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "Arguments")]
    pub struct Query {
        #[arguments(login: "rust-lang")]
        pub organization: Option<Organization>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "Arguments")]
    pub struct Organization {
        #[arguments(number: $project_number)]
        pub project_v2: Option<ProjectV2>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "Arguments")]
    pub struct ProjectV2 {
        #[arguments(first: 100, after: $after)]
        pub items: ProjectV2ItemConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ProjectV2ItemConnection {
        pub nodes: Option<Vec<Option<ProjectV2Item>>>,
        pub page_info: PageInfo,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ProjectV2Item {
        pub content: Option<ProjectV2ItemContent>,

        // Currently we hard code the field names we care about here.
        #[cynic(rename = "fieldValueByName")]
        #[arguments(name = "Status")]
        pub status: Option<ProjectV2ItemFieldValue>,
        #[cynic(rename = "fieldValueByName")]
        #[arguments(name = "Date")]
        pub date: Option<ProjectV2ItemFieldValue>,
    }

    impl ProjectV2Item {
        pub fn status(&self) -> Option<&str> {
            let Some(ref status) = self.status else {
                return None;
            };
            status.as_str()
        }

        pub fn date(&self) -> Option<Date> {
            let Some(ref date) = self.date else {
                return None;
            };
            date.as_date()
        }
    }

    #[derive(cynic::InlineFragments, Debug)]
    pub enum ProjectV2ItemContent {
        Issue(Issue),

        #[cynic(fallback)]
        Other,
    }

    #[derive(cynic::InlineFragments, Debug)]
    pub enum ProjectV2ItemFieldValue {
        ProjectV2ItemFieldSingleSelectValue(ProjectV2ItemFieldSingleSelectValue),
        ProjectV2ItemFieldDateValue(ProjectV2ItemFieldDateValue),

        #[cynic(fallback)]
        Other,
    }

    impl ProjectV2ItemFieldValue {
        pub fn as_str(&self) -> Option<&str> {
            Some(match self {
                Self::ProjectV2ItemFieldSingleSelectValue(val) => val.name.as_deref()?,
                _ => return None,
            })
        }

        pub fn as_date(&self) -> Option<Date> {
            match self {
                Self::ProjectV2ItemFieldDateValue(val) => val.date,
                _ => None,
            }
        }
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Issue {
        pub title: String,
        pub url: Uri,
        pub number: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ProjectV2ItemFieldSingleSelectValue {
        pub name: Option<String>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct ProjectV2ItemFieldDateValue {
        pub date: Option<Date>,
    }
}
