#[macro_use]
extern crate cargotest;
extern crate flate2;
extern crate git2;
extern crate hamcrest;
extern crate tar;

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use cargotest::{cargo_process, process};
use cargotest::support::{project, execs, paths, git, path2url, cargo_exe};
use cargotest::support::registry::Package;
use flate2::read::GzDecoder;
use hamcrest::{assert_that, existing_file, contains, equal_to};
use tar::Archive;

#[test]
fn simple() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            exclude = ["*.txt"]
            license = "MIT"
            description = "foo"
        "#)
        .file("src/main.rs", r#"
            fn main() { println!("hello"); }
        "#)
        .file("src/bar.txt", "") // should be ignored when packaging
        .build();

    assert_that(p.cargo("package"),
                execs().with_status(0).with_stderr(&format!("\
[WARNING] manifest has no documentation[..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));
    assert_that(&p.root().join("target/package/foo-0.0.1.crate"), existing_file());
    assert_that(p.cargo("package").arg("-l"),
                execs().with_status(0).with_stdout("\
Cargo.toml
src[/]main.rs
"));
    assert_that(p.cargo("package"),
                execs().with_status(0).with_stdout(""));

    let f = File::open(&p.root().join("target/package/foo-0.0.1.crate")).unwrap();
    let mut rdr = GzDecoder::new(f).unwrap();
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    for f in ar.entries().unwrap() {
        let f = f.unwrap();
        let fname = f.header().path_bytes();
        let fname = &*fname;
        assert!(fname == b"foo-0.0.1/Cargo.toml" ||
                fname == b"foo-0.0.1/Cargo.toml.orig" ||
                fname == b"foo-0.0.1/src/main.rs",
                "unexpected filename: {:?}", f.header().path())
    }
}

#[test]
fn metadata_warning() {
    let p = project("all")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .build();
    assert_that(p.cargo("package"),
                execs().with_status(0).with_stderr(&format!("\
warning: manifest has no description, license, license-file, documentation, \
homepage or repository.
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));

    let p = project("one")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .build();
    assert_that(p.cargo("package"),
                execs().with_status(0).with_stderr(&format!("\
warning: manifest has no description, documentation, homepage or repository.
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));

    let p = project("all")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "foo"
            repository = "bar"
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .build();
    assert_that(p.cargo("package"),
                execs().with_status(0).with_stderr(&format!("\
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));
}

#[test]
fn package_verbose() {
    let root = paths::root().join("all");
    let p = git::repo(&root)
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .file("a/Cargo.toml", r#"
            [project]
            name = "a"
            version = "0.0.1"
            authors = []
        "#)
        .file("a/src/lib.rs", "")
        .build();
    let mut cargo = cargo_process();
    cargo.cwd(p.root());
    assert_that(cargo.clone().arg("build"), execs().with_status(0));

    println!("package main repo");
    assert_that(cargo.clone().arg("package").arg("-v").arg("--no-verify"),
                execs().with_status(0).with_stderr("\
[WARNING] manifest has no description[..]
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] foo v0.0.1 ([..])
[ARCHIVING] [..]
[ARCHIVING] [..]
"));

    println!("package sub-repo");
    assert_that(cargo.arg("package").arg("-v").arg("--no-verify")
                     .cwd(p.root().join("a")),
                execs().with_status(0).with_stderr("\
[WARNING] manifest has no description[..]
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] a v0.0.1 ([..])
[ARCHIVING] [..]
[ARCHIVING] [..]
"));
}

#[test]
fn package_verification() {
    let p = project("all")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .build();
    assert_that(p.cargo("build"),
                execs().with_status(0));
    assert_that(p.cargo("package"),
                execs().with_status(0).with_stderr(&format!("\
[WARNING] manifest has no description[..]
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));
}

#[test]
fn path_dependency_no_version() {
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

    assert_that(p.cargo("package"),
                execs().with_status(101).with_stderr("\
[WARNING] manifest has no documentation, homepage or repository.
See http://doc.crates.io/manifest.html#package-metadata for more info.
[ERROR] all path dependencies must have a version specified when packaging.
dependency `bar` does not specify a version.
"));
}

#[test]
fn exclude() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            exclude = [
                "*.txt",
                # file in root
                "file_root_1",       # NO_CHANGE (ignored)
                "/file_root_2",      # CHANGING (packaged -> ignored)
                "file_root_3/",      # NO_CHANGE (packaged)
                "file_root_4/*",     # NO_CHANGE (packaged)
                "file_root_5/**",    # NO_CHANGE (packaged)
                # file in sub-dir
                "file_deep_1",       # CHANGING (packaged -> ignored)
                "/file_deep_2",      # NO_CHANGE (packaged)
                "file_deep_3/",      # NO_CHANGE (packaged)
                "file_deep_4/*",     # NO_CHANGE (packaged)
                "file_deep_5/**",    # NO_CHANGE (packaged)
                # dir in root
                "dir_root_1",        # CHANGING (packaged -> ignored)
                "/dir_root_2",       # CHANGING (packaged -> ignored)
                "dir_root_3/",       # CHANGING (packaged -> ignored)
                "dir_root_4/*",      # NO_CHANGE (ignored)
                "dir_root_5/**",     # NO_CHANGE (ignored)
                # dir in sub-dir
                "dir_deep_1",        # CHANGING (packaged -> ignored)
                "/dir_deep_2",       # NO_CHANGE
                "dir_deep_3/",       # CHANGING (packaged -> ignored)
                "dir_deep_4/*",      # CHANGING (packaged -> ignored)
                "dir_deep_5/**",     # CHANGING (packaged -> ignored)
            ]
        "#)
        .file("src/main.rs", r#"
            fn main() { println!("hello"); }
        "#)
        .file("bar.txt", "")
        .file("src/bar.txt", "")
        // file in root
        .file("file_root_1", "")
        .file("file_root_2", "")
        .file("file_root_3", "")
        .file("file_root_4", "")
        .file("file_root_5", "")
        // file in sub-dir
        .file("some_dir/file_deep_1", "")
        .file("some_dir/file_deep_2", "")
        .file("some_dir/file_deep_3", "")
        .file("some_dir/file_deep_4", "")
        .file("some_dir/file_deep_5", "")
        // dir in root
        .file("dir_root_1/some_dir/file", "")
        .file("dir_root_2/some_dir/file", "")
        .file("dir_root_3/some_dir/file", "")
        .file("dir_root_4/some_dir/file", "")
        .file("dir_root_5/some_dir/file", "")
        // dir in sub-dir
        .file("some_dir/dir_deep_1/some_dir/file", "")
        .file("some_dir/dir_deep_2/some_dir/file", "")
        .file("some_dir/dir_deep_3/some_dir/file", "")
        .file("some_dir/dir_deep_4/some_dir/file", "")
        .file("some_dir/dir_deep_5/some_dir/file", "")
        .build();

    assert_that(p.cargo("package").arg("--no-verify").arg("-v"),
                execs().with_status(0).with_stdout("").with_stderr("\
[WARNING] manifest has no description[..]
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] foo v0.0.1 ([..])
[WARNING] [..] file `dir_root_1[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `dir_root_2[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `dir_root_3[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `some_dir[/]dir_deep_1[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `some_dir[/]dir_deep_3[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `some_dir[/]dir_deep_4[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `some_dir[/]dir_deep_5[/]some_dir[/]file` WILL be excluded [..]
See [..]
[WARNING] [..] file `some_dir[/]file_deep_1` WILL be excluded [..]
See [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
"));

    assert_that(&p.root().join("target/package/foo-0.0.1.crate"), existing_file());

    assert_that(p.cargo("package").arg("-l"),
                execs().with_status(0).with_stdout("\
Cargo.toml
dir_root_1[/]some_dir[/]file
dir_root_2[/]some_dir[/]file
dir_root_3[/]some_dir[/]file
file_root_3
file_root_4
file_root_5
some_dir[/]dir_deep_1[/]some_dir[/]file
some_dir[/]dir_deep_2[/]some_dir[/]file
some_dir[/]dir_deep_3[/]some_dir[/]file
some_dir[/]dir_deep_4[/]some_dir[/]file
some_dir[/]dir_deep_5[/]some_dir[/]file
some_dir[/]file_deep_1
some_dir[/]file_deep_2
some_dir[/]file_deep_3
some_dir[/]file_deep_4
some_dir[/]file_deep_5
src[/]main.rs
"));
}

#[test]
fn include() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            exclude = ["*.txt"]
            include = ["foo.txt", "**/*.rs", "Cargo.toml"]
        "#)
        .file("foo.txt", "")
        .file("src/main.rs", r#"
            fn main() { println!("hello"); }
        "#)
        .file("src/bar.txt", "") // should be ignored when packaging
        .build();

    assert_that(p.cargo("package").arg("--no-verify").arg("-v"),
                execs().with_status(0).with_stderr("\
[WARNING] manifest has no description[..]
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] foo v0.0.1 ([..])
[ARCHIVING] [..]
[ARCHIVING] [..]
[ARCHIVING] [..]
"));
}

#[test]
fn package_lib_with_bin() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            extern crate foo;
            fn main() {}
        "#)
        .file("src/lib.rs", "")
        .build();

    assert_that(p.cargo("package").arg("-v"),
                execs().with_status(0));
}

#[test]
fn package_git_submodule() {
    let project = git::new("foo", |project| {
        project.file("Cargo.toml", r#"
                    [project]
                    name = "foo"
                    version = "0.0.1"
                    authors = ["foo@example.com"]
                    license = "MIT"
                    description = "foo"
                    repository = "foo"
                "#)
                .file("src/lib.rs", "pub fn foo() {}")
    }).unwrap();
    let library = git::new("bar", |library| {
        library.file("Makefile", "all:")
    }).unwrap();

    let repository = git2::Repository::open(&project.root()).unwrap();
    let url = path2url(library.root()).to_string();
    git::add_submodule(&repository, &url, Path::new("bar"));
    git::commit(&repository);

    let repository = git2::Repository::open(&project.root().join("bar")).unwrap();
    repository.reset(&repository.revparse_single("HEAD").unwrap(),
                     git2::ResetType::Hard, None).unwrap();

    assert_that(cargo_process().arg("package").cwd(project.root())
                 .arg("--no-verify").arg("-v"),
                execs().with_status(0).with_stderr_contains("[ARCHIVING] bar/Makefile"));
}

#[test]
fn no_duplicates_from_modified_tracked_files() {
    let root = paths::root().join("all");
    let p = git::repo(&root)
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .build();
    File::create(p.root().join("src/main.rs")).unwrap().write_all(br#"
            fn main() { println!("A change!"); }
        "#).unwrap();
    let mut cargo = cargo_process();
    cargo.cwd(p.root());
    assert_that(cargo.clone().arg("build"), execs().with_status(0));
    assert_that(cargo.arg("package").arg("--list"),
                execs().with_status(0).with_stdout("\
Cargo.toml
src/main.rs
"));
}

#[test]
fn ignore_nested() {
    let cargo_toml = r#"
            [project]
            name = "nested"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = "nested"
        "#;
    let main_rs = r#"
            fn main() { println!("hello"); }
        "#;
    let p = project("nested")
        .file("Cargo.toml", cargo_toml)
        .file("src/main.rs", main_rs)
        // If a project happens to contain a copy of itself, we should
        // ignore it.
        .file("a_dir/nested/Cargo.toml", cargo_toml)
        .file("a_dir/nested/src/main.rs", main_rs)
        .build();

    assert_that(p.cargo("package"),
                execs().with_status(0).with_stderr(&format!("\
[WARNING] manifest has no documentation[..]
See http://doc.crates.io/manifest.html#package-metadata for more info.
[PACKAGING] nested v0.0.1 ({dir})
[VERIFYING] nested v0.0.1 ({dir})
[COMPILING] nested v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));
    assert_that(&p.root().join("target/package/nested-0.0.1.crate"), existing_file());
    assert_that(p.cargo("package").arg("-l"),
                execs().with_status(0).with_stdout("\
Cargo.toml
src[..]main.rs
"));
    assert_that(p.cargo("package"),
                execs().with_status(0).with_stdout(""));

    let f = File::open(&p.root().join("target/package/nested-0.0.1.crate")).unwrap();
    let mut rdr = GzDecoder::new(f).unwrap();
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    for f in ar.entries().unwrap() {
        let f = f.unwrap();
        let fname = f.header().path_bytes();
        let fname = &*fname;
        assert!(fname == b"nested-0.0.1/Cargo.toml" ||
                fname == b"nested-0.0.1/Cargo.toml.orig" ||
                fname == b"nested-0.0.1/src/main.rs",
                "unexpected filename: {:?}", f.header().path())
    }
}

#[cfg(unix)] // windows doesn't allow these characters in filenames
#[test]
fn package_weird_characters() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() { println!("hello"); }
        "#)
        .file("src/:foo", "")
        .build();

    assert_that(p.cargo("package"),
                execs().with_status(101).with_stderr("\
warning: [..]
See [..]
[PACKAGING] foo [..]
[ERROR] failed to prepare local package for uploading

Caused by:
  cannot package a filename with a special character `:`: src/:foo
"));
}

#[test]
fn repackage_on_source_change() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() { println!("hello"); }
        "#)
        .build();

    assert_that(p.cargo("package"),
                execs().with_status(0));

    // Add another source file
    let mut file = File::create(p.root().join("src").join("foo.rs")).unwrap_or_else(|e| {
        panic!("could not create file {}: {}", p.root().join("src/foo.rs").display(), e)
    });

    file.write_all(br#"
        fn main() { println!("foo"); }
    "#).unwrap();
    std::mem::drop(file);

    let mut pro = process(&cargo_exe());
    pro.arg("package").cwd(p.root());

    // Check that cargo rebuilds the tarball
    assert_that(pro, execs().with_status(0).with_stderr(&format!("\
[WARNING] [..]
See [..]
[PACKAGING] foo v0.0.1 ({dir})
[VERIFYING] foo v0.0.1 ({dir})
[COMPILING] foo v0.0.1 ({dir}[..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
",
        dir = p.url())));

    // Check that the tarball contains the added file
    let f = File::open(&p.root().join("target/package/foo-0.0.1.crate")).unwrap();
    let mut rdr = GzDecoder::new(f).unwrap();
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    let entries = ar.entries().unwrap();
    let entry_paths = entries.map(|entry| {
        entry.unwrap().path().unwrap().into_owned()
    }).collect::<Vec<PathBuf>>();
    assert_that(&entry_paths, contains(vec![PathBuf::from("foo-0.0.1/src/foo.rs")]));
}

#[test]
#[cfg(unix)]
fn broken_symlink() {
    use std::os::unix::fs;

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            license = "MIT"
            description = 'foo'
            documentation = 'foo'
            homepage = 'foo'
            repository = 'foo'
        "#)
        .file("src/main.rs", r#"
            fn main() { println!("hello"); }
        "#)
        .build();
    t!(fs::symlink("nowhere", &p.root().join("src/foo.rs")));

    assert_that(p.cargo("package").arg("-v"),
                execs().with_status(101)
                       .with_stderr_contains("\
error: failed to prepare local package for uploading

Caused by:
  failed to open for archiving: `[..]foo.rs`

Caused by:
  [..]
"));
}

#[test]
fn do_not_package_if_repository_is_dirty() {
    let p = project("foo").build();

    // Create a Git repository containing a minimal Rust project.
    let _ = git::repo(&paths::root().join("foo"))
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
        "#)
        .file("src/main.rs", "fn main() {}")
        .build();

    // Modify Cargo.toml without committing the change.
    p.change_file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            license = "MIT"
            description = "foo"
            documentation = "foo"
            homepage = "foo"
            repository = "foo"
            # change
    "#);

    assert_that(p.cargo("package"),
                execs().with_status(101)
                       .with_stderr("\
error: 1 files in the working directory contain changes that were not yet \
committed into git:

Cargo.toml

to proceed despite this, pass the `--allow-dirty` flag
"));
}

#[test]
fn generated_manifest() {
    Package::new("abc", "1.0.0").publish();
    Package::new("def", "1.0.0").publish();
    Package::new("ghi", "1.0.0").publish();
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
            exclude = ["*.txt"]
            license = "MIT"
            description = "foo"

            [project.metadata]
            foo = 'bar'

            [workspace]

            [dependencies]
            bar = { path = "bar", version = "0.1" }
            def = "1.0"
            ghi = "1.0"
            abc = "1.0"
        "#)
        .file("src/main.rs", "")
        .file("bar/Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("bar/src/lib.rs", "")
        .build();

    assert_that(p.cargo("package").arg("--no-verify"),
                execs().with_status(0));

    let f = File::open(&p.root().join("target/package/foo-0.0.1.crate")).unwrap();
    let mut rdr = GzDecoder::new(f).unwrap();
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    let mut entry = ar.entries().unwrap()
                        .map(|f| f.unwrap())
                        .find(|e| e.path().unwrap().ends_with("Cargo.toml"))
                        .unwrap();
    let mut contents = String::new();
    entry.read_to_string(&mut contents).unwrap();
    // BTreeMap makes the order of dependencies in the generated file deterministic
    // by sorting alphabetically
    assert_that(&contents[..], equal_to(
r#"# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO
#
# When uploading crates to the registry Cargo will automatically
# "normalize" Cargo.toml files for maximal compatibility
# with all versions of Cargo and also rewrite `path` dependencies
# to registry (e.g. crates.io) dependencies
#
# If you believe there's an error in this file please file an
# issue against the rust-lang/cargo repository. If you're
# editing this file be aware that the upstream Cargo.toml
# will likely look very different (and much more reasonable)

[package]
name = "foo"
version = "0.0.1"
authors = []
exclude = ["*.txt"]
description = "foo"
license = "MIT"

[package.metadata]
foo = "bar"
[dependencies.abc]
version = "1.0"

[dependencies.bar]
version = "0.1"

[dependencies.def]
version = "1.0"

[dependencies.ghi]
version = "1.0"
"#));
}

#[test]
fn ignore_workspace_specifier() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"

            authors = []

            [workspace]

            [dependencies]
            bar = { path = "bar", version = "0.1" }
        "#)
        .file("src/main.rs", "")
        .file("bar/Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
            workspace = ".."
        "#)
        .file("bar/src/lib.rs", "")
        .build();

    assert_that(p.cargo("package").arg("--no-verify").cwd(p.root().join("bar")),
                execs().with_status(0));

    let f = File::open(&p.root().join("target/package/bar-0.1.0.crate")).unwrap();
    let mut rdr = GzDecoder::new(f).unwrap();
    let mut contents = Vec::new();
    rdr.read_to_end(&mut contents).unwrap();
    let mut ar = Archive::new(&contents[..]);
    let mut entry = ar.entries().unwrap()
                        .map(|f| f.unwrap())
                        .find(|e| e.path().unwrap().ends_with("Cargo.toml"))
                        .unwrap();
    let mut contents = String::new();
    entry.read_to_string(&mut contents).unwrap();
    assert_that(&contents[..], equal_to(
r#"# THIS FILE IS AUTOMATICALLY GENERATED BY CARGO
#
# When uploading crates to the registry Cargo will automatically
# "normalize" Cargo.toml files for maximal compatibility
# with all versions of Cargo and also rewrite `path` dependencies
# to registry (e.g. crates.io) dependencies
#
# If you believe there's an error in this file please file an
# issue against the rust-lang/cargo repository. If you're
# editing this file be aware that the upstream Cargo.toml
# will likely look very different (and much more reasonable)

[package]
name = "bar"
version = "0.1.0"
authors = []
"#));
}

#[test]
fn package_two_kinds_of_deps() {
    Package::new("other", "1.0.0").publish();
    Package::new("other1", "1.0.0").publish();
    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies]
            other = "1.0"
            other1 = { version = "1.0" }
        "#)
        .file("src/main.rs", "")
        .build();

    assert_that(p.cargo("package").arg("--no-verify"),
                execs().with_status(0));
}
