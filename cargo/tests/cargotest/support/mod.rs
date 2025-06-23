use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::prelude::*;
use std::os;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;
use std::usize;

use serde_json::{self, Value};
use url::Url;
use hamcrest as ham;
use cargo::util::ProcessBuilder;
use cargo::util::{CargoError, CargoErrorKind, ProcessError};

use support::paths::CargoPathExt;

#[macro_export]
macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

pub mod paths;
pub mod git;
pub mod registry;
pub mod cross_compile;
pub mod publish;

/*
 *
 * ===== Builders =====
 *
 */

#[derive(PartialEq,Clone)]
struct FileBuilder {
    path: PathBuf,
    body: String
}

impl FileBuilder {
    pub fn new(path: PathBuf, body: &str) -> FileBuilder {
        FileBuilder { path: path, body: body.to_string() }
    }

    fn mk(&self) {
        self.dirname().mkdir_p();

        let mut file = fs::File::create(&self.path).unwrap_or_else(|e| {
            panic!("could not create file {}: {}", self.path.display(), e)
        });

        t!(file.write_all(self.body.as_bytes()));
    }

    fn dirname(&self) -> &Path {
        self.path.parent().unwrap()
    }
}

#[derive(PartialEq,Clone)]
struct SymlinkBuilder {
    dst: PathBuf,
    src: PathBuf,
}

impl SymlinkBuilder {
    pub fn new(dst: PathBuf, src: PathBuf) -> SymlinkBuilder {
        SymlinkBuilder { dst: dst, src: src }
    }

    #[cfg(unix)]
    fn mk(&self) {
        self.dirname().mkdir_p();
        t!(os::unix::fs::symlink(&self.dst, &self.src));
    }

    #[cfg(windows)]
    fn mk(&self) {
        self.dirname().mkdir_p();
        t!(os::windows::fs::symlink_file(&self.dst, &self.src));
    }

    fn dirname(&self) -> &Path {
        self.src.parent().unwrap()
    }
}

#[derive(PartialEq,Clone)]
pub struct Project{
    root: PathBuf,
}

#[must_use]
#[derive(PartialEq,Clone)]
pub struct ProjectBuilder {
    name: String,
    root: Project,
    files: Vec<FileBuilder>,
    symlinks: Vec<SymlinkBuilder>,
}

impl ProjectBuilder {
    pub fn root(&self) -> PathBuf {
        self.root.root()
    }

    pub fn build_dir(&self) -> PathBuf {
        self.root.build_dir()
    }

    pub fn target_debug_dir(&self) -> PathBuf {
        self.root.target_debug_dir()
    }

    pub fn new(name: &str, root: PathBuf) -> ProjectBuilder {
        ProjectBuilder {
            name: name.to_string(),
            root: Project{ root },
            files: vec![],
            symlinks: vec![],
        }
    }

    pub fn file<B: AsRef<Path>>(mut self, path: B,
                                body: &str) -> Self {
        self._file(path.as_ref(), body);
        self
    }

    fn _file(&mut self, path: &Path, body: &str) {
        self.files.push(FileBuilder::new(self.root.root.join(path), body));
    }

    pub fn symlink<T: AsRef<Path>>(mut self, dst: T,
                                   src: T) -> Self {
        self.symlinks.push(SymlinkBuilder::new(self.root.root.join(dst),
                                               self.root.root.join(src)));
        self
    }

    pub fn build(self) -> Project {
        // First, clean the directory if it already exists
        self.rm_root();

        // Create the empty directory
        self.root.root.mkdir_p();

        for file in self.files.iter() {
            file.mk();
        }

        for symlink in self.symlinks.iter() {
            symlink.mk();
        }

        let ProjectBuilder{ name: _, root, files: _, symlinks: _, .. } = self;
        root
    }

    fn rm_root(&self) {
        self.root.root.rm_rf()
    }
}

