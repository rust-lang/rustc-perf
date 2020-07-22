pub mod next_commit {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Response {
        pub commit: Option<String>,
    }
}
