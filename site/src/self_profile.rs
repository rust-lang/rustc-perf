//! This module handles self-profile "rich" APIs (e.g., chrome profiler JSON)
//! generation from the raw artifacts on demand.

use anyhow::Context;
use std::collections::HashMap;

mod codegen_schedule;
pub mod crox;
pub mod flamegraph;

pub struct Output {
    pub data: Vec<u8>,
    pub filename: &'static str,
    pub is_download: bool,
}

pub fn generate(
    title: &str,
    self_profile_base_data: Option<Vec<u8>>,
    self_profile_data: Vec<u8>,
    mut params: HashMap<String, String>,
) -> anyhow::Result<Output> {
    let removed = params.remove("type");
    match removed.as_deref() {
        Some("crox") => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("crox opts")?;
            Ok(Output {
                filename: "chrome_profiler.json",
                data: crox::generate(self_profile_data, opt).context("crox")?,
                is_download: true,
            })
        }
        Some("flamegraph") => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("flame opts")?;
            Ok(Output {
                filename: "flamegraph.svg",
                data: flamegraph::generate(title, self_profile_data, opt).context("flame")?,
                is_download: false,
            })
        }
        Some("codegen-schedule") => {
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