impl Project {
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }

    pub fn build_dir(&self) -> PathBuf {
        self.root.join("target")
    }

    pub fn target_debug_dir(&self) -> PathBuf {
        self.build_dir().join("debug")
    }

    pub fn url(&self) -> Url { path2url(self.root()) }

    pub fn example_lib(&self, name: &str, kind: &str) -> PathBuf {
        let prefix = Project::get_lib_prefix(kind);

        let extension = Project::get_lib_extension(kind);

        let lib_file_name = format!("{}{}.{}",
                                    prefix,
                                    name,
                                    extension);

        self.target_debug_dir()
            .join("examples")
            .join(&lib_file_name)
    }

    pub fn bin(&self, b: &str) -> PathBuf {
        self.build_dir().join("debug").join(&format!("{}{}", b,
                                                     env::consts::EXE_SUFFIX))
    }

    pub fn release_bin(&self, b: &str) -> PathBuf {
        self.build_dir().join("release").join(&format!("{}{}", b,
                                                       env::consts::EXE_SUFFIX))
    }

    pub fn target_bin(&self, target: &str, b: &str) -> PathBuf {
        self.build_dir().join(target).join("debug")
                        .join(&format!("{}{}", b, env::consts::EXE_SUFFIX))
    }

    pub fn change_file(&self, path: &str, body: &str) {
        FileBuilder::new(self.root.join(path), body).mk()
    }

    pub fn process<T: AsRef<OsStr>>(&self, program: T) -> ProcessBuilder {
        let mut p = ::process(program);
        p.cwd(self.root());
        return p
    }

    pub fn cargo(&self, cmd: &str) -> ProcessBuilder {
        let mut p = self.process(&cargo_exe());
        p.arg(cmd);
        return p;
    }

    pub fn read_lockfile(&self) -> String {
        let mut buffer = String::new();
        fs::File::open(self.root().join("Cargo.lock")).unwrap()
            .read_to_string(&mut buffer).unwrap();
        buffer
    }

    fn get_lib_prefix(kind: &str) -> &str {
        match kind {
            "lib" | "rlib" => "lib",
            "staticlib" | "dylib" | "proc-macro" => {
                if cfg!(windows) {
                    ""
                } else {
                    "lib"
                }
            }
            _ => unreachable!()
        }
    }

    fn get_lib_extension(kind: &str) -> &str {
        match kind {
            "lib" | "rlib" => "rlib",
            "staticlib" => {
                if cfg!(windows) {
                    "lib"
                } else {
                    "a"
                }
            }
            "dylib" | "proc-macro" => {
                if cfg!(windows) {
                    "dll"
                } else if cfg!(target_os="macos") {
                    "dylib"
                } else {
                    "so"
                }
            }
            _ => unreachable!()
        }
    }
}

// Generates a project layout
pub fn project(name: &str) -> ProjectBuilder {
    ProjectBuilder::new(name, paths::root().join(name))
}

// Generates a project layout inside our fake home dir
pub fn project_in_home(name: &str) -> ProjectBuilder {
    ProjectBuilder::new(name, paths::home().join(name))
}

// === Helpers ===

pub fn main_file(println: &str, deps: &[&str]) -> String {
    let mut buf = String::new();

    for dep in deps.iter() {
        buf.push_str(&format!("extern crate {};\n", dep));
    }

    buf.push_str("fn main() { println!(");
    buf.push_str(&println);
    buf.push_str("); }\n");

    buf.to_string()
}

trait ErrMsg<T> {
    fn with_err_msg(self, val: String) -> Result<T, String>;
}

impl<T, E: fmt::Display> ErrMsg<T> for Result<T, E> {
    fn with_err_msg(self, val: String) -> Result<T, String> {
        match self {
            Ok(val) => Ok(val),
            Err(err) => Err(format!("{}; original={}", val, err))
        }
    }
}

// Path to cargo executables
pub fn cargo_dir() -> PathBuf {
    env::var_os("CARGO_BIN_PATH").map(PathBuf::from).or_else(|| {
        env::current_exe().ok().map(|mut path| {
            path.pop();
            if path.ends_with("deps") {
                path.pop();
            }
            path
        })
    }).unwrap_or_else(|| {
        panic!("CARGO_BIN_PATH wasn't set. Cannot continue running test")
    })
}

pub fn cargo_exe() -> PathBuf {
    cargo_dir().join(format!("cargo{}", env::consts::EXE_SUFFIX))
}

/// Returns an absolute path in the filesystem that `path` points to. The
/// returned path does not contain any symlinks in its hierarchy.
/*
 *
 * ===== Matchers =====
 *
 */

