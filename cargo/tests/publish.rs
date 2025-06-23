extern crate cargotest;
extern crate flate2;
extern crate hamcrest;
extern crate tar;

use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

use cargotest::support::git::repo;
use cargotest::support::paths;
use cargotest::support::{project, execs, publish};
use flate2::read::GzDecoder;
use hamcrest::assert_that;
use tar::Archive;

#[test]
fn simple() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish").arg("--no-verify")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(0).with_stderr(&format!("\
[UPDATING] registry `{reg}`
[WARNING] manifest has no documentation, [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[UPLOADING] foo v0.0.1 ({dir})
",
        dir = p.url(),
        reg = publish::registry())));

    let mut f = File::open(&publish::upload_path().join("api/v1/crates/new")).unwrap();
    // Skip the metadata payload and the size of the tarball
    let mut sz = [0; 4];
    assert_eq!(f.read(&mut sz).unwrap(), 4);
    let sz = ((sz[0] as u32) <<  0) |
             ((sz[1] as u32) <<  8) |
             ((sz[2] as u32) << 16) |
             ((sz[3] as u32) << 24);
    f.seek(SeekFrom::Current(sz as i64 + 4)).unwrap();

    // Verify the tarball
    let mut rdr = GzDecoder::new(f).unwrap();
    assert_eq!(rdr.header().filename().unwrap(), b"foo-0.0.1.crate");
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    for file in ar.entries().unwrap() {
        let file = file.unwrap();
        let fname = file.header().path_bytes();
        let fname = &*fname;
        assert!(fname == b"foo-0.0.1/Cargo.toml" ||
                fname == b"foo-0.0.1/Cargo.toml.orig" ||
                fname == b"foo-0.0.1/src/main.rs",
                "unexpected filename: {:?}", file.header().path());
    }
}

// TODO: Deprecated
// remove once it has been decided --host can be removed
#[test]
fn simple_with_host() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish").arg("--no-verify")
                 .arg("--host").arg(publish::registry().to_string()),
                execs().with_status(0).with_stderr(&format!("\
[WARNING] The flag '--host' is no longer valid.

Previous versions of Cargo accepted this flag, but it is being
deprecated. The flag is being renamed to 'index', as the flag
wants the location of the index to which to publish. Please
use '--index' instead.

This will soon become a hard error, so it's either recommended
to update to a fixed version or contact the upstream maintainer
about this warning.
[UPDATING] registry `{reg}`
[WARNING] manifest has no documentation, [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[UPLOADING] foo v0.0.1 ({dir})
",
        dir = p.url(),
        reg = publish::registry())));

    let mut f = File::open(&publish::upload_path().join("api/v1/crates/new")).unwrap();
    // Skip the metadata payload and the size of the tarball
    let mut sz = [0; 4];
    assert_eq!(f.read(&mut sz).unwrap(), 4);
    let sz = ((sz[0] as u32) <<  0) |
             ((sz[1] as u32) <<  8) |
             ((sz[2] as u32) << 16) |
             ((sz[3] as u32) << 24);
    f.seek(SeekFrom::Current(sz as i64 + 4)).unwrap();

    // Verify the tarball
    let mut rdr = GzDecoder::new(f).unwrap();
    assert_eq!(rdr.header().filename().unwrap(), "foo-0.0.1.crate".as_bytes());
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    for file in ar.entries().unwrap() {
        let file = file.unwrap();
        let fname = file.header().path_bytes();
        let fname = &*fname;
        assert!(fname == b"foo-0.0.1/Cargo.toml" ||
                fname == b"foo-0.0.1/Cargo.toml.orig" ||
                fname == b"foo-0.0.1/src/main.rs",
                "unexpected filename: {:?}", file.header().path());
    }
}

