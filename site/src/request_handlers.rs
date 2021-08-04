mod bootstrap;
mod dashboard;
mod github;
mod graph;
mod next_commit;
mod self_profile;
mod status_page;

pub use bootstrap::handle_bootstrap;
pub use dashboard::handle_dashboard;
pub use github::handle_github;
pub use graph::{handle_graph, handle_graph_new};
pub use next_commit::handle_next_commit;
pub use self_profile::{
    get_self_profile_raw, handle_self_profile, handle_self_profile_processed_download,
    handle_self_profile_raw, handle_self_profile_raw_download,
};
pub use status_page::handle_status_page;

use crate::api::{info, ServerResult};
use crate::load::SiteCtxt;

pub fn handle_info(ctxt: &SiteCtxt) -> info::Response {
    let mut metrics = ctxt.index.load().metrics();
    metrics.sort();
    info::Response {
        stats: metrics,
        as_of: ctxt.index.load().commits().last().map(|d| d.date),
    }
}

pub async fn handle_collected() -> ServerResult<()> {
    Ok(())
}
