extern crate cargotest;
extern crate hamcrest;

use cargotest::is_nightly;
use cargotest::support::{execs, project};
use cargotest::support::registry::Package;
use hamcrest::assert_that;

#[test]
fn check_success() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
            extern crate bar;
            fn main() {
                ::bar::baz();
            }
        "#)
        .build();
    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            pub fn baz() {}
        "#)
        .build();

    assert_that(foo.cargo("check"),
                execs().with_status(0));
}

#[test]
fn check_fail() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
            extern crate bar;
            fn main() {
                ::bar::baz(42);
            }
        "#)
        .build();
    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            pub fn baz() {}
        "#)
        .build();

    assert_that(foo.cargo("check"),
                execs().with_status(101));
}

#[test]
fn custom_derive() {
    if !is_nightly() {
        return
    }
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
#![feature(proc_macro)]

#[macro_use]
extern crate bar;

trait B {
    fn b(&self);
}

#[derive(B)]
struct A;

fn main() {
    let a = A;
    a.b();
}
"#)
        .build();
    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
            [lib]
            proc-macro = true
        "#)
        .file("src/lib.rs", r#"
#![feature(proc_macro, proc_macro_lib)]
#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(B)]
pub fn derive(_input: TokenStream) -> TokenStream {
    format!("impl B for A {{ fn b(&self) {{}} }}").parse().unwrap()
}
"#)
        .build();

    assert_that(foo.cargo("check"),
                execs().with_status(0));
}

#[test]
fn check_build() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
            extern crate bar;
            fn main() {
                ::bar::baz();
            }
        "#)
        .build();

    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            pub fn baz() {}
        "#)
        .build();

    assert_that(foo.cargo("check"),
                execs().with_status(0));
    assert_that(foo.cargo("build"),
                execs().with_status(0));
}

#[test]
fn build_check() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
            extern crate bar;
            fn main() {
                ::bar::baz();
            }
        "#)
        .build();

    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            pub fn baz() {}
        "#)
        .build();

    assert_that(foo.cargo("build"),
                execs().with_status(0));
    assert_that(foo.cargo("check"),
                execs().with_status(0));
}

// Checks that where a project has both a lib and a bin, the lib is only checked
// not built.
#[test]
fn issue_3418() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.1.0"
            authors = []

            [dependencies]
        "#)
        .file("src/lib.rs", "")
        .file("src/main.rs", "fn main() {}")
        .build();

    assert_that(foo.cargo("check").arg("-v"),
                execs().with_status(0)
                       .with_stderr_contains("[..] --emit=dep-info,metadata [..]"));
}

// Some weirdness that seems to be caused by a crate being built as well as
// checked, but in this case with a proc macro too.
#[test]
fn issue_3419() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies]
            rustc-serialize = "*"
        "#)
        .file("src/lib.rs", r#"
            extern crate rustc_serialize;

            use rustc_serialize::Decodable;

            pub fn take<T: Decodable>() {}
        "#)
        .file("src/main.rs", r#"
            extern crate rustc_serialize;

            extern crate foo;

            #[derive(RustcDecodable)]
            pub struct Foo;

            fn main() {
                foo::take::<Foo>();
            }
        "#)
        .build();

    Package::new("rustc-serialize", "1.0.0")
        .file("src/lib.rs",
              r#"pub trait Decodable: Sized {
                    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
                 }
                 pub trait Decoder {
                    type Error;
                    fn read_struct<T, F>(&mut self, s_name: &str, len: usize, f: F)
                                         -> Result<T, Self::Error>
                    where F: FnOnce(&mut Self) -> Result<T, Self::Error>;
                 } "#).publish();

    assert_that(p.cargo("check"),
                execs().with_status(0));
}

// test `cargo rustc --profile check`
#[test]
fn rustc_check() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
            extern crate bar;
            fn main() {
                ::bar::baz();
            }
        "#)
        .build();
    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            pub fn baz() {}
        "#)
        .build();

    assert_that(foo.cargo("rustc")
                   .arg("--profile")
                   .arg("check")
                   .arg("--")
                   .arg("--emit=metadata"),
                execs().with_status(0));
}

#[test]
fn rustc_check_err() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [dependencies.bar]
            path = "../bar"
        "#)
        .file("src/main.rs", r#"
            extern crate bar;
            fn main() {
                ::bar::qux();
            }
        "#)
        .build();
    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.1.0"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            pub fn baz() {}
        "#)
        .build();

    assert_that(foo.cargo("rustc")
                   .arg("--profile")
                   .arg("check")
                   .arg("--")
                   .arg("--emit=metadata"),
                execs().with_status(101));
}

#[test]
fn check_all() {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [workspace]
            [dependencies]
            b = { path = "b" }
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("examples/a.rs", "fn main() {}")
        .file("tests/a.rs", "")
        .file("src/lib.rs", "")
        .file("b/Cargo.toml", r#"
            [package]
            name = "b"
            version = "0.0.1"
            authors = []
        "#)
        .file("b/src/main.rs", "fn main() {}")
        .file("b/src/lib.rs", "")
        .build();

    assert_that(p.cargo("check").arg("--all").arg("-v"),
                execs().with_status(0)
        .with_stderr_contains("[..] --crate-name foo src[/]lib.rs [..]")
        .with_stderr_contains("[..] --crate-name foo src[/]main.rs [..]")
        .with_stderr_contains("[..] --crate-name b b[/]src[/]lib.rs [..]")
        .with_stderr_contains("[..] --crate-name b b[/]src[/]main.rs [..]")
        );
}

#[test]
fn check_virtual_all_implied() {
    let p = project("workspace")
        .file("Cargo.toml", r#"
            [workspace]
            members = ["foo", "bar"]
        "#)
        .file("foo/Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"
        "#)
        .file("foo/src/lib.rs", r#"
            pub fn foo() {}
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
        "#)
        .file("bar/src/lib.rs", r#"
            pub fn bar() {}
        "#)
        .build();

    assert_that(p.cargo("check").arg("-v"),
                execs().with_status(0)
        .with_stderr_contains("[..] --crate-name foo foo[/]src[/]lib.rs [..]")
        .with_stderr_contains("[..] --crate-name bar bar[/]src[/]lib.rs [..]")
        );
}

#[test]
fn check_all_targets() {
    let foo = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", "fn main() {}")
        .file("src/lib.rs", "pub fn smth() {}")
        .file("examples/example1.rs", "fn main() {}")
        .file("tests/test2.rs", "#[test] fn t() {}")
        .file("benches/bench3.rs", "")
        .build();

    assert_that(foo.cargo("check").arg("--all-targets").arg("-v"),
                execs().with_status(0)
        .with_stderr_contains("[..] --crate-name foo src[/]lib.rs [..]")
        .with_stderr_contains("[..] --crate-name foo src[/]main.rs [..]")
        .with_stderr_contains("[..] --crate-name example1 examples[/]example1.rs [..]")
        .with_stderr_contains("[..] --crate-name test2 tests[/]test2.rs [..]")
        .with_stderr_contains("[..] --crate-name bench3 benches[/]bench3.rs [..]")
        );
}