#[derive(Clone)]
pub struct Execs {
    expect_stdout: Option<String>,
    expect_stdin: Option<String>,
    expect_stderr: Option<String>,
    expect_exit_code: Option<i32>,
    expect_stdout_contains: Vec<String>,
    expect_stderr_contains: Vec<String>,
    expect_stdout_contains_n: Vec<(String, usize)>,
    expect_stdout_not_contains: Vec<String>,
    expect_stderr_not_contains: Vec<String>,
    expect_json: Option<Vec<Value>>,
}

impl Execs {
    pub fn with_stdout<S: ToString>(mut self, expected: S) -> Execs {
        self.expect_stdout = Some(expected.to_string());
        self
    }

    pub fn with_stderr<S: ToString>(mut self, expected: S) -> Execs {
        self._with_stderr(&expected);
        self
    }

    fn _with_stderr(&mut self, expected: &ToString) {
        self.expect_stderr = Some(expected.to_string());
    }

    pub fn with_status(mut self, expected: i32) -> Execs {
        self.expect_exit_code = Some(expected);
        self
    }

    pub fn with_stdout_contains<S: ToString>(mut self, expected: S) -> Execs {
        self.expect_stdout_contains.push(expected.to_string());
        self
    }

    pub fn with_stderr_contains<S: ToString>(mut self, expected: S) -> Execs {
        self.expect_stderr_contains.push(expected.to_string());
        self
    }

    pub fn with_stdout_contains_n<S: ToString>(mut self, expected: S, number: usize) -> Execs {
        self.expect_stdout_contains_n.push((expected.to_string(), number));
        self
    }

    pub fn with_stdout_does_not_contain<S: ToString>(mut self, expected: S) -> Execs {
        self.expect_stdout_not_contains.push(expected.to_string());
        self
    }

    pub fn with_stderr_does_not_contain<S: ToString>(mut self, expected: S) -> Execs {
        self.expect_stderr_not_contains.push(expected.to_string());
        self
    }

    pub fn with_json(mut self, expected: &str) -> Execs {
        self.expect_json = Some(expected.split("\n\n").map(|obj| {
            obj.parse().unwrap()
        }).collect());
        self
    }

    fn match_output(&self, actual: &Output) -> ham::MatchResult {
        self.match_status(actual)
            .and(self.match_stdout(actual))
            .and(self.match_stderr(actual))
    }

    fn match_status(&self, actual: &Output) -> ham::MatchResult {
        match self.expect_exit_code {
            None => ham::success(),
            Some(code) => {
                ham::expect(
                    actual.status.code() == Some(code),
                    format!("exited with {}\n--- stdout\n{}\n--- stderr\n{}",
                            actual.status,
                            String::from_utf8_lossy(&actual.stdout),
                            String::from_utf8_lossy(&actual.stderr)))
            }
        }
    }

    fn match_stdout(&self, actual: &Output) -> ham::MatchResult {
        self.match_std(self.expect_stdout.as_ref(), &actual.stdout,
                       "stdout", &actual.stderr, MatchKind::Exact)?;
        for expect in self.expect_stdout_contains.iter() {
            self.match_std(Some(expect), &actual.stdout, "stdout",
                           &actual.stderr, MatchKind::Partial)?;
        }
        for expect in self.expect_stderr_contains.iter() {
            self.match_std(Some(expect), &actual.stderr, "stderr",
                           &actual.stdout, MatchKind::Partial)?;
        }
        for &(ref expect, number) in self.expect_stdout_contains_n.iter() {
            self.match_std(Some(&expect), &actual.stdout, "stdout",
                           &actual.stderr, MatchKind::PartialN(number))?;
        }
        for expect in self.expect_stdout_not_contains.iter() {
            self.match_std(Some(expect), &actual.stdout, "stdout",
                           &actual.stderr, MatchKind::NotPresent)?;
        }
        for expect in self.expect_stderr_not_contains.iter() {
            self.match_std(Some(expect), &actual.stderr, "stderr",
                           &actual.stdout, MatchKind::NotPresent)?;
        }

        if let Some(ref objects) = self.expect_json {
            let stdout = str::from_utf8(&actual.stdout)
                .map_err(|_| "stdout was not utf8 encoded".to_owned())?;
            let lines = stdout.lines().collect::<Vec<_>>();
            if lines.len() != objects.len() {
                return Err(format!("expected {} json lines, got {}, stdout:\n{}",
                                   objects.len(), lines.len(), stdout));
            }
            for (obj, line) in objects.iter().zip(lines) {
                self.match_json(obj, line)?;
            }
        }
        Ok(())
    }

