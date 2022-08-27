pub mod next_artifact {
    use database::Commit;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub enum NextArtifact {
        Release(String),
        Commit {
            commit: Commit,
            include: Option<String>,
            exclude: Option<String>,
            runs: Option<i32>,
        },
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Response {
        pub artifact: Option<NextArtifact>,
    }
}
