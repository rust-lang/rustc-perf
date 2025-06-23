#[macro_use]
extern crate cargotest;
extern crate hamcrest;

use std::io::{Read, Write};
use std::fs::File;

use cargotest::sleep_ms;
use cargotest::support::{project, execs, git};
use cargotest::support::registry::Package;
use hamcrest::{assert_that, existing_file, existing_dir, is_not};

#[test]
fn simple_explicit() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = ".."
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());

    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn inferred_root() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());

    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn inferred_path_dep() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = { path = "bar" }

            [workspace]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}")
        .file("bar/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());

    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn transitive_path_dep() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = { path = "bar" }

            [workspace]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            baz = { path = "../baz" }
        "#)
        .file("bar/src/main.rs", "fn main() {}")
        .file("bar/src/lib.rs", "")
        .file("baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.1.0"
            authors = []
        "#)
        .file("baz/src/main.rs", "fn main() {}")
        .file("baz/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), is_not(existing_file()));
    assert_that(&p.bin("baz"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());
    assert_that(&p.bin("baz"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("baz")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());
    assert_that(&p.bin("baz"), existing_file());

    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("baz/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn parent_pointer_works() {
    let p = project("foo")
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = { path = "../bar" }

            [workspace]
        "#)
        .file("foo/src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = "../foo"
        "#)
        .file("bar/src/main.rs", "fn main() {}")
        .file("bar/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("foo")),
                execs().with_status(0));
    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/Cargo.lock"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn same_names_in_workspace() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
            workspace = ".."
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: two packages named `foo` in this workspace:
- [..]Cargo.toml
- [..]Cargo.toml
"));
}

#[test]
fn parent_doesnt_point_to_child() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(101)
                       .with_stderr("\
error: current package believes it's in a workspace when it's not:
current: [..]Cargo.toml
workspace: [..]Cargo.toml

this may be fixable [..]
"));
}

#[test]
fn invalid_parent_pointer() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
            workspace = "foo"
        "#)
        .file("src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: failed to read `[..]Cargo.toml`

Caused by:
  [..]
"));
}

#[test]
fn invalid_members() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["foo"]
        "#)
        .file("src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: failed to read `[..]Cargo.toml`

Caused by:
  [..]
"));
}

#[test]
fn bare_workspace_ok() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
        "#)
        .file("src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
}

#[test]
fn two_roots() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []

            [workspace]
            members = [".."]
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: multiple workspace roots found in the same workspace:
  [..]
  [..]
"));
}

#[test]
fn workspace_isnt_root() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
            workspace = "bar"
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: root of a workspace inferred but wasn't a root: [..]
"));
}

#[test]
fn dangling_member() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = "../baz"
        "#)
        .file("bar/src/main.rs", "fn main() {}")
        .file("baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.1.0"
            authors = []
            workspace = "../baz"
        "#)
        .file("baz/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: package `[..]` is a member of the wrong workspace
expected: [..]
actual: [..]
"));
}

#[test]
fn cycle() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
            workspace = "bar"
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = ".."
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101));
}

#[test]
fn share_dependencies() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            dep1 = "0.1"

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            dep1 = "< 0.1.5"
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    Package::new("dep1", "0.1.3").publish();
    Package::new("dep1", "0.1.8").publish();

    assert_that(p.cargo("build"),
                execs().with_status(0)
                       .with_stderr("\
[UPDATING] registry `[..]`
[DOWNLOADING] dep1 v0.1.3 ([..])
[COMPILING] dep1 v0.1.3
[COMPILING] foo v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
}

#[test]
fn fetch_fetches_all() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            dep1 = "*"
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    Package::new("dep1", "0.1.3").publish();

    assert_that(p.cargo("fetch"),
                execs().with_status(0)
                       .with_stderr("\
[UPDATING] registry `[..]`
[DOWNLOADING] dep1 v0.1.3 ([..])
"));
}

#[test]
fn lock_works_for_everyone() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            dep2 = "0.1"

            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            dep1 = "0.1"
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    Package::new("dep1", "0.1.0").publish();
    Package::new("dep2", "0.1.0").publish();

    assert_that(p.cargo("generate-lockfile"),
                execs().with_status(0)
                       .with_stderr("\
[UPDATING] registry `[..]`
"));

    Package::new("dep1", "0.1.1").publish();
    Package::new("dep2", "0.1.1").publish();

    assert_that(p.cargo("build"),
                execs().with_status(0)
                       .with_stderr("\
[DOWNLOADING] dep2 v0.1.0 ([..])
[COMPILING] dep2 v0.1.0
[COMPILING] foo v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0)
                       .with_stderr("\
[DOWNLOADING] dep1 v0.1.0 ([..])
[COMPILING] dep1 v0.1.0
[COMPILING] bar v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
}

#[test]
fn virtual_works() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["bar"]
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();
    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(0));
    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.bin("bar"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn explicit_package_argument_works_with_virtual_manifest() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["bar"]
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();
    assert_that(p.cargo("build").cwd(p.root()).args(&["--package", "bar"]),
                execs().with_status(0));
    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.bin("bar"), existing_file());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
}

#[test]
fn virtual_misconfigure() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();
    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(101)
                       .with_stderr("\
error: current package believes it's in a workspace when it's not:
current: [..]bar[..]Cargo.toml
workspace: [..]Cargo.toml

this may be fixable by adding `bar` to the `workspace.members` array of the \
manifest located at: [..]
"));
}

#[test]
fn virtual_build_all_implied() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["bar"]
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();
    assert_that(p.cargo("build"),
                execs().with_status(0));
}

#[test]
fn virtual_build_no_members() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
        "#);
    let p = p.build();
    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: manifest path `[..]` contains no package: The manifest is virtual, \
and the workspace has no members.
"));
}

#[test]
fn include_virtual() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            [workspace]
            members = ["bar"]
        "#)
        .file("src/main.rs", "")
        .file("bar/Cargo.toml", r#"
            [workspace]
        "#);
    let p = p.build();
    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: multiple workspace roots found in the same workspace:
  [..]
  [..]
"));
}

#[test]
fn members_include_path_deps() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["p1"]

            [dependencies]
            p3 = { path = "p3" }
        "#)
        .file("src/lib.rs", "")
        .file("p1/Cargo.toml", r#"
            [project]
            name = "p1"
            version = "0.1.0"
            authors = []

            [dependencies]
            p2 = { path = "../p2" }
        "#)
        .file("p1/src/lib.rs", "")
        .file("p2/Cargo.toml", r#"
            [project]
            name = "p2"
            version = "0.1.0"
            authors = []
        "#)
        .file("p2/src/lib.rs", "")
        .file("p3/Cargo.toml", r#"
            [project]
            name = "p3"
            version = "0.1.0"
            authors = []
        "#)
        .file("p3/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("p1")),
                execs().with_status(0));
    assert_that(p.cargo("build").cwd(p.root().join("p2")),
                execs().with_status(0));
    assert_that(p.cargo("build").cwd(p.root().join("p3")),
                execs().with_status(0));
    assert_that(p.cargo("build"),
                execs().with_status(0));

    assert_that(&p.root().join("target"), existing_dir());
    assert_that(&p.root().join("p1/target"), is_not(existing_dir()));
    assert_that(&p.root().join("p2/target"), is_not(existing_dir()));
    assert_that(&p.root().join("p3/target"), is_not(existing_dir()));
}

#[test]
fn new_warns_you_this_will_not_work() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
        "#)
        .file("src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("new").arg("--lib").arg("bar").env("USER", "foo"),
                execs().with_status(0)
                       .with_stderr("\
warning: compiling this new crate may not work due to invalid workspace \
configuration

current package believes it's in a workspace when it's not:
current: [..]
workspace: [..]

this may be fixable by ensuring that this crate is depended on by the workspace \
root: [..]
[CREATED] library `bar` project
"));
}

#[test]
fn lock_doesnt_change_depending_on_crate() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ['baz']

            [dependencies]
            foo = "*"
        "#)
        .file("src/lib.rs", "")
        .file("baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = "*"
        "#)
        .file("baz/src/lib.rs", "");
    let p = p.build();

    Package::new("foo", "1.0.0").publish();
    Package::new("bar", "1.0.0").publish();

    assert_that(p.cargo("build"),
                execs().with_status(0));

    let mut lockfile = String::new();
    t!(t!(File::open(p.root().join("Cargo.lock"))).read_to_string(&mut lockfile));

    assert_that(p.cargo("build").cwd(p.root().join("baz")),
                execs().with_status(0));

    let mut lockfile2 = String::new();
    t!(t!(File::open(p.root().join("Cargo.lock"))).read_to_string(&mut lockfile2));

    assert_eq!(lockfile, lockfile2);
}

#[test]
fn rebuild_please() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ['lib', 'bin']
        "#)
        .file("lib/Cargo.toml", r#"
            [package]
            name = "lib"
            version = "0.1.0"
        "#)
        .file("lib/src/lib.rs", r#"
            pub fn foo() -> u32 { 0 }
        "#)
        .file("bin/Cargo.toml", r#"
            [package]
            name = "bin"
            version = "0.1.0"

            [dependencies]
            lib = { path = "../lib" }
        "#)
        .file("bin/src/main.rs", r#"
            extern crate lib;

            fn main() {
                assert_eq!(lib::foo(), 0);
            }
        "#);
    let p = p.build();

    assert_that(p.cargo("run").cwd(p.root().join("bin")),
                execs().with_status(0));

    sleep_ms(1000);

    t!(t!(File::create(p.root().join("lib/src/lib.rs"))).write_all(br#"
        pub fn foo() -> u32 { 1 }
    "#));

    assert_that(p.cargo("build").cwd(p.root().join("lib")),
                execs().with_status(0));

    assert_that(p.cargo("run").cwd(p.root().join("bin")),
                execs().with_status(101));
}

#[test]
fn workspace_in_git() {
    let git_project = git::new("dep1", |project| {
        project
            .file("Cargo.toml", r#"
                [workspace]
                members = ["foo"]
            "#)
            .file("foo/Cargo.toml", r#"
                [package]
                name = "foo"
                version = "0.1.0"
            "#)
            .file("foo/src/lib.rs", "")
    }).unwrap();
    let p = project("foo")
        .file("Cargo.toml", &format!(r#"
            [package]
            name = "lib"
            version = "0.1.0"

            [dependencies.foo]
            git = '{}'
        "#, git_project.url()))
        .file("src/lib.rs", r#"
            pub fn foo() -> u32 { 0 }
        "#);
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(0));
}


#[test]
fn lockfile_can_specify_nonexistant_members() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["a"]
        "#)
        .file("a/Cargo.toml", r#"
            [project]
            name = "a"
            version = "0.1.0"
            authors = []
        "#)
        .file("a/src/main.rs", "fn main() {}")
        .file("Cargo.lock", r#"
            [[package]]
            name = "a"
            version = "0.1.0"

            [[package]]
            name = "b"
            version = "0.1.0"
        "#);

    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("a")), execs().with_status(0));
}

#[test]
fn you_cannot_generate_lockfile_for_empty_workspaces() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("update"),
                execs().with_status(101)
                       .with_stderr("\
error: you can't generate a lockfile for an empty workspace.
"));
}

#[test]
fn workspace_with_transitive_dev_deps() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.5.0"
            authors = ["mbrubeck@example.com"]

            [dependencies.bar]
            path = "bar"

            [workspace]
        "#)
        .file("src/main.rs", r#"fn main() {}"#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.5.0"
            authors = ["mbrubeck@example.com"]

            [dev-dependencies.baz]
            path = "../baz"
        "#)
        .file("bar/src/lib.rs", r#"
            pub fn init() {}

            #[cfg(test)]

            #[test]
            fn test() {
                extern crate baz;
                baz::do_stuff();
            }
        "#)
        .file("baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.5.0"
            authors = ["mbrubeck@example.com"]
        "#)
        .file("baz/src/lib.rs", r#"pub fn do_stuff() {}"#);
    let p = p.build();

    assert_that(p.cargo("test").args(&["-p", "bar"]),
                execs().with_status(0));
}

#[test]
fn error_if_parent_cargo_toml_is_invalid() {
    let p = project("foo")
        .file("Cargo.toml", "Totally not a TOML file")
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("bar")),
                execs().with_status(101)
                       .with_stderr_contains("\
[ERROR] failed to parse manifest at `[..]`"));
}

#[test]
fn relative_path_for_member_works() {
    let p = project("foo")
        .file("foo/Cargo.toml", r#"
        [project]
        name = "foo"
        version = "0.1.0"
        authors = []

        [workspace]
        members = ["../bar"]
    "#)
        .file("foo/src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
        [project]
        name = "bar"
        version = "0.1.0"
        authors = []
        workspace = "../foo"
    "#)
        .file("bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("foo")), execs().with_status(0));
    assert_that(p.cargo("build").cwd(p.root().join("bar")), execs().with_status(0));
}

#[test]
fn relative_path_for_root_works() {
    let p = project("foo")
        .file("Cargo.toml", r#"
        [project]
        name = "foo"
        version = "0.1.0"
        authors = []

        [workspace]

        [dependencies]
        subproj = { path = "./subproj" }
    "#)
        .file("src/main.rs", "fn main() {}")
        .file("subproj/Cargo.toml", r#"
        [project]
        name = "subproj"
        version = "0.1.0"
        authors = []
    "#)
        .file("subproj/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root())
                    .arg("--manifest-path").arg("./Cargo.toml"),
                execs().with_status(0));

    assert_that(p.cargo("build").cwd(p.root().join("subproj"))
                    .arg("--manifest-path").arg("../Cargo.toml"),
                execs().with_status(0));
}

#[test]
fn path_dep_outside_workspace_is_not_member() {
    let p = project("foo")
        .file("ws/Cargo.toml", r#"
            [project]
            name = "ws"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = { path = "../foo" }

            [workspace]
        "#)
        .file("ws/src/lib.rs", r"extern crate foo;")
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("ws")),
                execs().with_status(0));
}

#[test]
fn test_in_and_out_of_workspace() {
    let p = project("foo")
        .file("ws/Cargo.toml", r#"
            [project]
            name = "ws"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = { path = "../foo" }

            [workspace]
            members = [ "../bar" ]
        "#)
        .file("ws/src/lib.rs", r"extern crate foo; pub fn f() { foo::f() }")
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = { path = "../bar" }
        "#)
        .file("foo/src/lib.rs", "extern crate bar; pub fn f() { bar::f() }")
        .file("bar/Cargo.toml", r#"
            [project]
            workspace = "../ws"
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/lib.rs", "pub fn f() { }");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("ws")),
                execs().with_status(0));

    assert_that(&p.root().join("ws/Cargo.lock"), existing_file());
    assert_that(&p.root().join("ws/target"), existing_dir());
    assert_that(&p.root().join("foo/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("foo/target"), is_not(existing_dir()));
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("bar/target"), is_not(existing_dir()));

    assert_that(p.cargo("build").cwd(p.root().join("foo")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/Cargo.lock"), existing_file());
    assert_that(&p.root().join("foo/target"), existing_dir());
    assert_that(&p.root().join("bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("bar/target"), is_not(existing_dir()));
}

#[test]
fn test_path_dependency_under_member() {
    let p = project("foo")
        .file("ws/Cargo.toml", r#"
            [project]
            name = "ws"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = { path = "../foo" }

            [workspace]
        "#)
        .file("ws/src/lib.rs", r"extern crate foo; pub fn f() { foo::f() }")
        .file("foo/Cargo.toml", r#"
            [project]
            workspace = "../ws"
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = { path = "./bar" }
        "#)
        .file("foo/src/lib.rs", "extern crate bar; pub fn f() { bar::f() }")
        .file("foo/bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/bar/src/lib.rs", "pub fn f() { }");
    let p = p.build();

    assert_that(p.cargo("build").cwd(p.root().join("ws")),
                execs().with_status(0));

    assert_that(&p.root().join("foo/bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("foo/bar/target"), is_not(existing_dir()));

    assert_that(p.cargo("build").cwd(p.root().join("foo/bar")),
                execs().with_status(0));

    assert_that(&p.root().join("foo/bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("foo/bar/target"), is_not(existing_dir()));
}

#[test]
fn excluded_simple() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "ws"
            version = "0.1.0"
            authors = []

            [workspace]
            exclude = ["foo"]
        "#)
        .file("src/lib.rs", "")
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(0));
    assert_that(&p.root().join("target"), existing_dir());
    assert_that(p.cargo("build").cwd(p.root().join("foo")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/target"), existing_dir());
}

#[test]
fn exclude_members_preferred() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "ws"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["foo/bar"]
            exclude = ["foo"]
        "#)
        .file("src/lib.rs", "")
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/src/lib.rs", "")
        .file("foo/bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/bar/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(0));
    assert_that(&p.root().join("target"), existing_dir());
    assert_that(p.cargo("build").cwd(p.root().join("foo")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/target"), existing_dir());
    assert_that(p.cargo("build").cwd(p.root().join("foo/bar")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/bar/target"), is_not(existing_dir()));
}

#[test]
fn exclude_but_also_depend() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "ws"
            version = "0.1.0"
            authors = []

            [dependencies]
            bar = { path = "foo/bar" }

            [workspace]
            exclude = ["foo"]
        "#)
        .file("src/lib.rs", "")
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/src/lib.rs", "")
        .file("foo/bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("foo/bar/src/lib.rs", "");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(0));
    assert_that(&p.root().join("target"), existing_dir());
    assert_that(p.cargo("build").cwd(p.root().join("foo")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/target"), existing_dir());
    assert_that(p.cargo("build").cwd(p.root().join("foo/bar")),
                execs().with_status(0));
    assert_that(&p.root().join("foo/bar/target"), existing_dir());
}

#[test]
fn glob_syntax() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["crates/*"]
            exclude = ["crates/qux"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("crates/bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = "../.."
        "#)
        .file("crates/bar/src/main.rs", "fn main() {}")
        .file("crates/baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.1.0"
            authors = []
            workspace = "../.."
        "#)
        .file("crates/baz/src/main.rs", "fn main() {}")
        .file("crates/qux/Cargo.toml", r#"
            [project]
            name = "qux"
            version = "0.1.0"
            authors = []
        "#)
        .file("crates/qux/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), is_not(existing_file()));
    assert_that(&p.bin("baz"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("crates/bar")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());

    assert_that(p.cargo("build").cwd(p.root().join("crates/baz")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("baz"), existing_file());

    assert_that(p.cargo("build").cwd(p.root().join("crates/qux")),
                execs().with_status(0));
    assert_that(&p.bin("qux"), is_not(existing_file()));

    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.root().join("crates/bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("crates/baz/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("crates/qux/Cargo.lock"), existing_file());
}

/*FIXME: This fails because of how workspace.exclude and workspace.members are working.
#[test]
fn glob_syntax_2() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["crates/b*"]
            exclude = ["crates/q*"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("crates/bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = "../.."
        "#)
        .file("crates/bar/src/main.rs", "fn main() {}")
        .file("crates/baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.1.0"
            authors = []
            workspace = "../.."
        "#)
        .file("crates/baz/src/main.rs", "fn main() {}")
        .file("crates/qux/Cargo.toml", r#"
            [project]
            name = "qux"
            version = "0.1.0"
            authors = []
        "#)
        .file("crates/qux/src/main.rs", "fn main() {}");
    p.build();

    assert_that(p.cargo("build"), execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), is_not(existing_file()));
    assert_that(&p.bin("baz"), is_not(existing_file()));

    assert_that(p.cargo("build").cwd(p.root().join("crates/bar")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("bar"), existing_file());

    assert_that(p.cargo("build").cwd(p.root().join("crates/baz")),
                execs().with_status(0));
    assert_that(&p.bin("foo"), existing_file());
    assert_that(&p.bin("baz"), existing_file());

    assert_that(p.cargo("build").cwd(p.root().join("crates/qux")),
                execs().with_status(0));
    assert_that(&p.bin("qux"), is_not(existing_file()));

    assert_that(&p.root().join("Cargo.lock"), existing_file());
    assert_that(&p.root().join("crates/bar/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("crates/baz/Cargo.lock"), is_not(existing_file()));
    assert_that(&p.root().join("crates/qux/Cargo.lock"), existing_file());
}
*/

#[test]
fn glob_syntax_invalid_members() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []

            [workspace]
            members = ["crates/*"]
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("crates/bar/src/main.rs", "fn main() {}");
    let p = p.build();

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: failed to read `[..]Cargo.toml`

Caused by:
  [..]
"));
}

/// This is a freshness test for feature use with workspaces
///
/// feat_lib is used by caller1 and caller2, but with different features enabled.
/// This test ensures that alternating building caller1, caller2 doesn't force
/// recompile of feat_lib.
///
/// Ideally once we solve https://github.com/rust-lang/cargo/issues/3620, then
/// a single cargo build at the top level will be enough.
#[test]
fn dep_used_with_separate_features() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["feat_lib", "caller1", "caller2"]
        "#)
        .file("feat_lib/Cargo.toml", r#"
            [project]
            name = "feat_lib"
            version = "0.1.0"
            authors = []

            [features]
            myfeature = []
        "#)
        .file("feat_lib/src/lib.rs", "")
        .file("caller1/Cargo.toml", r#"
            [project]
            name = "caller1"
            version = "0.1.0"
            authors = []

            [dependencies]
            feat_lib = { path = "../feat_lib" }
        "#)
        .file("caller1/src/main.rs", "fn main() {}")
        .file("caller1/src/lib.rs", "")
        .file("caller2/Cargo.toml", r#"
            [project]
            name = "caller2"
            version = "0.1.0"
            authors = []

            [dependencies]
            feat_lib = { path = "../feat_lib", features = ["myfeature"] }
            caller1 = { path = "../caller1" }
        "#)
        .file("caller2/src/main.rs", "fn main() {}")
        .file("caller2/src/lib.rs", "");
    let p = p.build();

    // Build the entire workspace
    assert_that(p.cargo("build").arg("--all"),
                execs().with_status(0)
                       .with_stderr("\
[..]Compiling feat_lib v0.1.0 ([..])
[..]Compiling caller1 v0.1.0 ([..])
[..]Compiling caller2 v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
    assert_that(&p.bin("caller1"), existing_file());
    assert_that(&p.bin("caller2"), existing_file());


    // Build caller1. should build the dep library. Because the features
    // are different than the full workspace, it rebuilds.
    // Ideally once we solve https://github.com/rust-lang/cargo/issues/3620, then
    // a single cargo build at the top level will be enough.
    assert_that(p.cargo("build").cwd(p.root().join("caller1")),
                execs().with_status(0)
                       .with_stderr("\
[..]Compiling feat_lib v0.1.0 ([..])
[..]Compiling caller1 v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));

    // Alternate building caller2/caller1 a few times, just to make sure
    // features are being built separately.  Should not rebuild anything
    assert_that(p.cargo("build").cwd(p.root().join("caller2")),
                execs().with_status(0)
                       .with_stderr("\
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
    assert_that(p.cargo("build").cwd(p.root().join("caller1")),
                execs().with_status(0)
                       .with_stderr("\
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
    assert_that(p.cargo("build").cwd(p.root().join("caller2")),
                execs().with_status(0)
                       .with_stderr("\
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
}

/*FIXME: This fails because of how workspace.exclude and workspace.members are working.
#[test]
fn include_and_exclude() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["foo"]
            exclude = ["foo/bar"]
            "#)
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
            authors = []
            "#)
        .file("foo/src/lib.rs", "")
        .file("foo/bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
            authors = []
            "#)
        .file("foo/bar/src/lib.rs", "");
    p.build();

    assert_that(p.cargo("build").cwd(p.root().join("foo")),
    execs().with_status(0));
    assert_that(&p.root().join("target"), existing_dir());
    assert_that(&p.root().join("foo/target"), is_not(existing_dir()));
    assert_that(p.cargo("build").cwd(p.root().join("foo/bar")),
    execs().with_status(0));
    assert_that(&p.root().join("foo/bar/target"), existing_dir());
}
*/