    fn match_stderr(&self, actual: &Output) -> ham::MatchResult {
        self.match_std(self.expect_stderr.as_ref(), &actual.stderr,
                       "stderr", &actual.stdout, MatchKind::Exact)
    }

    fn match_std(&self, expected: Option<&String>, actual: &[u8],
                 description: &str, extra: &[u8],
                 kind: MatchKind) -> ham::MatchResult {
        let out = match expected {
            Some(out) => out,
            None => return ham::success(),
        };
        let actual = match str::from_utf8(actual) {
            Err(..) => return Err(format!("{} was not utf8 encoded",
                                       description)),
            Ok(actual) => actual,
        };
        // Let's not deal with \r\n vs \n on windows...
        let actual = actual.replace("\r", "");
        let actual = actual.replace("\t", "<tab>");

        match kind {
            MatchKind::Exact => {
                let a = actual.lines();
                let e = out.lines();

                let diffs = self.diff_lines(a, e, false);
                ham::expect(diffs.is_empty(),
                            format!("differences:\n\
                                    {}\n\n\
                                    other output:\n\
                                    `{}`", diffs.join("\n"),
                                    String::from_utf8_lossy(extra)))
            }
            MatchKind::Partial => {
                let mut a = actual.lines();
                let e = out.lines();

                let mut diffs = self.diff_lines(a.clone(), e.clone(), true);
                while let Some(..) = a.next() {
                    let a = self.diff_lines(a.clone(), e.clone(), true);
                    if a.len() < diffs.len() {
                        diffs = a;
                    }
                }
                ham::expect(diffs.is_empty(),
                            format!("expected to find:\n\
                                     {}\n\n\
                                     did not find in output:\n\
                                     {}", out,
                                     actual))
            }
            MatchKind::PartialN(number) => {
                let mut a = actual.lines();
                let e = out.lines();

                let mut matches = 0;

                while let Some(..) = { 
                    if self.diff_lines(a.clone(), e.clone(), true).is_empty() {
                        matches += 1;
                    }
                    a.next()
                } {}
                
                ham::expect(matches == number,
                            format!("expected to find {} occurences:\n\
                                     {}\n\n\
                                     did not find in output:\n\
                                     {}", number, out,
                                     actual))
            }
            MatchKind::NotPresent => {
                ham::expect(!actual.contains(out),
                            format!("expected not to find:\n\
                                     {}\n\n\
                                     but found in output:\n\
                                     {}", out,
                                     actual))
            }
        }
    }

    fn match_json(&self, expected: &Value, line: &str) -> ham::MatchResult {
        let actual = match line.parse() {
             Err(e) => return Err(format!("invalid json, {}:\n`{}`", e, line)),
             Ok(actual) => actual,
        };

        match find_mismatch(expected, &actual) {
            Some((expected_part, actual_part)) => Err(format!(
                "JSON mismatch\nExpected:\n{}\nWas:\n{}\nExpected part:\n{}\nActual part:\n{}\n",
                serde_json::to_string_pretty(expected).unwrap(),
                serde_json::to_string_pretty(&actual).unwrap(),
                serde_json::to_string_pretty(expected_part).unwrap(),
                serde_json::to_string_pretty(actual_part).unwrap(),
            )),
            None => Ok(()),
        }
    }

