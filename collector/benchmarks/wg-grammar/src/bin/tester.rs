extern crate gll;
extern crate proc_macro2;
extern crate rust_grammar;
extern crate structopt;
extern crate walkdir;

use gll::runtime::{MoreThanOne, ParseNodeKind, ParseNodeShape};
use rust_grammar::parse;
use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "file")]
    /// Test parsing an individual Rust file
    File {
        #[structopt(parse(from_os_str), long = "graphviz-forest")]
        /// Dump the internal parse forest as a Graphviz .dot file
        graphviz_forest: Option<PathBuf>,

        #[structopt(parse(from_os_str))]
        /// Rust input file
        file: PathBuf,
    },

    #[structopt(name = "dir")]
    /// Test parsing a directory of Rust files
    Dir {
        #[structopt(short = "v", long = "verbose")]
        /// Print information about each file on stderr
        verbose: bool,

        #[structopt(parse(from_os_str))]
        /// Directory to find Rust files in
        dir: PathBuf,
    },
}

type ModuleContentsResult<'a, 'i> = parse::ParseResult<
    'a,
    'i,
    proc_macro2::TokenStream,
    parse::ModuleContents<'a, 'i, proc_macro2::TokenStream>,
>;

type ModuleContentsHandle<'a, 'i> = parse::Handle<
    'a,
    'i,
    proc_macro2::TokenStream,
    parse::ModuleContents<'a, 'i, proc_macro2::TokenStream>,
>;

/// Read the contents of the file at the given `path`, parse it
/// using the `ModuleContents` rule, and pass the result to `f`.
fn parse_file_with<R>(path: &Path, f: impl FnOnce(ModuleContentsResult) -> R) -> R {
    let src = fs::read_to_string(path).unwrap();
    match src.parse::<proc_macro2::TokenStream>() {
        Ok(tts) => parse::ModuleContents::parse_with(tts, |_, result| f(result)),
        // FIXME(eddyb) provide more information in this error case.
        Err(_) => f(Err(parse::ParseError::NoParse)),
    }
}

/// Output the result of a single file to stderr,
/// optionally prefixed by a given `path`.
fn report_file_result(
    path: Option<&Path>,
    result: ModuleContentsResult,
    ambiguity_result: Result<(), MoreThanOne>,
) {
    if let Some(path) = path {
        eprint!("{}: ", path.display());
    }
    // Avoid printing too much, especially not any parse nodes.
    match (result, ambiguity_result) {
        (Ok(_), Ok(_)) => eprintln!("OK"),
        (Ok(_), Err(_)) => eprintln!("OK (ambiguous)"),
        (Err(parse::ParseError::TooShort(handle)), _) => {
            eprint!("FAIL after ");

            #[cfg(procmacro2_semver_exempt)]
            {
                // HACK(eddyb) work around `proc-macro2` `Span` printing limitation
                let end_location = handle.source_info().end.end();
                eprintln!("{}:{}", end_location.line, end_location.column);
            }
            #[cfg(not(procmacro2_semver_exempt))]
            {
                let _ = handle;
                eprintln!(
                    "(missing location information; \
                     set `RUSTFLAGS='--cfg procmacro2_semver_exempt'`)"
                );
            }
        }
        (Err(parse::ParseError::NoParse), _) => eprintln!("FAIL (lexer error?)"),
    }
}

fn ambiguity_check(handle: ModuleContentsHandle) -> Result<(), MoreThanOne> {
    let sppf = &handle.parser.sppf;

    let mut queue = VecDeque::new();
    queue.push_back(handle.node);
    let mut seen: BTreeSet<_> = queue.iter().cloned().collect();

    while let Some(source) = queue.pop_front() {
        let mut add_children = |children: &[_]| {
            for &child in children {
                if seen.insert(child) {
                    queue.push_back(child);
                }
            }
        };
        match source.kind.shape() {
            ParseNodeShape::Opaque => {}
            ParseNodeShape::Alias(_) => add_children(&[source.unpack_alias()]),
            ParseNodeShape::Opt(_) => {
                if let Some(child) = source.unpack_opt() {
                    add_children(&[child]);
                }
            }
            ParseNodeShape::Choice => add_children(&[sppf.one_choice(source)?]),
            ParseNodeShape::Split(..) => {
                let (left, right) = sppf.one_split(source)?;
                add_children(&[left, right])
            }
        }
    }

    Ok(())
}

