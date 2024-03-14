use anyhow::Context as _;
use ruzstd::StreamingDecoder;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let destination = std::env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("destination file should be first argument"))?;
    let resp = reqwest::blocking::get("https://perf-data.rust-lang.org/export.db.zst")
        .context("fetching database")?;
    let resp = resp.error_for_status().context("fetching database")?;

    let mut reader = std::io::BufReader::new(resp);
    let mut decoder = StreamingDecoder::new(&mut reader).context("cannot decode")?;
    let mut file = std::io::BufWriter::new(
        std::fs::File::create(destination).context("creating destination file")?,
    );
    std::io::copy(&mut decoder, &mut file).context("decoding into file")?;
    file.flush().context("flushing into file")?;
    Ok(())
}
