use collector::{ArtifactData, CommitData};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir("rustc-timing/times")? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            continue;
        }

        if entry.path().extension().map_or(true, |e| e != "bincode") {
            continue;
        }

        let filename = entry.file_name();
        let filename = filename.to_str().unwrap();
        let file_contents = fs::read(entry.path())?;
        if filename.starts_with("artifact-") {
            let contents: ArtifactData = match bincode::deserialize(&file_contents) {
                Ok(j) => j,
                Err(err) => {
                    panic!("Failed to parse for {:?}: {:?}", filename, err);
                }
            };
            let encoded = serde_json::to_string(&contents)?;
            fs::remove_file(entry.path())?;
            fs::write(entry.path().with_extension("json"), &encoded)?;
        } else {
            let contents: CommitData = match bincode::deserialize(&file_contents) {
                Ok(json) => json,
                Err(err) => {
                    panic!("Failed to parse for {:?}: {:?}", filename, err);
                }
            };
            let encoded = serde_json::to_string(&contents)?;
            fs::remove_file(entry.path())?;
            fs::write(entry.path().with_extension("json"), &encoded)?;
        }
    }

    Ok(())
}
