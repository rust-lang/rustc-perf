use anyhow::Context as _;
use snap::read::FrameDecoder;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let destination = std::env::args()
        .nth(1)
        .ok_or(anyhow::anyhow!("destination file should be first argument"))?;
    let resp = reqwest::blocking::get("https://perf-data.rust-lang.org/export.db.sz")
        .context("fetching database")?;
    let resp = resp.error_for_status().context("fetching database")?;
    let mut decoder = FrameDecoder::new(std::io::BufReader::new(resp));
    let mut file = std::io::BufWriter::new(
        std::fs::File::create(destination).context("creating destination file")?,
    );
    std::io::copy(&mut decoder, &mut file).context("decoding into file")?;
    file.flush().context("flushing into file")?;
    Ok(())
}
