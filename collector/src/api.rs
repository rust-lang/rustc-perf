pub mod next_commit {
    use database::Commit;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct NextCommit {
        pub commit: Commit,
        pub include: Option<String>,
        pub exclude: Option<String>,
        pub runs: Option<i32>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Response {
        pub commit: Option<NextCommit>,
    }
}
