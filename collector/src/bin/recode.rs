use collector::{CommitData};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir("rustc-timing/times")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }

        let filename = entry.file_name();
        let filename = filename.to_str().unwrap();
        let file_contents = fs::read(entry.path())?;
        if filename.starts_with("commit-") && filename.ends_with(".sz") {
            eprintln!("{:?}", filename);
            use std::io::Read;
            let mut out =
                String::with_capacity(snap::decompress_len(&file_contents).unwrap_or(0));
            let mut szip_reader = snap::Reader::new(&file_contents[..]);
            szip_reader.read_to_string(&mut out).unwrap();
            let file_contents = out.as_str();

            let contents: CommitData = match serde_json::from_str(&file_contents) {
                Ok(json) => json,
                Err(err) => {
                    log::error!("Failed to parse JSON for {}: {:?}", filename, err);
                    continue;
                }
            };
            let mut v = snap::Writer::new(Vec::new());
            serde_json::to_writer(&mut v, &contents)?;
            fs::write(entry.path(), v.into_inner()?)?;
        }
    }

    Ok(())
}