// TODO: Deprecated
// remove once it has been decided --host can be removed
#[test]
fn simple_with_index_and_host() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish").arg("--no-verify")
                 .arg("--index").arg(publish::registry().to_string())
                 .arg("--host").arg(publish::registry().to_string()),
                execs().with_status(0).with_stderr(&format!("\
[WARNING] The flag '--host' is no longer valid.

Previous versions of Cargo accepted this flag, but it is being
deprecated. The flag is being renamed to 'index', as the flag
wants the location of the index to which to publish. Please
use '--index' instead.

This will soon become a hard error, so it's either recommended
to update to a fixed version or contact the upstream maintainer
about this warning.
[UPDATING] registry `{reg}`
[WARNING] manifest has no documentation, [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[UPLOADING] foo v0.0.1 ({dir})
",
        dir = p.url(),
        reg = publish::registry())));

    let mut f = File::open(&publish::upload_path().join("api/v1/crates/new")).unwrap();
    // Skip the metadata payload and the size of the tarball
    let mut sz = [0; 4];
    assert_eq!(f.read(&mut sz).unwrap(), 4);
    let sz = ((sz[0] as u32) <<  0) |
             ((sz[1] as u32) <<  8) |
             ((sz[2] as u32) << 16) |
             ((sz[3] as u32) << 24);
    f.seek(SeekFrom::Current(sz as i64 + 4)).unwrap();

    // Verify the tarball
    let mut rdr = GzDecoder::new(f).unwrap();
    assert_eq!(rdr.header().filename().unwrap(), "foo-0.0.1.crate".as_bytes());
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    for file in ar.entries().unwrap() {
        let file = file.unwrap();
        let fname = file.header().path_bytes();
        let fname = &*fname;
        assert!(fname == b"foo-0.0.1/Cargo.toml" ||
                fname == b"foo-0.0.1/Cargo.toml.orig" ||
                fname == b"foo-0.0.1/src/main.rs",
                "unexpected filename: {:?}", file.header().path());
    }
}

#[test]
fn git_deps() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"

            [dependencies.foo]
            git = "git://path/to/nowhere"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish").arg("-v").arg("--no-verify")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(101).with_stderr("\
[UPDATING] registry [..]
[ERROR] crates cannot be published to crates.io with dependencies sourced from \
a repository\neither publish `foo` as its own crate on crates.io and \
specify a crates.io version as a dependency or pull it into this \
repository and specify it with a path and version\n\
(crate `foo` has repository path `git://path/to/nowhere`)\
"));
}

#[test]
fn path_dependency_no_version() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"

            [dependencies.bar]
            path = "bar"
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("bar/Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.0.1"
            authors = []
        "#)
        .file("bar/src/lib.rs", "")
        .build();

    assert_that(p.cargo("publish")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(101).with_stderr("\
[UPDATING] registry [..]
[ERROR] all path dependencies must have a version specified when publishing.
dependency `bar` does not specify a version
"));
}

#[test]
fn unpublishable_crate() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            publish = false
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(101).with_stderr("\
[ERROR] some crates cannot be published.
`foo` is marked as unpublishable
"));
}

#[test]
fn dont_publish_dirty() {
    publish::setup();
    let p = project("foo")
        .file("bar", "")
        .build();

    let _ = repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(101).with_stderr("\
[UPDATING] registry `[..]`
error: 1 files in the working directory contain changes that were not yet \
committed into git:

bar

to proceed despite this, pass the `--allow-dirty` flag
"));
}

#[test]
fn publish_clean() {
    publish::setup();

    let p = project("foo").build();

    let _ = repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(0));
}

#[test]
fn publish_in_sub_repo() {
    publish::setup();

    let p = project("foo")
        .file("baz", "")
        .build();

    let _ = repo(&paths::root().join("foo"))
        .file("bar/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("bar/src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish").cwd(p.root().join("bar"))
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(0));
}

#[test]
fn publish_when_ignored() {
    publish::setup();

    let p = project("foo")
        .file("baz", "")
        .build();

    let _ = repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .file(".gitignore", "baz")
        .build();

    assert_that(p.cargo("publish")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(0));
}

#[test]
fn ignore_when_crate_ignored() {
    publish::setup();

    let p = project("foo")
        .file("bar/baz", "")
        .build();

    let _ = repo(&paths::root().join("foo"))
        .file(".gitignore", "bar")
        .nocommit_file("bar/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .nocommit_file("bar/src/main.rs", "fn main() {}");
    assert_that(p.cargo("publish").cwd(p.root().join("bar"))
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(0));
}

#[test]
fn new_crate_rejected() {
    publish::setup();

    let p = project("foo")
        .file("baz", "")
        .build();

    let _ = repo(&paths::root().join("foo"))
        .nocommit_file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .nocommit_file("src/main.rs", "fn main() {}");
    assert_that(p.cargo("publish")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(101));
}

#[test]
fn dry_run() {
    publish::setup();

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(p.cargo("publish").arg("--dry-run")
                 .arg("--index").arg(publish::registry().to_string()),
                execs().with_status(0).with_stderr(&format!("\
[UPDATING] registry `[..]`
[WARNING] manifest has no documentation, [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 [..]
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[UPLOADING] foo v0.0.1 ({dir})
[WARNING] aborting upload due to dry run
",
        dir = p.url())));

    // Ensure the API request wasn't actually made
    assert!(!publish::upload_path().join("api/v1/crates/new").exists());
}
