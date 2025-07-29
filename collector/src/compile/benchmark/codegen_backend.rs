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

impl From<CodegenBackend> for database::CodegenBackend {
    fn from(value: CodegenBackend) -> Self {
        match value {
            CodegenBackend::Llvm => database::CodegenBackend::Llvm,
            CodegenBackend::Cranelift => database::CodegenBackend::Cranelift,
        }
    }
}
