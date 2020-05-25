use r2d2::{ManageConnection, PooledConnection};
use std::path::PathBuf;

pub struct SqliteTransaction(PooledConnection<Sqlite>, bool);

impl SqliteTransaction {
    pub fn start(conn: PooledConnection<Sqlite>) -> Self {
        conn.execute_batch("BEGIN DEFERRED").unwrap();
        Self(conn, false)
    }

    pub fn commit(mut self) -> Result<(), rusqlite::Error> {
        let r = self.0.execute_batch("COMMIT");
        self.1 = true;
        r
    }
    pub fn finish(mut self) -> Result<(), rusqlite::Error> {
        let r = self.0.execute_batch("ROLLBACK");
        self.1 = true;
        r
    }
}

impl std::ops::Deref for SqliteTransaction {
    type Target = PooledConnection<Sqlite>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SqliteTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for SqliteTransaction {
    fn drop(&mut self) {
        if !self.1 {
            self.0.execute_batch("ROLLBACK").unwrap();
        }
    }
}

pub struct Sqlite(PathBuf);

impl Sqlite {
    pub fn new(path: PathBuf) -> Self {
        Sqlite(path)
    }
}

impl ManageConnection for Sqlite {
    type Connection = rusqlite::Connection;
    type Error = rusqlite::Error;
    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = rusqlite::Connection::open(&self.0)?;
        conn.pragma_update(None, "cache_size", &-128000)?;
        Ok(conn)
    }
    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.execute_batch("")
    }
    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