    fn diff_lines<'a>(&self, actual: str::Lines<'a>, expected: str::Lines<'a>,
                      partial: bool) -> Vec<String> {
        let actual = actual.take(if partial {
            expected.clone().count()
        } else {
            usize::MAX
        });
        zip_all(actual, expected).enumerate().filter_map(|(i, (a,e))| {
            match (a, e) {
                (Some(a), Some(e)) => {
                    if lines_match(&e, &a) {
                        None
                    } else {
                        Some(format!("{:3} - |{}|\n    + |{}|\n", i, e, a))
                    }
                },
                (Some(a), None) => {
                    Some(format!("{:3} -\n    + |{}|\n", i, a))
                },
                (None, Some(e)) => {
                    Some(format!("{:3} - |{}|\n    +\n", i, e))
                },
                (None, None) => panic!("Cannot get here")
            }
        }).collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MatchKind {
    Exact,
    Partial,
    PartialN(usize),
    NotPresent,
}

pub fn lines_match(expected: &str, mut actual: &str) -> bool {
    let expected = substitute_macros(expected);
    for (i, part) in expected.split("[..]").enumerate() {
        match actual.find(part) {
            Some(j) => {
                if i == 0 && j != 0 {
                    return false
                }
                actual = &actual[j + part.len()..];
            }
            None => {
                return false
            }
        }
    }
    actual.is_empty() || expected.ends_with("[..]")
}

#[test]
fn lines_match_works() {
    assert!(lines_match("a b", "a b"));
    assert!(lines_match("a[..]b", "a b"));
    assert!(lines_match("a[..]", "a b"));
    assert!(lines_match("[..]", "a b"));
    assert!(lines_match("[..]b", "a b"));

    assert!(!lines_match("[..]b", "c"));
    assert!(!lines_match("b", "c"));
    assert!(!lines_match("b", "cb"));
}

// Compares JSON object for approximate equality.
// You can use `[..]` wildcard in strings (useful for OS dependent things such
// as paths).  You can use a `"{...}"` string literal as a wildcard for
// arbitrary nested JSON (useful for parts of object emitted by other programs
// (e.g. rustc) rather than Cargo itself).  Arrays are sorted before comparison.
fn find_mismatch<'a>(expected: &'a Value, actual: &'a Value)
                     -> Option<(&'a Value, &'a Value)> {
    use serde_json::Value::*;
    match (expected, actual) {
        (&Number(ref l), &Number(ref r)) if l == r => None,
        (&Bool(l), &Bool(r)) if l == r => None,
        (&String(ref l), &String(ref r)) if lines_match(l, r) => None,
        (&Array(ref l), &Array(ref r)) => {
            if l.len() != r.len() {
                return Some((expected, actual));
            }

            let mut l = l.iter().collect::<Vec<_>>();
            let mut r = r.iter().collect::<Vec<_>>();

            l.retain(|l| {
                match r.iter().position(|r| find_mismatch(l, r).is_none()) {
                    Some(i) => {
                        r.remove(i);
                        false
                    }
                    None => true
                }
            });

            if l.len() > 0 {
                assert!(r.len() > 0);
                Some((&l[0], &r[0]))
            } else {
                assert!(r.len() == 0);
                None
            }
        }
        (&Object(ref l), &Object(ref r)) => {
            let same_keys = l.len() == r.len() && l.keys().all(|k| r.contains_key(k));
            if !same_keys {
                return Some((expected, actual));
            }

            l.values().zip(r.values())
             .filter_map(|(l, r)| find_mismatch(l, r))
             .nth(0)
        }
        (&Null, &Null) => None,
        // magic string literal "{...}" acts as wildcard for any sub-JSON
        (&String(ref l), _) if l == "{...}" => None,
        _ => Some((expected, actual)),
    }

}

struct ZipAll<I1: Iterator, I2: Iterator> {
    first: I1,
    second: I2,
}

impl<T, I1: Iterator<Item=T>, I2: Iterator<Item=T>> Iterator for ZipAll<I1, I2> {
    type Item = (Option<T>, Option<T>);
    fn next(&mut self) -> Option<(Option<T>, Option<T>)> {
        let first = self.first.next();
        let second = self.second.next();

        match (first, second) {
            (None, None) => None,
            (a, b) => Some((a, b))
        }
    }
}

fn zip_all<T, I1: Iterator<Item=T>, I2: Iterator<Item=T>>(a: I1, b: I2) -> ZipAll<I1, I2> {
    ZipAll {
        first: a,
        second: b,
    }
}

impl fmt::Display for Execs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "execs")
    }
}

impl ham::Matcher<ProcessBuilder> for Execs {
    fn matches(&self, mut process: ProcessBuilder) -> ham::MatchResult {
        self.matches(&mut process)
    }
}

