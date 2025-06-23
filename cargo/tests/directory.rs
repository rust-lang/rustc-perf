#[macro_use]
extern crate cargotest;
extern crate hamcrest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::str;

use cargotest::cargo_process;
use cargotest::support::git;
use cargotest::support::paths;
use cargotest::support::registry::{Package, cksum};
use cargotest::support::{project, execs, ProjectBuilder};
use hamcrest::assert_that;

fn setup() {
    let root = paths::root();
    t!(fs::create_dir(&root.join(".cargo")));
    t!(t!(File::create(root.join(".cargo/config"))).write_all(br#"
        [source.crates-io]
        replace-with = 'my-awesome-local-registry'

        [source.my-awesome-local-registry]
        directory = 'index'
    "#));
}

struct VendorPackage {
    p: Option<ProjectBuilder>,
    cksum: Checksum,
}

#[derive(Serialize)]
struct Checksum {
    package: Option<String>,
    files: HashMap<String, String>,
}

impl VendorPackage {
    fn new(name: &str) -> VendorPackage {
        VendorPackage {
            p: Some(project(&format!("index/{}", name))),
            cksum: Checksum {
                package: Some(String::new()),
                files: HashMap::new(),
            },
        }
    }

    fn file(&mut self, name: &str, contents: &str) -> &mut VendorPackage {
        self.p = Some(self.p.take().unwrap().file(name, contents));
        self.cksum.files.insert(name.to_string(), cksum(contents.as_bytes()));
        self
    }

    fn disable_checksum(&mut self) -> &mut VendorPackage {
        self.cksum.package = None;
        self
    }

    fn build(&mut self) {
        let p = self.p.take().unwrap();
        let json = serde_json::to_string(&self.cksum).unwrap();
        let p = p.file(".cargo-checksum.json", &json);
        let _ = p.build();
    }
}

#[test]
fn simple() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#)
        .build();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));
}

