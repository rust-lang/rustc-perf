use crate::utils::mangling::demangle_file;
use anyhow::Context;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::Command;
use std::{fs, io};

/// Annotate and demangle the output of Cachegrind using the `cg_annotate` tool.
pub fn cachegrind_annotate(
    input: &Path,
    cgout_output: &Path,
    cgann_output: &Path,
) -> anyhow::Result<()> {
    let tmp_cgout = tempfile::NamedTempFile::new()?.into_temp_path();

    // It's useful to filter all `file:function` entries from
    // jemalloc into a single fake
    // `<all-jemalloc-files>:<all-jemalloc-functions>` entry. That
    // way the cost of all allocations is visible in one line,
    // rather than spread across many small entries.
    //
    // The downside is that we don't get any annotations within
    // jemalloc source files, but this is no real loss, given that
    // jemalloc is basically a black box whose code we never look
    // at anyway. DHAT is the best way to profile allocations.
    let reader = io::BufReader::new(fs::File::open(input)?);
    let mut writer = io::BufWriter::new(fs::File::create(&tmp_cgout)?);
    let mut in_jemalloc_file = false;

    // A Cachegrind profile contains `fn=<function-name>` lines,
    // `fl=<filename>` lines, and everything else. We just need to
    // modify the `fn=` and `fl=` lines that refer to jemalloc
    // code.
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("fl=") {
            // All jemalloc filenames have `/jemalloc/` or
            // something like `/jemalloc-sys-1e20251078fe5355/` in
            // them.
            in_jemalloc_file = line.contains("/jemalloc");
            if in_jemalloc_file {
                writeln!(writer, "fl=<all-jemalloc-files>")?;
                continue;
            }
        } else if line.starts_with("fn=") {
            // Any function within a jemalloc file is a jemalloc
            // function.
            if in_jemalloc_file {
                writeln!(writer, "fn=<all-jemalloc-functions>")?;
                continue;
            }
        }
        writeln!(writer, "{}", line)?;
    }
    writer.flush()?;

    demangle_file(&tmp_cgout, cgout_output).context("Cannot demangle cgout file")?;

    let mut cg_annotate_cmd = Command::new("cg_annotate");
    cg_annotate_cmd
        .arg("--auto=yes")
        .arg("--show-percs=yes")
        .arg(cgout_output);
    fs::write(cgann_output, cg_annotate_cmd.output()?.stdout)?;
    Ok(())
}
