pub mod next_artifact {
    use database::ArtifactId;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct NextArtifact {
        pub artifact: ArtifactId,
        pub include: Option<String>,
        pub exclude: Option<String>,
        pub runs: Option<i32>,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Response {
        pub artifact: Option<NextArtifact>,
    }
}