#[test]
fn simple_install() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/main.rs", r#"
            extern crate foo;

            pub fn main() {
                foo::foo();
            }
        "#)
        .build();

    assert_that(cargo_process().arg("install").arg("bar"),
                execs().with_status(0).with_stderr(
"  Installing bar v0.1.0
   Compiling foo v0.1.0
   Compiling bar v0.1.0
    Finished release [optimized] target(s) in [..] secs
  Installing [..]bar[..]
warning: be sure to add `[..]` to your PATH to be able to run the installed binaries
"));
}

#[test]
fn simple_install_fail() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
            baz = "9.8.7"
        "#)
        .file("src/main.rs", r#"
            extern crate foo;

            pub fn main() {
                foo::foo();
            }
        "#)
        .build();

    assert_that(cargo_process().arg("install").arg("bar"),
                execs().with_status(101).with_stderr(
"  Installing bar v0.1.0
error: failed to compile `bar v0.1.0`, intermediate artifacts can be found at `[..]`

Caused by:
  no matching package named `baz` found (required by `bar`)
location searched: registry https://github.com/rust-lang/crates.io-index
version required: ^9.8.7
"));
}

#[test]
fn install_without_feature_dep() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .build();

    VendorPackage::new("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
            baz = { version = "9.8.7", optional = true }

            [features]
            wantbaz = ["baz"]
        "#)
        .file("src/main.rs", r#"
            extern crate foo;

            pub fn main() {
                foo::foo();
            }
        "#)
        .build();

    assert_that(cargo_process().arg("install").arg("bar"),
                execs().with_status(0).with_stderr(
"  Installing bar v0.1.0
   Compiling foo v0.1.0
   Compiling bar v0.1.0
    Finished release [optimized] target(s) in [..] secs
  Installing [..]bar[..]
warning: be sure to add `[..]` to your PATH to be able to run the installed binaries
"));
}

#[test]
fn not_there() {
    setup();

    let _ = project("index").build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#)
        .build();

    assert_that(p.cargo("build"),
                execs().with_status(101).with_stderr("\
error: no matching package named `foo` found (required by `bar`)
location searched: [..]
version required: ^0.1.0
"));
}

#[test]
fn multiple() {
    setup();

    VendorPackage::new("foo-0.1.0")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .file(".cargo-checksum", "")
        .build();

    VendorPackage::new("foo-0.2.0")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.2.0"
            authors = []
        "#)
        .file("src/lib.rs", "pub fn foo() {}")
        .file(".cargo-checksum", "")
        .build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#)
        .build();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));
}

#[test]
fn crates_io_then_directory() {
    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", r#"
            extern crate foo;

            pub fn bar() {
                foo::foo();
            }
        "#)
        .build();

    let cksum = Package::new("foo", "0.1.0")
                        .file("src/lib.rs", "pub fn foo() -> u32 { 0 }")
                        .publish();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[UPDATING] registry `[..]`
[DOWNLOADING] foo v0.1.0 ([..])
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));

    setup();

    let mut v = VendorPackage::new("foo");
    v.file("Cargo.toml", r#"
        [package]
        name = "foo"
        version = "0.1.0"
        authors = []
    "#);
    v.file("src/lib.rs", "pub fn foo() -> u32 { 1 }");
    v.cksum.package = Some(cksum);
    v.build();

    assert_that(p.cargo("build"),
                execs().with_status(0).with_stderr("\
[COMPILING] foo v0.1.0
[COMPILING] bar v0.1.0 ([..]bar)
[FINISHED] [..]
"));
}

#[test]
fn crates_io_then_bad_checksum() {
    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", "")
        .build();

    Package::new("foo", "0.1.0").publish();

    assert_that(p.cargo("build"),
                execs().with_status(0));
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .build();

    assert_that(p.cargo("build"),
                execs().with_status(101).with_stderr("\
error: checksum for `foo v0.1.0` changed between lock files

this could be indicative of a few possible errors:

    * the lock file is corrupt
    * a replacement source in use (e.g. a mirror) returned a different checksum
    * the source itself may be corrupt in one way or another

unable to verify that `foo v0.1.0` is the same as when the lockfile was generated

"));
}

#[test]
fn bad_file_checksum() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .build();

    let mut f = t!(File::create(paths::root().join("index/foo/src/lib.rs")));
    t!(f.write_all(b"fn foo() -> u32 { 0 }"));

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", "")
        .build();

    assert_that(p.cargo("build"),
                execs().with_status(101).with_stderr("\
error: the listed checksum of `[..]lib.rs` has changed:
expected: [..]
actual:   [..]

directory sources are not intended to be edited, if modifications are \
required then it is recommended that [replace] is used with a forked copy of \
the source
"));
}

#[test]
fn only_dot_files_ok() {
    setup();

    VendorPackage::new("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .build();
    VendorPackage::new("bar")
        .file(".foo", "")
        .build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []

            [dependencies]
            foo = "0.1.0"
        "#)
        .file("src/lib.rs", "")
        .build();

    assert_that(p.cargo("build"), execs().with_status(0));
}

#[test]
fn git_lock_file_doesnt_change() {

    let git = git::new("git", |p| {
        p.file("Cargo.toml", r#"
            [project]
            name = "git"
            version = "0.5.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
    }).unwrap();

    VendorPackage::new("git")
        .file("Cargo.toml", r#"
            [package]
            name = "git"
            version = "0.5.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .disable_checksum()
        .build();

    let p = project("bar")
        .file("Cargo.toml", &format!(r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies]
            git = {{ git = '{0}' }}
        "#, git.url()))
        .file("src/lib.rs", "")
        .build();

    assert_that(p.cargo("build"), execs().with_status(0));

    let mut lock1 = String::new();
    t!(t!(File::open(p.root().join("Cargo.lock"))).read_to_string(&mut lock1));

    let root = paths::root();
    t!(fs::create_dir(&root.join(".cargo")));
    t!(t!(File::create(root.join(".cargo/config"))).write_all(&format!(r#"
        [source.my-git-repo]
        git = '{}'
        replace-with = 'my-awesome-local-registry'

        [source.my-awesome-local-registry]
        directory = 'index'
    "#, git.url()).as_bytes()));

    assert_that(p.cargo("build"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] [..]
[COMPILING] [..]
[FINISHED] [..]
"));

    let mut lock2 = String::new();
    t!(t!(File::open(p.root().join("Cargo.lock"))).read_to_string(&mut lock2));
    assert!(lock1 == lock2, "lock files changed");
}

#[test]
fn git_override_requires_lockfile() {
    VendorPackage::new("git")
        .file("Cargo.toml", r#"
            [package]
            name = "git"
            version = "0.5.0"
            authors = []
        "#)
        .file("src/lib.rs", "")
        .disable_checksum()
        .build();

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies]
            git = { git = 'https://example.com/' }
        "#)
        .file("src/lib.rs", "")
        .build();

    let root = paths::root();
    t!(fs::create_dir(&root.join(".cargo")));
    t!(t!(File::create(root.join(".cargo/config"))).write_all(br#"
        [source.my-git-repo]
        git = 'https://example.com/'
        replace-with = 'my-awesome-local-registry'

        [source.my-awesome-local-registry]
        directory = 'index'
    "#));

    assert_that(p.cargo("build"),
                execs().with_status(101)
                       .with_stderr("\
error: failed to load source for a dependency on `git`

Caused by:
  Unable to update [..]

Caused by:
  the source my-git-repo requires a lock file to be present first before it can be
used against vendored source code

remove the source replacement configuration, generate a lock file, and then
restore the source replacement configuration to continue the build

"));
}
