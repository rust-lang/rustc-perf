use crate::{ArtifactId, ArtifactIdIter};

#[derive(Debug)]
pub struct StatisticSeries {
    pub artifact_ids: ArtifactIdIter,
    pub points: std::vec::IntoIter<Option<f64>>,
}

impl Iterator for StatisticSeries {
    type Item = (ArtifactId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.artifact_ids.next()?, self.points.next().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.artifact_ids.size_hint()
    }
}
