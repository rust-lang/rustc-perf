//! Tests for `find_reviewers_from_diff`

use super::super::*;
use std::fmt::Write;

fn test_from_diff(diff: &Vec<FileDiff>, config: toml::Table, expected: &[&str]) {
    let aconfig: AssignConfig = config.try_into().unwrap();
    assert_eq!(
        find_reviewers_from_diff(&aconfig, &*diff).unwrap(),
        expected.iter().map(|x| x.to_string()).collect::<Vec<_>>()
    );
}

/// Generates a fake diff that touches the given files.
///
/// `paths` should be a slice of `(path, added, removed)` tuples where `added`
/// is the number of lines added, and `removed` is the number of lines
/// removed.
fn make_fake_diff(paths: &[(&str, u32, u32)]) -> Vec<FileDiff> {
    // This isn't a properly structured diff, but it has approximately enough
    // information for what is needed for testing.
    paths
        .iter()
        .map(|(path, added, removed)| {
            let mut diff = "@@ -0,0 +1 @@ ".to_string();
            for n in 0..*added {
                writeln!(diff, "+Added line {n}").unwrap();
            }
            for n in 0..*removed {
                writeln!(diff, "-Removed line {n}").unwrap();
            }
            diff.push('\n');
            FileDiff {
                filename: path.to_string(),
                patch: diff,
            }
        })
        .collect()
}

#[test]
fn no_matching_owners() {
    // When none of the owners match the diff.
    let config = toml::toml!(
        [owners]
        "/compiler" = ["compiler"]
        "/library" = ["libs"]
    );
    let diff = make_fake_diff(&[("foo/bar.rs", 5, 0)]);
    test_from_diff(&diff, config, &[]);
}

#[test]
fn from_diff_submodule() {
    let config = toml::toml!(
        [owners]
        "/src" = ["user1", "user2"]
    );
    let diff = vec![FileDiff {
        filename: "src/jemalloc".to_string(),
        patch: "@@ -1 +1 @@\n\
            -Subproject commit 2dba541881fb8e35246d653bbe2e7c7088777a4a\n\
            +Subproject commit b001609960ca33047e5cbc5a231c1e24b6041d4b\n\
        "
        .to_string(),
    }];
    test_from_diff(&diff, config, &["user1", "user2"]);
}

#[test]
fn prefixed_dirs() {
    // Test dirs with multiple overlapping prefixes.
    let config = toml::toml!(
        [owners]
        "/compiler" = ["compiler"]
        "/compiler/rustc_llvm" = ["llvm"]
        "/compiler/rustc_parse" = ["parser"]
        "/compiler/rustc_parse/src/parse/lexer" = ["lexer"]
    );
    // Base compiler rule should catch anything in compiler/
    let diff = make_fake_diff(&[("compiler/foo", 1, 1)]);
    test_from_diff(&diff, config.clone(), &["compiler"]);

    // Anything in rustc_llvm should go to llvm.
    let diff = make_fake_diff(&[("compiler/rustc_llvm/foo", 1, 1)]);
    test_from_diff(&diff, config.clone(), &["llvm"]);

    // 1 change in rustc_llvm, multiple changes in other directories, the
    // other directories win because they have more changes.
    let diff = make_fake_diff(&[
        ("compiler/rustc_llvm/foo", 1, 1),
        ("compiler/rustc_traits/src/foo.rs", 0, 1),
        ("compiler/rustc_macros//src/foo.rs", 2, 3),
    ]);
    test_from_diff(&diff, config.clone(), &["compiler"]);

    // Change in a deeply nested directory should win over its parent.
    let diff = make_fake_diff(&[("compiler/rustc_parse/src/parse/lexer/foo.rs", 1, 1)]);
    test_from_diff(&diff, config.clone(), &["lexer"]);

    // Most changes in one component should win over the base compiler.
    let diff = make_fake_diff(&[
        ("compiler/rustc_parse/src/foo.rs", 5, 10),
        ("compiler/rustc_llvm/src/foo.rs", 1, 1),
    ]);
    test_from_diff(&diff, config.clone(), &["parser"]);
}

#[test]
fn deleted_file() {
    // Test dirs matching for a deleted file.
    let config = toml::toml!(
        [owners]
        "/compiler" = ["compiler"]
        "/compiler/rustc_parse" = ["parser"]
    );
    let diff = make_fake_diff(&[("compiler/rustc_parse/src/foo.rs", 0, 10)]);
    test_from_diff(&diff, config, &["parser"]);
}

#[test]
fn empty_file_still_counts() {
    let config = toml::toml!(
        [owners]
        "/compiler" = ["compiler"]
        "/compiler/rustc_parse" = ["parser"]
    );
    let diff = vec![FileDiff {
        filename: "compiler/rustc_parse/src/foo.rs".to_string(),
        patch: "new file mode 100644\n\
                index 0000000..e69de29\n"
            .to_string(),
    }];
    test_from_diff(&diff, config, &["parser"]);
}

#[test]
fn basic_gitignore_pattern() {
    let config = toml::toml!(
        [owners]
        "*.js" = ["javascript-reviewers"]
        "/compiler/rustc_parse" = ["parser"]
    );
    let diff = make_fake_diff(&[("src/librustdoc/html/static/js/settings.js", 10, 1)]);
    test_from_diff(&diff, config, &["javascript-reviewers"]);
}
