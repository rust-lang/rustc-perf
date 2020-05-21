use std::path::Path;

fn main() {
    let dir = std::env::args().nth(1).expect("database as first arg");
    let db = site::db::open(&dir, true);
    let mut index = site::db::Index::load(&db);

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
        site::ingest::ingest(&db, &mut index, Path::new(&path));
    }

    index.store(&db);
    db.flush().unwrap();
}
