use crate::load::SiteCtxt;

use collector::Bound;
use database::selector::StatisticSeries;
use database::selector::{BenchmarkQuery, SeriesResponse};
use database::ArtifactId;
use database::{Commit, Index};

use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::Instant;

/// Finds the most appropriate `ArtifactId` for a given bound.
///
/// Searches the commits in the index either from the left or the right.
/// If not found in those commits, searches through the artifacts in the index.
pub fn artifact_id_for_bound(data: &Index, bound: Bound, is_left: bool) -> Option<ArtifactId> {
    let commits = data.commits();
    let commit = if is_left {
        commits
            .iter()
            .find(|commit| bound.left_match(commit))
            .cloned()
    } else {
        commits
            .iter()
            .rfind(|commit| bound.right_match(commit))
            .cloned()
    };
    commit.map(ArtifactId::Commit).or_else(|| {
        data.artifacts()
            .find(|aid| match &bound {
                Bound::Commit(c) => *c == **aid,
                Bound::Date(_) => false,
                Bound::None => false,
            })
            .map(|aid| ArtifactId::Tag(aid.to_string()))
    })
}

pub fn range_subset(data: Vec<Commit>, range: RangeInclusive<Bound>) -> Vec<Commit> {
    let (a, b) = range.into_inner();

    let left_idx = data.iter().position(|commit| a.left_match(commit));
    let right_idx = data.iter().rposition(|commit| b.right_match(commit));

    if let (Some(left), Some(right)) = (left_idx, right_idx) {
        data.get(left..=right)
            .map(|s| s.to_vec())
            .unwrap_or_else(|| {
                log::error!(
                    "Failed to compute left/right indices from {:?}..={:?}",
                    a,
                    b
                );
                vec![]
            })
    } else {
        vec![]
    }
}

impl SiteCtxt {
    pub async fn statistic_series<Q: BenchmarkQuery>(
        &self,
        query: Q,
        artifact_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<Q::TestCase, StatisticSeries>>, String> {
        StatisticSeries::execute_query(artifact_ids, self, query).await
    }
}

trait StatisticSeriesExt {
    async fn execute_query<Q: BenchmarkQuery>(
        artifact_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        query: Q,
    ) -> Result<Vec<SeriesResponse<Q::TestCase, StatisticSeries>>, String>;
}

impl StatisticSeriesExt for StatisticSeries {
    async fn execute_query<Q: BenchmarkQuery>(
        artifact_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        query: Q,
    ) -> Result<Vec<SeriesResponse<Q::TestCase, Self>>, String> {
        let dumped = format!("{:?}", query);

        let index = ctxt.index.load();
        let mut conn = ctxt.conn().await;

        let start = Instant::now();
        let result = query.execute(conn.as_mut(), &index, artifact_ids).await?;
        log::trace!(
            "{:?}: run {} from {}",
            start.elapsed(),
            result.len(),
            dumped
        );
        Ok(result)
    }
}
