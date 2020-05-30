use crate::pool::postgres::PostgresConnection;
use crate::pool::sqlite::SqliteConnection;
use crate::{
    pool::{Connection, ManagedConnection},
    QueuedCommit,
};
use futures::join;

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
    async fn load_index(&mut self) -> Option<Vec<u8>> {
        let (a, b) = join!(self.a.load_index(), self.b.load_index());
        assert!(a == b);
        a
    }
    async fn store_index(&mut self, index: &[u8]) {
        join!(self.a.store_index(index), self.b.store_index(index));
    }
    async fn get_pstats(
        &self,
        series: &[u32],
        cid: &[Option<crate::CollectionIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let (a, b) = join!(
            self.a.get_pstats(series, cid),
            self.b.get_pstats(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn insert_pstat(&self, series: u32, cid: crate::CollectionIdNumber, stat: f64) {
        join!(
            self.a.insert_pstat(series, cid, stat),
            self.b.insert_pstat(series, cid, stat)
        );
    }
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: crate::CollectionIdNumber,
    ) -> Option<crate::QueryDatum> {
        let (a, b) = join!(
            self.a.get_self_profile_query(series, cid),
            self.b.get_self_profile_query(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn insert_self_profile_query(
        &self,
        series: u32,
        cid: crate::CollectionIdNumber,
        data: crate::QueryDatum,
    ) {
        join!(
            self.a.insert_self_profile_query(series, cid, data.clone()),
            self.b.insert_self_profile_query(series, cid, data)
        );
    }
    async fn get_error(&self, series: u32, cid: crate::CollectionIdNumber) -> Option<String> {
        let (a, b) = join!(self.a.get_error(series, cid), self.b.get_error(series, cid));
        assert_eq!(a, b);
        a
    }
    async fn insert_error(&self, series: u32, cid: crate::CollectionIdNumber, text: String) {
        join!(
            self.a.insert_error(series, cid, text.clone()),
            self.b.insert_error(series, cid, text)
        );
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
}

#[async_trait::async_trait]
impl<'a> Connection for BothTransaction<'a> {
    async fn maybe_create_indices(&mut self) {
        join!(
            self.a.conn().maybe_create_indices(),
            self.b.conn().maybe_create_indices()
        );
    }
    async fn transaction(&mut self) -> Box<dyn super::Transaction> {
        panic!("nested transactions not supported")
    }
    async fn load_index(&mut self) -> Option<Vec<u8>> {
        let (a, b) = join!(self.a.conn().load_index(), self.b.conn().load_index());
        assert_eq!(a, b);
        a
    }
    async fn store_index(&mut self, index: &[u8]) {
        join!(
            self.a.conn().store_index(index),
            self.b.conn().store_index(index)
        );
    }
    async fn get_pstats(
        &self,
        series: &[u32],
        cid: &[Option<crate::CollectionIdNumber>],
    ) -> Vec<Vec<Option<f64>>> {
        let (a, b) = join!(
            self.a.conn_ref().get_pstats(series, cid),
            self.b.conn_ref().get_pstats(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn insert_pstat(&self, series: u32, cid: crate::CollectionIdNumber, stat: f64) {
        join!(
            self.a.conn_ref().insert_pstat(series, cid, stat),
            self.b.conn_ref().insert_pstat(series, cid, stat)
        );
    }
    async fn get_self_profile_query(
        &self,
        series: u32,
        cid: crate::CollectionIdNumber,
    ) -> Option<crate::QueryDatum> {
        let (a, b) = join!(
            self.a.conn_ref().get_self_profile_query(series, cid),
            self.b.conn_ref().get_self_profile_query(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn insert_self_profile_query(
        &self,
        series: u32,
        cid: crate::CollectionIdNumber,
        data: crate::QueryDatum,
    ) {
        join!(
            self.a
                .conn_ref()
                .insert_self_profile_query(series, cid, data.clone()),
            self.b
                .conn_ref()
                .insert_self_profile_query(series, cid, data)
        );
    }
    async fn get_error(&self, series: u32, cid: crate::CollectionIdNumber) -> Option<String> {
        let (a, b) = join!(
            self.a.conn_ref().get_error(series, cid),
            self.b.conn_ref().get_error(series, cid)
        );
        assert_eq!(a, b);
        a
    }
    async fn insert_error(&self, series: u32, cid: crate::CollectionIdNumber, text: String) {
        join!(
            self.a.conn_ref().insert_error(series, cid, text.clone()),
            self.b.conn_ref().insert_error(series, cid, text)
        );
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
}
