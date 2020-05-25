use rusqlite::{params, Connection};
use std::path::Path;

fn main() {
    let db = std::env::args().nth(1).expect("database as first arg");
    let conn = Connection::open(&db).unwrap();
    let mut index = site::db::Index::load(&conn);
    conn.pragma_update(None, "cache_size", &-64000).unwrap();

    // When ingesting a bunch of data (the primary use case for this script),
    // we generally don't need durability.
    conn.pragma_update(None, "journal_mode", &"WAL").unwrap();
    conn.pragma_update(None, "synchronous", &"NORMAL").unwrap();

    conn.execute(
        "create table if not exists interned(name text primary key, value blob);",
        params![],
    )
    .unwrap();
    conn.execute(
        "create table if not exists errors(series integer, cid integer, value text);",
        params![],
    )
    .unwrap();
    conn.execute(
        "create table if not exists pstat(series integer, cid integer, value real);",
        params![],
    )
    .unwrap();
    conn.execute(
        "create table if not exists self_profile_query(
            series integer,
            cid integer,
            self_time integer,
            blocked_time integer,
            incremental_load_time integer,
            number_of_cache_hits integer,
            invocation_count integer
        );",
        params![],
    )
    .unwrap();

    // otherwise, we'll be really slow.
    conn.execute("drop index if exists self_profile_query_sc;", params![])
        .unwrap();

    conn.execute("drop index if exists pstat_sc;", params![])
        .unwrap();

    let paths = std::env::args().skip(2).collect::<Vec<_>>();
    let paths_count = paths.len();
    let mut last = std::time::Instant::now();
    for (idx, path) in paths.into_iter().enumerate() {
        if idx % 10 == 0 {
            eprintln!(
                "{}/{}, per {:?}; estimated total time {:?}",
                idx,
                paths_count,
                last.elapsed() / 10,
                last.elapsed() / 10 * paths_count as u32
            );
            last = std::time::Instant::now();
        }
        site::ingest::ingest(&conn, &mut index, Path::new(&path));
    }

    // reset to good default settings
    conn.pragma_update(None, "journal_mode", &"DELETE").unwrap();
    conn.pragma_update(None, "synchronous", &"FULL").unwrap();
    index.store(&conn);

    conn.execute(
        "create index if not exists self_profile_query_sc on self_profile_query(series, cid);",
        params![],
    )
    .unwrap();

    conn.execute(
        "create index if not exists pstat_sc on pstat(series, cid);",
        params![],
    )
    .unwrap();

    conn.execute("analyze;", params![]).unwrap();
}
