#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize)]
pub struct CompilerTarget(pub String);

impl Default for CompilerTarget {
    fn default() -> Self {
        return Self("x86_64-unknown-linux-gnu".to_string())
    }
}
