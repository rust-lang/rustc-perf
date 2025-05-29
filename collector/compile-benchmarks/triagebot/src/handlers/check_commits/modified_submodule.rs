use std::sync::LazyLock;

use regex::{Regex, RegexBuilder};

use crate::github::FileDiff;

const SUBMODULE_WARNING_MSG: &str = "Some commits in this PR modify **submodules**.";

static SUBPROJECT_COMMIT_RE: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"^\+Subproject commit [a-zA-Z0-9]+$")
        .multi_line(true)
        .build()
        .unwrap()
});

/// Returns a message if the PR modifies a git submodule.
pub(super) fn modifies_submodule(diff: &[FileDiff]) -> Option<String> {
    if diff
        .iter()
        .any(|fd| SUBPROJECT_COMMIT_RE.is_match(&fd.patch))
    {
        Some(SUBMODULE_WARNING_MSG.to_string())
    } else {
        None
    }
}

#[test]
fn no_submodule_update() {
    let filediff = FileDiff {
        filename: "src/lib.rs".to_string(),
        patch: "@@ -1 +1 @@\
            -let mut my_var = 5;\
            +let mut my_var = \"tmp\";"
            .to_string(),
    };

    assert_eq!(modifies_submodule(&[filediff]), None)
}

#[test]
fn simple_submodule_update() {
    // Taken from https://api.github.com/repos/rust-lang/rust/compare/5af801b687e6e8b860ae970e725c8b9a3820d0ce...d6c4ab81be200855df856468ddedde057958441a
    let filediff = FileDiff {
        filename: "src/tools/rustc-perf".to_string(),
        patch: "@@ -1 +1 @@\n\
            -Subproject commit c0f3b53c8e5de87714d18a5f42998859302ae03a\n\
            +Subproject commit 8158f78f738715c060d230351623a7f7cc01bf97"
            .to_string(),
    };

    assert_eq!(
        modifies_submodule(&[filediff]),
        Some(SUBMODULE_WARNING_MSG.to_string())
    )
}

#[test]
fn no_submodule_update_tricky_case() {
    let filediff = FileDiff {
        filename: "src/tools.sh".to_string(),
        patch: "@@ -1 +1 @@\
            -let mut subproject_commit = 5;\
            +let mut subproject_commit = \"+Subproject commit \";"
            .to_string(),
    };

    assert_eq!(modifies_submodule(&[filediff]), None)
}
