use crate::pool::postgres::PostgresConnection;
use crate::pool::sqlite::SqliteConnection;
use crate::{
    pool::{Connection, ManagedConnection},
    CollectionId, Index, QueuedCommit,
};
use futures::join;
use hashbrown::HashMap;

pub struct BothConnection<A, B> {
    a: A,
    b: B,
}

impl BothConnection<SqliteConnection, ManagedConnection<PostgresConnection>> {
    pub fn new(sqlite: SqliteConnection, postgres: ManagedConnection<PostgresConnection>) -> Self {
        Self {
            a: sqlite,
            b: postgres,
        }
    }
}

pub struct BothTransaction<'a> {
    a: Box<dyn super::Transaction + 'a>,
    b: Box<dyn super::Transaction + 'a>,
}

#[async_trait::async_trait]
impl<'a> super::Transaction for BothTransaction<'a> {
    fn conn(&mut self) -> &mut dyn Connection {
        self
    }
    fn conn_ref(&self) -> &dyn Connection {
        self
    }
    async fn commit(self: Box<Self>) -> Result<(), anyhow::Error> {
        let (a, b) = join!(self.a.commit(), self.b.commit());
        a?;
        b?;
        Ok(())
    }
    async fn finish(self: Box<Self>) -> Result<(), anyhow::Error> {
        let (a, b) = join!(self.a.finish(), self.b.finish());
        a?;
        b?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<A, B> Connection for BothConnection<A, B>
where
    A: Connection,
    B: Connection,
{
    async fn maybe_create_indices(&mut self) {
        join!(self.a.maybe_create_indices(), self.b.maybe_create_indices());
    }
    async fn transaction(&mut self) -> Box<dyn super::Transaction + '_> {
        let (a, b) = join!(self.a.transaction(), self.b.transaction());
        Box::new(BothTransaction { a, b })
    }
    async fn load_index(&mut self) -> Index {
        let (a, b) = join!(self.a.load_index(), self.b.load_index());
        assert!(a == b);
        a
    }
    async fn get_pstats(
        &self,
        series: &[u32],
        cid: &[Option<crate::ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let (a, b) = join!(
            self.a.get_pstats(series, cid),
            self.b.get_pstats(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: crate::ArtifactIdNumber,
    ) -> Option<crate::QueryDatum> {
        let (a, b) = join!(
            self.a.get_self_profile_query(series, cid),
            self.b.get_self_profile_query(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn get_error(&self, cid: crate::ArtifactIdNumber) -> HashMap<String, Option<String>> {
        let (a, b) = join!(self.a.get_error(cid), self.b.get_error(cid));
        assert_eq!(a, b);
        a
    }
    async fn queue_pr(&self, pr: u32) {
        join!(self.a.queue_pr(pr), self.b.queue_pr(pr));
    }
    async fn pr_attach_commit(&self, pr: u32, sha: &str, parent_sha: &str) -> bool {
        let (a, b) = join!(
            self.a.pr_attach_commit(pr, sha, parent_sha),
            self.b.pr_attach_commit(pr, sha, parent_sha)
        );
        assert_eq!(a, b);
        a
    }
    async fn queued_commits(&self) -> Vec<QueuedCommit> {
        let (a, b) = join!(self.a.queued_commits(), self.b.queued_commits());
        assert_eq!(a, b);
        a
    }
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit> {
        let (a, b) = join!(self.a.mark_complete(sha), self.b.mark_complete(sha));
        assert_eq!(a, b);
        a
    }
    async fn collection_id(&self) -> CollectionId {
        let (a, b) = join!(self.a.collection_id(), self.b.collection_id());
        assert_eq!(a, b);
        a
    }
    async fn record_statistic(
        &self,
        collection: CollectionId,
        artifact: crate::ArtifactIdNumber,
        krate: &str,
        profile: crate::Profile,
        cache: crate::Cache,
        statistic: &str,
        value: f64,
    ) {
        join!(
            self.a
                .record_statistic(collection, artifact, krate, profile, cache, statistic, value),
            self.b
                .record_statistic(collection, artifact, krate, profile, cache, statistic, value)
        );
    }
    async fn artifact_id(&self, artifact: &crate::ArtifactId) -> crate::ArtifactIdNumber {
        let (a, b) = join!(self.a.artifact_id(artifact), self.b.artifact_id(artifact));
        assert_eq!(a, b);
        a
    }
    async fn record_self_profile_query(
        &self,
        collection: CollectionId,
        artifact: crate::ArtifactIdNumber,
        krate: &str,
        profile: crate::Profile,
        cache: crate::Cache,
        query: &str,
        qd: crate::QueryDatum,
    ) {
        join!(
            self.a.record_self_profile_query(
                collection,
                artifact,
                krate,
                profile,
                cache,
                query,
                qd.clone()
            ),
            self.b.record_self_profile_query(
                collection,
                artifact,
                krate,
                profile,
                cache,
                query,
                qd.clone()
            )
        );
    }
    async fn record_error(&self, artifact: crate::ArtifactIdNumber, krate: &str, error: &str) {
        join!(
            self.a.record_error(artifact, krate, error),
            self.b.record_error(artifact, krate, error)
        );
    }
    async fn record_benchmark(&self, krate: &str, supports_stable: bool) {
        join!(
            self.a.record_benchmark(krate, supports_stable),
            self.b.record_benchmark(krate, supports_stable)
        );
    }
}

#[async_trait::async_trait]
impl<'a> Connection for BothTransaction<'a> {
    async fn maybe_create_indices(&mut self) {
        join!(
            self.a.conn().maybe_create_indices(),
            self.b.conn().maybe_create_indices()
        );
    }
    async fn transaction<'b>(&'b mut self) -> Box<dyn super::Transaction + 'b> {
        panic!("nested transactions not supported")
    }
    async fn load_index(&mut self) -> Index {
        let (a, b) = join!(self.a.conn().load_index(), self.b.conn().load_index());
        for (a, b) in a.commits().iter().zip(b.commits().iter()) {
            assert_eq!(a, b);
        }
        assert_eq!(a.commits.map.len(), b.commits.map.len());
        assert_eq!(a.commits.map, b.commits.map);
        assert!(a == b);
        a
    }
    async fn get_pstats(
        &self,
        series: &[u32],
        cid: &[Option<crate::ArtifactIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let (a, b) = join!(
            self.a.conn_ref().get_pstats(series, cid),
            self.b.conn_ref().get_pstats(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: crate::ArtifactIdNumber,
    ) -> Option<crate::QueryDatum> {
        let (a, b) = join!(
            self.a.conn_ref().get_self_profile_query(series, cid),
            self.b.conn_ref().get_self_profile_query(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn get_error(&self, cid: crate::ArtifactIdNumber) -> HashMap<String, Option<String>> {
        let (a, b) = join!(
            self.a.conn_ref().get_error(cid),
            self.b.conn_ref().get_error(cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn queue_pr(&self, pr: u32) {
        join!(
            self.a.conn_ref().queue_pr(pr),
            self.b.conn_ref().queue_pr(pr)
        );
    }
    async fn pr_attach_commit(&self, pr: u32, sha: &str, parent_sha: &str) -> bool {
        let (a, b) = join!(
            self.a.conn_ref().pr_attach_commit(pr, sha, parent_sha),
            self.b.conn_ref().pr_attach_commit(pr, sha, parent_sha)
        );
        assert_eq!(a, b);
        a
    }
    async fn queued_commits(&self) -> Vec<QueuedCommit> {
        let (a, b) = join!(
            self.a.conn_ref().queued_commits(),
            self.b.conn_ref().queued_commits()
        );
        assert_eq!(a, b);
        a
    }
    async fn mark_complete(&self, sha: &str) -> Option<QueuedCommit> {
        let (a, b) = join!(
            self.a.conn_ref().mark_complete(sha),
            self.b.conn_ref().mark_complete(sha)
        );
        assert_eq!(a, b);
        a
    }
    async fn collection_id(&self) -> CollectionId {
        let (a, b) = join!(
            self.a.conn_ref().collection_id(),
            self.b.conn_ref().collection_id()
        );
        assert_eq!(a, b);
        a
    }
    async fn record_statistic(
        &self,
        collection: CollectionId,
        artifact: crate::ArtifactIdNumber,
        krate: &str,
        profile: crate::Profile,
        cache: crate::Cache,
        statistic: &str,
        value: f64,
    ) {
        join!(
            self.a
                .conn_ref()
                .record_statistic(collection, artifact, krate, profile, cache, statistic, value),
            self.b
                .conn_ref()
                .record_statistic(collection, artifact, krate, profile, cache, statistic, value)
        );
    }
    async fn artifact_id(&self, artifact: &crate::ArtifactId) -> crate::ArtifactIdNumber {
        let (a, b) = join!(
            self.a.conn_ref().artifact_id(artifact),
            self.b.conn_ref().artifact_id(artifact)
        );
        assert_eq!(a, b);
        a
    }
    async fn record_self_profile_query(
        &self,
        collection: CollectionId,
        artifact: crate::ArtifactIdNumber,
        krate: &str,
        profile: crate::Profile,
        cache: crate::Cache,
        query: &str,
        qd: crate::QueryDatum,
    ) {
        join!(
            self.a.conn_ref().record_self_profile_query(
                collection,
                artifact,
                krate,
                profile,
                cache,
                query,
                qd.clone()
            ),
            self.b.conn_ref().record_self_profile_query(
                collection,
                artifact,
                krate,
                profile,
                cache,
                query,
                qd.clone()
            )
        );
    }
    async fn record_error(&self, artifact: crate::ArtifactIdNumber, krate: &str, error: &str) {
        join!(
            self.a.conn_ref().record_error(artifact, krate, error),
            self.b.conn_ref().record_error(artifact, krate, error)
        );
    }
    async fn record_benchmark(&self, krate: &str, supports_stable: bool) {
        join!(
            self.a.conn_ref().record_benchmark(krate, supports_stable),
            self.b.conn_ref().record_benchmark(krate, supports_stable)
        );
    }
}
