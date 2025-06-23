extern crate cargotest;
extern crate git2;
extern crate hamcrest;
extern crate url;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use cargotest::support::{execs, project};
use cargotest::support::registry::Package;
use cargotest::support::paths;
use cargotest::support::git;
use hamcrest::assert_that;

use url::Url;

fn find_index() -> PathBuf {
    let dir = paths::home().join(".cargo/registry/index");
    dir.read_dir().unwrap().next().unwrap().unwrap().path()
}

fn run_test(path_env: Option<&OsStr>) {
    const N: usize = 50;

    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies]
            bar = "*"
        "#)
        .file("src/lib.rs", "")
        .build();
    Package::new("bar", "0.1.0").publish();

    assert_that(foo.cargo("build"),
                execs().with_status(0));

    let index = find_index();
    let path = paths::home().join("tmp");
    let url = Url::from_file_path(&path).unwrap().to_string();
    let repo = git2::Repository::init(&path).unwrap();
    let index = git2::Repository::open(&index).unwrap();
    let mut cfg = repo.config().unwrap();
    cfg.set_str("user.email", "foo@bar.com").unwrap();
    cfg.set_str("user.name", "Foo Bar").unwrap();
    let mut cfg = index.config().unwrap();
    cfg.set_str("user.email", "foo@bar.com").unwrap();
    cfg.set_str("user.name", "Foo Bar").unwrap();

    for _ in 0..N {
        git::commit(&repo);
        index.remote_anonymous(&url).unwrap()
             .fetch(&["refs/heads/master:refs/remotes/foo/master"],
                    None,
                    None).unwrap();
    }
    drop((repo, index));
    Package::new("bar", "0.1.1").publish();

    let before = find_index().join(".git/objects/pack")
                    .read_dir().unwrap()
                    .count();
    assert!(before > N);

    let mut cmd = foo.cargo("update");
    cmd.env("__CARGO_PACKFILE_LIMIT", "10");
    if let Some(path) = path_env {
        cmd.env("PATH", path);
    }
    cmd.env("RUST_LOG", "trace");
    assert_that(cmd, execs().with_status(0));
    let after = find_index().join(".git/objects/pack")
                    .read_dir().unwrap()
                    .count();
    assert!(after < before,
            "packfiles before: {}\n\
             packfiles after:  {}", before, after);
}

#[test]
fn use_git_gc() {
    if Command::new("git").arg("--version").output().is_err() {
        return
    }
    run_test(None);
}

#[test]
// it looks like this test passes on some windows machines but not others,
// notably not on AppVeyor's machines. Sounds like another but for another day.
#[cfg_attr(windows, ignore)]
fn avoid_using_git() {
    let path = env::var_os("PATH").unwrap_or_default();
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    let idx = paths.iter().position(|p| {
        p.join("git").exists() || p.join("git.exe").exists()
    });
    match idx {
        Some(i) => { paths.remove(i); }
        None => return,
    }
    run_test(Some(&env::join_paths(&paths).unwrap()));
}
