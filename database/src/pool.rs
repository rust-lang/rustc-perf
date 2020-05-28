use crate::{CollectionIdNumber, QueryDatum};

pub mod sqlite;

#[async_trait::async_trait]
pub trait Connection: Send {
    async fn maybe_create_tables(&mut self);
    async fn maybe_create_indices(&mut self);
    async fn transaction(&mut self) -> Box<dyn Transaction + '_>;

    async fn load_index(&mut self) -> Option<Vec<u8>>;
    async fn store_index(&mut self, index: &[u8]);

    async fn get_pstat(&mut self, series: u32, cid: CollectionIdNumber) -> Option<f64>;
    async fn insert_pstat(&mut self, series: u32, cid: CollectionIdNumber, stat: f64);
    async fn get_self_profile_query(
        &mut self,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<QueryDatum>;
    async fn insert_self_profile_query(
        &mut self,
        series: u32,
        cid: CollectionIdNumber,
        data: &QueryDatum,
    );
    async fn get_error(&mut self, series: u32, cid: CollectionIdNumber) -> Option<String>;
    async fn insert_error(&mut self, series: u32, cid: CollectionIdNumber, text: &str);
}

#[async_trait::async_trait]
pub trait Transaction:
    Send + std::ops::Deref<Target = dyn Connection> + std::ops::DerefMut
{
    fn conn(&mut self) -> &mut dyn Connection {
        self.deref_mut()
    }

    async fn commit(&mut self) -> Result<(), anyhow::Error>;
    async fn finish(&mut self) -> Result<(), anyhow::Error>;
}

pub enum Pool {
    Sqlite(r2d2::Pool<sqlite::Sqlite>),
}

impl Pool {
    pub fn connection(&self) -> Box<dyn Connection> {
        match self {
            Pool::Sqlite(p) => Box::new(sqlite::SqliteConnection::new(p.get().unwrap())),
        }
    }

    pub fn open(uri: &str) -> Pool {
        Pool::Sqlite(
            r2d2::Pool::builder()
                .max_size(16)
                .connection_timeout(std::time::Duration::from_secs(1))
                .build(sqlite::Sqlite::new(uri.into()))
                .unwrap(),
        )
    }
}
