use anyhow::Context;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Demangles Rust symbols in a file and writes result to path.
pub fn demangle_file(file: &Path, path: &Path) -> anyhow::Result<()> {
    let mut file =
        BufReader::new(File::open(file).context("Cannot open file containing Rust symbols")?);
    let mut output = BufWriter::new(
        File::create(path).context("Cannot create file with demangled Rust symbols")?,
    );

    rustc_demangle::demangle_stream(&mut file, &mut output, false)
        .context("Cannot demangle symbols")?;

    Ok(())
}
