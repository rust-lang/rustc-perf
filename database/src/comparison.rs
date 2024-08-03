use std::str::FromStr;

use serde::{de::IntoDeserializer, Deserialize, Serialize};

/// This enum contains all "known" metrics coming from rustc or profiling tools that we know
/// (and care) about.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Metric {
    #[serde(rename = "context-switches")]
    ContextSwitches,
    #[serde(rename = "cpu-clock")]
    CpuClock,
    #[serde(rename = "cpu-clock:u")]
    CpuClockUser,
    #[serde(rename = "cycles")]
    Cycles,
    #[serde(rename = "cycles:u")]
    CyclesUser,
    #[serde(rename = "faults")]
    Faults,
    #[serde(rename = "faults:u")]
    FaultsUser,
    #[serde(rename = "instructions:u")]
    InstructionsUser,
    #[serde(rename = "max-rss")]
    MaxRSS,
    #[serde(rename = "task-clock")]
    TaskClock,
    #[serde(rename = "task-clock:u")]
    TaskClockUser,
    #[serde(rename = "wall-time")]
    WallTime,
    #[serde(rename = "branch-misses")]
    BranchMisses,
    #[serde(rename = "cache-misses")]
    CacheMisses,
    /// Rustc guesses the codegen unit size by MIR count.
    #[serde(rename = "size:codegen_unit_size_estimate")]
    CodegenUnitSize,
    /// The codegen unit size by llvm ir count, the real size of a cgu.
    #[serde(rename = "size:cgu_instructions")]
    CodegenUnitLlvmIrCount,
    #[serde(rename = "size:dep_graph")]
    DepGraphSize,
    #[serde(rename = "size:linked_artifact")]
    LinkedArtifactSize,
    #[serde(rename = "size:object_file")]
    ObjectFileSize,
    #[serde(rename = "size:query_cache")]
    QueryCacheSize,
    #[serde(rename = "size:work_product_index")]
    WorkProductIndexSize,
    #[serde(rename = "size:crate_metadata")]
    CrateMetadataSize,
    #[serde(rename = "size:dwo_file")]
    DwoFileSize,
    #[serde(rename = "size:assembly_file")]
    AssemblyFileSize,
    #[serde(rename = "size:llvm_bitcode")]
    LlvmBitcodeSize,
    #[serde(rename = "size:llvm_ir")]
    LlvmIrSize,
    /// Total bytes of a generated documentation directory
    #[serde(rename = "size:doc_bytes")]
    DocByteSize,
    /// Number of files inside a generated documentation directory.
    #[serde(rename = "size:doc_files_count")]
    DocFilesCount,
}

impl FromStr for Metric {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Metric::deserialize(s.into_deserializer())
            .map_err(|e: serde::de::value::Error| format!("Unknown metric `{s}`: {e:?}"))
    }
}

impl Metric {
    pub fn as_str(&self) -> &str {
        match self {
            Metric::ContextSwitches => "context-switches",
            Metric::CpuClock => "cpu-clock",
            Metric::CpuClockUser => "cpu-clock:u",
            Metric::Cycles => "cycles",
            Metric::CyclesUser => "cycles:u",
            Metric::Faults => "faults",
            Metric::FaultsUser => "faults:u",
            Metric::InstructionsUser => "instructions:u",
            Metric::MaxRSS => "max-rss",
            Metric::TaskClock => "task-clock",
            Metric::TaskClockUser => "task-clock:u",
            Metric::WallTime => "wall-time",
            Metric::BranchMisses => "branch-misses",
            Metric::CacheMisses => "cache-misses",
            Metric::CodegenUnitSize => "size:codegen_unit_size_estimate",
            Metric::CodegenUnitLlvmIrCount => "size:cgu_instructions",
            Metric::DepGraphSize => "size:dep_graph",
            Metric::LinkedArtifactSize => "size:linked_artifact",
            Metric::ObjectFileSize => "size:object_file",
            Metric::QueryCacheSize => "size:query_cache",
            Metric::WorkProductIndexSize => "size:work_product_index",
            Metric::CrateMetadataSize => "size:crate_metadata",
            Metric::DwoFileSize => "size:dwo_file",
            Metric::AssemblyFileSize => "size:assembly_file",
            Metric::LlvmBitcodeSize => "size:llvm_bitcode",
            Metric::LlvmIrSize => "size:llvm_ir",
            Metric::DocByteSize => "size:doc_bytes",
            Metric::DocFilesCount => "size:doc_files_count",
        }
    }
}