impl<'a> ham::Matcher<&'a mut ProcessBuilder> for Execs {
    fn matches(&self, process: &'a mut ProcessBuilder) -> ham::MatchResult {
        println!("running {}", process);
        let res = process.exec_with_output();

        match res {
            Ok(out) => self.match_output(&out),
            Err(CargoError(CargoErrorKind::ProcessErrorKind(
                ProcessError { output: Some(ref out), .. }), ..)) => {
                self.match_output(out)
            }
            Err(e) => {
                let mut s = format!("could not exec process {}: {}", process, e);
                match e.cause() {
                    Some(cause) => s.push_str(&format!("\ncaused by: {}",
                                                       cause.description())),
                    None => {}
                }
                Err(s)
            }
        }
    }
}

impl ham::Matcher<Output> for Execs {
    fn matches(&self, output: Output) -> ham::MatchResult {
        self.match_output(&output)
    }
}

pub fn execs() -> Execs {
    Execs {
        expect_stdout: None,
        expect_stderr: None,
        expect_stdin: None,
        expect_exit_code: None,
        expect_stdout_contains: Vec::new(),
        expect_stderr_contains: Vec::new(),
        expect_stdout_contains_n: Vec::new(),
        expect_stdout_not_contains: Vec::new(),
        expect_stderr_not_contains: Vec::new(),
        expect_json: None,
    }
}

#[derive(Clone)]
pub struct ShellWrites {
    expected: String
}

impl fmt::Display for ShellWrites {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}` written to the shell", self.expected)
    }
}

impl<'a> ham::Matcher<&'a [u8]> for ShellWrites {
    fn matches(&self, actual: &[u8])
        -> ham::MatchResult
    {
        let actual = String::from_utf8_lossy(actual);
        let actual = actual.to_string();
        ham::expect(actual == self.expected, actual)
    }
}

pub fn shell_writes<T: fmt::Display>(string: T) -> ShellWrites {
    ShellWrites { expected: string.to_string() }
}

pub trait Tap {
    fn tap<F: FnOnce(&mut Self)>(self, callback: F) -> Self;
}

impl<T> Tap for T {
    fn tap<F: FnOnce(&mut Self)>(mut self, callback: F) -> T {
        callback(&mut self);
        self
    }
}

pub fn basic_bin_manifest(name: &str) -> String {
    format!(r#"
        [package]

        name = "{}"
        version = "0.5.0"
        authors = ["wycats@example.com"]

        [[bin]]

        name = "{}"
    "#, name, name)
}

pub fn basic_lib_manifest(name: &str) -> String {
    format!(r#"
        [package]

        name = "{}"
        version = "0.5.0"
        authors = ["wycats@example.com"]

        [lib]

        name = "{}"
    "#, name, name)
}

pub fn path2url(p: PathBuf) -> Url {
    Url::from_file_path(&*p).ok().unwrap()
}

fn substitute_macros(input: &str) -> String {
    let macros = [
        ("[RUNNING]",     "     Running"),
        ("[COMPILING]",   "   Compiling"),
        ("[CREATED]",     "     Created"),
        ("[FINISHED]",    "    Finished"),
        ("[ERROR]",       "error:"),
        ("[WARNING]",     "warning:"),
        ("[DOCUMENTING]", " Documenting"),
        ("[FRESH]",       "       Fresh"),
        ("[UPDATING]",    "    Updating"),
        ("[ADDING]",      "      Adding"),
        ("[REMOVING]",    "    Removing"),
        ("[DOCTEST]",     "   Doc-tests"),
        ("[PACKAGING]",   "   Packaging"),
        ("[DOWNLOADING]", " Downloading"),
        ("[UPLOADING]",   "   Uploading"),
        ("[VERIFYING]",   "   Verifying"),
        ("[ARCHIVING]",   "   Archiving"),
        ("[INSTALLING]",  "  Installing"),
        ("[REPLACING]",   "   Replacing"),
        ("[UNPACKING]",   "   Unpacking"),
        ("[EXE]", if cfg!(windows) {".exe"} else {""}),
        ("[/]", if cfg!(windows) {"\\"} else {"/"}),
    ];
    let mut result = input.to_owned();
    for &(pat, subst) in macros.iter() {
        result = result.replace(pat, subst)
    }
    return result;
}
