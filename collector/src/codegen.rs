use std::collections::HashMap;
use std::io::{IsTerminal, Write};
use std::path::Path;
use std::process::Command;

use anyhow::Context;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use similar::{ChangeTag, DiffOp, TextDiff};

use crate::command_output;
use crate::runtime::BenchmarkGroupCrate;
use crate::toolchain::Toolchain;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum CodegenType {
    /// Display assembly with Intel syntax.
    #[value(name = "asm")]
    Assembly,
    /// Display assembly with Intel syntax, interspersed with Rust source code.
    #[value(name = "asm-source")]
    AssemblyWithSource,
    /// Display LLVM IR.
    LLVM,
    /// Display MIR.
    MIR,
}

/// Write the codegen diff of all functions of the given runtime benchmark group between the two toolchains to stdout.
pub fn codegen_diff(
    codegen_type: CodegenType,
    toolchain1: Toolchain,
    toolchain2: Toolchain,
    group: BenchmarkGroupCrate,
) -> anyhow::Result<()> {
    if !check_cargo_asm() {
        return Err(anyhow::anyhow!("`cargo-show-asm` does not seem to be installed. Run `cargo install cargo-show-asm` first."));
    }

    // List functions and their indices from the baseline compiler
    let functions1 = gather_functions(&toolchain1, &group.path, codegen_type)?;

    // Create a map from function name to its index for the second compiler
    let function2_map: HashMap<String, u32> =
        gather_functions(&toolchain2, &group.path, codegen_type)?
            .into_iter()
            .map(|entry| (entry.name, entry.index))
            .collect();

    let mut diffs = functions1
        .par_iter()
        .map(|function| {
            // Find the corresponding index for this function in the second compiler
            match function2_map.get(&function.name) {
                Some(toolchain2_index) => {
                    let codegen1 =
                        get_codegen(&toolchain1, &group.path, codegen_type, function.index)
                            .context("Cannot generate codegen using compiler 1")?;
                    let codegen2 =
                        get_codegen(&toolchain2, &group.path, codegen_type, *toolchain2_index)
                            .context("Cannot generate codegen using compiler 2")?;
                    if codegen1.trim() != codegen2.trim() {
                        log::debug!("Analysed function {}", function.name);
                        return Ok(Some(CodegenDiff::new(function, codegen1, codegen2)));
                    }
                }
                None => {
                    log::warn!(
                        "Could not find index for function {} for the second compiler",
                        function.name
                    );
                }
            }
            Ok::<_, anyhow::Error>(None)
        })
        .filter_map(|res| res.transpose())
        .collect::<anyhow::Result<Vec<CodegenDiff>>>()?;

    let group_namespace = get_namespace(&group.path)?;

    diffs.sort_by(|a, b| {
        let a_local = a.function.name.contains(&group_namespace);
        let b_local = b.function.name.contains(&group_namespace);

        a_local
            // Try to put functions from the benchmark itself first
            .cmp(&b_local)
            // We want true to be before false here
            .reverse()
            // Then sort in descending order by most insertions/deletions in the diff.
            // Replacements are often benign changes, like different assembly labels/register
            // numbers or LLVM IR SSA indices.
            .then(a.large_changes.cmp(&b.large_changes).reverse())
            // Then sort in descending order by most overall changes in the diff
            .then(a.similarity_ratio.partial_cmp(&b.similarity_ratio).unwrap())
            // If all else is equal, just sort by function name
            .then(a.function.name.cmp(&b.function.name))
    });

    let mut output = std::io::stdout().lock();
    let use_color = output.is_terminal();
    write_stats(&mut output, &diffs).context("Cannot write stats")?;
    for diff in diffs {
        write_diff(&mut output, use_color, &diff).context("Cannot write diff")?;
    }

    Ok(())
}

/// Resolve the Cargo crate namespace from the given directory.
fn get_namespace(directory: &Path) -> anyhow::Result<String> {
    let output: serde_json::Value = serde_json::from_slice(
        &command_output(
            Command::new("cargo")
                .arg("read-manifest")
                .current_dir(directory),
        )?
        .stdout,
    )?;
    output
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.replace('-', "_"))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Cannot read runtime benchmark group name from {}",
                directory.display()
            )
        })
}

struct CodegenDiff<'a> {
    function: &'a FunctionEntry,
    codegen1: String,
    codegen2: String,
    // How many insertions/removals (rather than replacements) are in this diff?
    large_changes: u64,
    // How much is codegen1 same as codegen2. 1.0 means that they are the same.
    similarity_ratio: f32,
}

impl<'a> CodegenDiff<'a> {
    fn new(function: &'a FunctionEntry, codegen1: String, codegen2: String) -> Self {
        let diff = TextDiff::from_lines(&codegen1, &codegen2);
        let similarity_ratio = diff.ratio();
        let large_changes = diff
            .ops()
            .iter()
            .filter(|op| matches!(op, DiffOp::Insert { .. } | DiffOp::Delete { .. }))
            .count() as u64;

        Self {
            function,
            codegen1,
            codegen2,
            large_changes,
            similarity_ratio,
        }
    }
}

