#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, clap::ValueEnum, serde::Deserialize)]
#[value(rename_all = "PascalCase")]
pub enum CodegenBackend {
    Llvm,
    Cranelift,
}

impl CodegenBackend {
    pub fn all() -> Vec<CodegenBackend> {
        vec![CodegenBackend::Llvm, CodegenBackend::Cranelift]
    }
}
