//! This module handles self-profile "rich" APIs (e.g., chrome profiler JSON)
//! generation from the raw artifacts on demand.

use anyhow::Context;
use std::collections::HashMap;

mod codegen_schedule;
pub mod crox;
pub mod flamegraph;

pub type ProcessorType = crate::api::self_profile_processed::ProcessorType;

pub struct Output {
    pub data: Vec<u8>,
    pub filename: &'static str,
    pub is_download: bool,
}

pub fn generate(
    title: &str,
    processor_type: ProcessorType,
    self_profile_base_data: Option<Vec<u8>>,
    self_profile_data: Vec<u8>,
    params: HashMap<String, String>,
) -> anyhow::Result<Output> {
    match processor_type {
        ProcessorType::Crox => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("crox opts")?;
            Ok(Output {
                filename: "chrome_profiler.json",
                data: crox::generate(self_profile_data, opt).context("crox")?,
                is_download: true,
            })
        }
        ProcessorType::Flamegraph => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("flame opts")?;
            Ok(Output {
                filename: "flamegraph.svg",
                data: flamegraph::generate(title, self_profile_data, opt).context("flame")?,
                is_download: false,
            })
        }
        ProcessorType::CodegenSchedule => {
            let opt =
                serde_json::from_str(&serde_json::to_string(&params).unwrap()).context("params")?;
            Ok(Output {
                filename: "schedule.html",
                data: codegen_schedule::generate(
                    title,
                    self_profile_base_data,
                    self_profile_data,
                    opt,
                )
                .context("codegen_schedule")?,
                is_download: false,
            })
        }
        _ => anyhow::bail!("Unknown type, specify type={crox,flamegraph,codegen-schedule}"),
    }
}
