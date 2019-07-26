use collector::{ArtifactData, CommitData};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir("rustc-timing/times")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }

        let filename = entry.file_name();
        let filename = filename.to_str().unwrap();
        if Path::new(&filename).extension().unwrap_or_default() == "json" {
            let file_contents = fs::read_to_string(entry.path())?;
            if filename.starts_with("artifact-") {
                let contents: ArtifactData = match serde_json::from_str(&file_contents) {
                    Ok(j) => j,
                    Err(err) => {
                        panic!("Failed to parse JSON for {:?}: {:?}", filename, err);
                    }
                };
                let encoded = bincode::serialize(&contents)?;
                fs::write(entry.path().with_extension("bincode"), &encoded)?;
            } else {
                let contents: CommitData = match serde_json::from_str(&file_contents) {
                    Ok(json) => json,
                    Err(err) => {
                        panic!("Failed to parse JSON for {:?}: {:?}", filename, err);
                    }
                };
                let encoded = bincode::serialize(&contents)?;
                fs::write(entry.path().with_extension("bincode"), &encoded)?;
            }
        }
    }

    Ok(())
}
