//! This is just a quick script to compact the rocksdb database.
//!
//! It's generally never needed -- this should happen automatically as the site
//! runs -- but it's short and easy to keep around, and sometimes useful.

fn main() {
    let db = site::db::open(&std::env::args().nth(1).expect("db dir"), false);
    db.compact_range(None::<&[u8]>, None::<&[u8]>);
}
