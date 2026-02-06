mod bootstrap;
mod dashboard;
mod github;
mod graph;
mod self_profile;
mod status_page;

pub use bootstrap::handle_bootstrap;
pub use dashboard::handle_dashboard;
pub use github::handle_github_webhook;
pub use graph::{
    handle_compile_detail_graphs, handle_compile_detail_sections, handle_graphs,
    handle_runtime_detail_graphs,
};
pub use self_profile::{
    handle_self_profile, handle_self_profile_processed_download, handle_self_profile_raw_download,
};
pub use status_page::handle_status_page;

use crate::api::{info, ServerResult};
use crate::load::SiteCtxt;

pub fn handle_info(ctxt: &SiteCtxt) -> info::Response {
    let mut compile_metrics = ctxt.index.load().compile_metrics();
    compile_metrics.sort();

    let mut runtime_metrics = ctxt.index.load().runtime_metrics();
    runtime_metrics.sort();

    info::Response {
        compile_metrics,
        runtime_metrics,
    }
}

pub async fn handle_collected() -> ServerResult<()> {
    Ok(())
}