fn main() {
    match Command::from_args() {
        Command::File {
            graphviz_forest,
            file,
        } => {
            // Not much to do, try to parse the file and report the result.
            parse_file_with(&file, |result| {
                let mut ambiguity_result = Ok(());
                match result {
                    Ok(handle) | Err(parse::ParseError::TooShort(handle)) => {
                        ambiguity_result = ambiguity_check(handle);

                        if let Some(out_path) = graphviz_forest {
                            handle
                                .parser
                                .sppf
                                .dump_graphviz(&mut fs::File::create(out_path).unwrap())
                                .unwrap();
                        }
                    }
                    Err(parse::ParseError::NoParse) => {}
                }
                report_file_result(None, result, ambiguity_result);
            });
        }
        Command::Dir { verbose, dir } => {
            // Counters for reporting overall stats at the end.
            let mut total_count = 0;
            let mut unambiguous_count = 0;
            let mut ambiguous_count = 0;
            let mut too_short_count = 0;
            let mut no_parse_count = 0;

            // Find all the `.rs` files inside the desired directory.
            let files = WalkDir::new(dir)
                .contents_first(true)
                .into_iter()
                .map(|entry| entry.unwrap())
                .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "rs"));

            let mut stdout = io::stdout();

            // Go through all the files and try to parse each of them.
            for file in files {
                let path = file.into_path();

                total_count += 1;
                if !verbose {
                    // Limit the compact output to 80 columns wide.
                    if total_count % 80 == 0 {
                        println!("");
                    }
                }

                // HACK(eddyb) avoid parsing some files that hit
                // `lykenware/gll` worst-cases (many GBs of RAM usage)
                // FIXME(eddyb) fix the problems (e.g. implement GC).
                const BLACKLIST: &[&str] = &[
                    "libcore/unicode/tables.rs",
                    "issues/issue-29466.rs",
                    "issues/issue-29227.rs",
                ];
                if BLACKLIST.iter().any(|&b| path.ends_with(b)) {
                    if verbose {
                        eprintln!("{}: SKIP (blacklisted)...", path.display());
                    } else {
                        print!("S");
                        stdout.flush().unwrap();
                    }
                    continue;
                }

                // Indicate the current file being parsed in verbose mode.
                // This can be used to find files to blacklist (see above).
                if verbose {
                    eprint!("{}...\r", path.display());
                }

                parse_file_with(&path, |result| {
                    // Increment counters and figure out the character to print.
                    let mut ambiguity_result = Ok(());
                    let (status, count) = match result {
                        Ok(handle) => {
                            ambiguity_result = ambiguity_check(handle);
                            if ambiguity_result.is_ok() {
                                ('.', &mut unambiguous_count)
                            } else {
                                ('-', &mut ambiguous_count)
                            }
                        }
                        Err(parse::ParseError::TooShort(_)) => ('X', &mut too_short_count),
                        Err(parse::ParseError::NoParse) => ('L', &mut no_parse_count),
                    };
                    *count += 1;

                    if verbose {
                        // Unless we're in verbose mode, in which case we print more.
                        report_file_result(Some(&path), result, ambiguity_result);
                    } else {
                        print!("{}", status);
                        stdout.flush().unwrap();
                    }
                })
            }

            // We're done, time to print out stats!
            println!("");
            println!("Out of {} Rust files tested:", total_count);
            println!("* {} parsed fully and unambiguously", unambiguous_count);
            println!("* {} parsed fully (but ambiguously)", ambiguous_count);
            println!("* {} parsed partially (only a prefix)", too_short_count);
            println!("* {} didn't parse at all (lexer error?)", no_parse_count);
        }
    }
}