fn write_stats<W: Write>(writer: &mut W, diffs: &[CodegenDiff]) -> anyhow::Result<()> {
    writeln!(writer, "Function size stats:")?;
    for diff in diffs {
        let size_before = diff.codegen1.len();
        let size_after = diff.codegen2.len();
        if size_before == size_after {
            continue;
        }
        let percent = (size_after as f64 / size_before as f64) - 1.0;
        let percent = percent * 100.0;
        writeln!(
            writer,
            "{}: {size_before} -> {size_after} ({}{:.2}%)",
            diff.function.name,
            if percent.is_sign_positive() { "+" } else { "-" },
            percent.abs()
        )?;
    }
    writer.write_all(b"\n")?;

    Ok(())
}

fn write_diff<W: Write>(writer: &mut W, use_color: bool, diff: &CodegenDiff) -> anyhow::Result<()> {
    use console::Style;

    let text_diff = TextDiff::from_lines(&diff.codegen1, &diff.codegen2);

    writeln!(writer, "Diff for {}", diff.function.name)?;
    writeln!(writer, "-----------------------------")?;
    for change in text_diff.iter_all_changes() {
        let (sign, style) = match change.tag() {
            ChangeTag::Delete => ("-", Style::new().red()),
            ChangeTag::Insert => ("+", Style::new().green()),
            ChangeTag::Equal => (" ", Style::new()),
        };
        if use_color {
            write!(
                writer,
                "{}{}",
                style.apply_to(sign).bold(),
                style.apply_to(change)
            )?;
        } else {
            write!(writer, "{}{}", sign, change)?;
        }
    }
    writeln!(writer, "-----------------------------")?;

    Ok(())
}

type FunctionIndex = u32;

fn get_codegen(
    toolchain: &Toolchain,
    dir: &Path,
    codegen: CodegenType,
    function_index: FunctionIndex,
) -> anyhow::Result<String> {
    let mut cmd = cargo_asm(toolchain, dir, codegen);
    match codegen {
        CodegenType::Assembly | CodegenType::AssemblyWithSource => {
            // Simplify assembly output to remove unneeded stuff
            cmd.arg("--simplify");
        }
        CodegenType::LLVM | CodegenType::MIR => {}
    }
    cmd.arg(format!("{}", function_index));

    Ok(String::from_utf8(command_output(&mut cmd)?.stdout)?)
}

#[derive(Debug)]
struct FunctionEntry {
    index: FunctionIndex,
    name: String,
}

fn gather_functions(
    toolchain: &Toolchain,
    dir: &Path,
    codegen: CodegenType,
) -> anyhow::Result<Vec<FunctionEntry>> {
    log::info!("Gathering functions from toolchain {}", toolchain.id);
    let output = cargo_asm(toolchain, dir, codegen).output()?;
    let output = String::from_utf8_lossy(&output.stdout);

    let functions: Vec<FunctionEntry> = output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if let Some((index, rest)) = line.split_once(' ') {
                let index = index.parse::<u32>().ok();
                let function = rest.trim_start_matches('"');
                let function = function.rsplit_once('"').map(|v| v.0);

                if let (Some(index), Some(function)) = (index, function) {
                    return Some(FunctionEntry {
                        index,
                        name: function.to_string(),
                    });
                }
            }
            None
        })
        .collect();
    log::info!("Found {} function(s)", functions.len());
    Ok(functions)
}

fn cargo_asm(toolchain: &Toolchain, dir: &Path, codegen: CodegenType) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("asm").arg("--no-color");
    cmd.current_dir(dir);

    cmd.env("RUSTC", &toolchain.components.rustc);

    // We want to make sure that the symbol names of functions will be as similar as possible
    // between any two versions of the compiler. It seems that using the `v0` mangling scheme is
    // better in this regard vs the `legacy` scheme. It also produces more understandable names
    // of symbols.
    cmd.env("RUSTFLAGS", "-Csymbol-mangling-version=v0");

    match codegen {
        CodegenType::Assembly => {
            cmd.arg("--intel");
        }
        CodegenType::AssemblyWithSource => {
            cmd.arg("--intel").arg("--rust");
        }
        CodegenType::LLVM => {
            cmd.arg("--llvm");
        }
        CodegenType::MIR => {
            cmd.arg("--mir");
        }
    }

    cmd
}

/// Checks if `cargo-show-asm` is installed and available under `cargo asm`.
fn check_cargo_asm() -> bool {
    let output = Command::new("cargo").arg("--list").output().unwrap();
    let output = String::from_utf8_lossy(&output.stdout);
    output
        .lines()
        .any(|line| line.split_whitespace().next() == Some("asm"))
}
