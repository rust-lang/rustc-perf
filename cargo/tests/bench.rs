extern crate cargotest;
extern crate cargo;
extern crate hamcrest;

use std::str;

use cargo::util::process;
use cargotest::is_nightly;
use cargotest::support::paths::CargoPathExt;
use cargotest::support::{project, execs, basic_bin_manifest, basic_lib_manifest};
use hamcrest::{assert_that, existing_file};

#[test]
fn cargo_bench_simple() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            #![feature(test)]
            #[cfg(test)]
            extern crate test;

            fn hello() -> &'static str {
                "hello"
            }

            pub fn main() {
                println!("{}", hello())
            }

            #[bench]
            fn bench_hello(_b: &mut test::Bencher) {
                assert_eq!(hello(), "hello")
            }"#)
        .build();

    assert_that(p.cargo("build"), execs());
    assert_that(&p.bin("foo"), existing_file());

    assert_that(process(&p.bin("foo")),
                execs().with_stdout("hello\n"));

    assert_that(p.cargo("bench"),
                execs().with_stderr(&format!("\
[COMPILING] foo v0.5.0 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", p.url()))
                       .with_stdout_contains("test bench_hello ... bench: [..]"));
}

#[test]
fn bench_bench_implicit() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml" , r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            fn main() { println!("Hello main!"); }"#)
        .file("tests/other.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run3(_ben: &mut test::Bencher) { }"#)
        .file("benches/mybench.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run2(_ben: &mut test::Bencher) { }"#)
        .build();

    assert_that(p.cargo("bench").arg("--benches"),
        execs().with_status(0)
               .with_stderr(format!("\
[COMPILING] foo v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]mybench-[..][EXE]
", dir = p.url()))
               .with_stdout_contains("test run2 ... bench: [..]"));
}

#[test]
fn bench_bin_implicit() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml" , r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/main.rs", r#"
            #![feature(test)]
            #[cfg(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }
            fn main() { println!("Hello main!"); }"#)
        .file("tests/other.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run3(_ben: &mut test::Bencher) { }"#)
        .file("benches/mybench.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run2(_ben: &mut test::Bencher) { }"#)
        .build();

    assert_that(p.cargo("bench").arg("--bins"),
        execs().with_status(0)
               .with_stderr(format!("\
[COMPILING] foo v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]
", dir = p.url()))
               .with_stdout_contains("test run1 ... bench: [..]"));
}

#[test]
fn bench_tarname() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml" , r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("benches/bin1.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }"#)
        .file("benches/bin2.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run2(_ben: &mut test::Bencher) { }"#)
        .build();

    assert_that(p.cargo("bench").arg("--bench").arg("bin2"),
        execs().with_status(0)
               .with_stderr(format!("\
[COMPILING] foo v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]bin2-[..][EXE]
", dir = p.url()))
               .with_stdout_contains("test run2 ... bench: [..]"));
}

#[test]
fn bench_multiple_targets() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml" , r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("benches/bin1.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run1(_ben: &mut test::Bencher) { }"#)
        .file("benches/bin2.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run2(_ben: &mut test::Bencher) { }"#)
        .file("benches/bin3.rs", r#"
            #![feature(test)]
            extern crate test;
            #[bench] fn run3(_ben: &mut test::Bencher) { }"#)
        .build();

    assert_that(p.cargo("bench")
                .arg("--bench").arg("bin1")
                .arg("--bench").arg("bin2"),
                execs()
                .with_status(0)
                .with_stdout_contains("test run1 ... bench: [..]")
                .with_stdout_contains("test run2 ... bench: [..]")
                .with_stdout_does_not_contain("run3"));
}

#[test]
fn cargo_bench_verbose() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            #![feature(test)]
            #[cfg(test)]
            extern crate test;
            fn main() {}
            #[bench] fn bench_hello(_b: &mut test::Bencher) {}
        "#)
        .build();

    assert_that(p.cargo("bench").arg("-v").arg("hello"),
                execs().with_stderr(&format!("\
[COMPILING] foo v0.5.0 ({url})
[RUNNING] `rustc [..] src[/]main.rs [..]`
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] `[..]target[/]release[/]deps[/]foo-[..][EXE] hello --bench`", url = p.url()))
                       .with_stdout_contains("test bench_hello ... bench: [..]"));
}

#[test]
fn many_similar_names() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/lib.rs", "
            #![feature(test)]
            #[cfg(test)]
            extern crate test;
            pub fn foo() {}
            #[bench] fn lib_bench(_b: &mut test::Bencher) {}
        ")
        .file("src/main.rs", "
            #![feature(test)]
            #[cfg(test)]
            extern crate foo;
            #[cfg(test)]
            extern crate test;
            fn main() {}
            #[bench] fn bin_bench(_b: &mut test::Bencher) { foo::foo() }
        ")
        .file("benches/foo.rs", r#"
            #![feature(test)]
            extern crate foo;
            extern crate test;
            #[bench] fn bench_bench(_b: &mut test::Bencher) { foo::foo() }
        "#)
        .build();

    let output = p.cargo("bench").exec_with_output().unwrap();
    let output = str::from_utf8(&output.stdout).unwrap();
    assert!(output.contains("test bin_bench"), "bin_bench missing\n{}", output);
    assert!(output.contains("test lib_bench"), "lib_bench missing\n{}", output);
    assert!(output.contains("test bench_bench"), "bench_bench missing\n{}", output);
}

#[test]
fn cargo_bench_failing_test() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            #![feature(test)]
            #[cfg(test)]
            extern crate test;
            fn hello() -> &'static str {
                "hello"
            }

            pub fn main() {
                println!("{}", hello())
            }

            #[bench]
            fn bench_hello(_b: &mut test::Bencher) {
                assert_eq!(hello(), "nope")
            }"#)
        .build();

    assert_that(p.cargo("build"), execs());
    assert_that(&p.bin("foo"), existing_file());

    assert_that(process(&p.bin("foo")),
                execs().with_stdout("hello\n"));

    assert_that(p.cargo("bench"),
                execs().with_stdout_contains("test bench_hello ... ")
                       .with_stderr_contains(format!("\
[COMPILING] foo v0.5.0 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]
thread '[..]' panicked at 'assertion failed: \
    `(left == right)`[..]", p.url()))
                       .with_stderr_contains("[..]left: `\"hello\"`[..]")
                       .with_stderr_contains("[..]right: `\"nope\"`[..]")
                       .with_stderr_contains("[..]src[/]main.rs:15[..]")
                       .with_status(101));
}

#[test]
fn bench_with_lib_dep() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [[bin]]
            name = "baz"
            path = "src/main.rs"
        "#)
        .file("src/lib.rs", r#"
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;
            ///
            /// ```rust
            /// extern crate foo;
            /// fn main() {
            ///     println!("{}", foo::foo());
            /// }
            /// ```
            ///
            pub fn foo(){}
            #[bench] fn lib_bench(_b: &mut test::Bencher) {}
        "#)
        .file("src/main.rs", "
            #![feature(test)]
            #[allow(unused_extern_crates)]
            extern crate foo;
            #[cfg(test)]
            extern crate test;

            fn main() {}

            #[bench]
            fn bin_bench(_b: &mut test::Bencher) {}
        ")
        .build();

    assert_that(p.cargo("bench"),
                execs().with_stderr(&format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]
[RUNNING] target[/]release[/]deps[/]baz-[..][EXE]", p.url()))
                       .with_stdout_contains("test lib_bench ... bench: [..]")
                       .with_stdout_contains("test bin_bench ... bench: [..]"));
}

#[test]
fn bench_with_deep_lib_dep() {
    if !is_nightly() { return }

    let p = project("bar")
        .file("Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.0.1"
            authors = []

            [dependencies.foo]
            path = "../foo"
        "#)
        .file("src/lib.rs", "
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate foo;
            #[cfg(test)]
            extern crate test;
            #[bench]
            fn bar_bench(_b: &mut test::Bencher) {
                foo::foo();
            }
        ")
        .build();
    let _p2 = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/lib.rs", "
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;

            pub fn foo() {}

            #[bench]
            fn foo_bench(_b: &mut test::Bencher) {}
        ")
        .build();

    assert_that(p.cargo("bench"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[COMPILING] foo v0.0.1 ([..])
[COMPILING] bar v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]bar-[..][EXE]", dir = p.url()))
                       .with_stdout_contains("test bar_bench ... bench: [..]"));
}

#[test]
fn external_bench_explicit() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [[bench]]
            name = "bench"
            path = "src/bench.rs"
        "#)
        .file("src/lib.rs", r#"
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;
            pub fn get_hello() -> &'static str { "Hello" }

            #[bench]
            fn internal_bench(_b: &mut test::Bencher) {}
        "#)
        .file("src/bench.rs", r#"
            #![feature(test)]
            #[allow(unused_extern_crates)]
            extern crate foo;
            extern crate test;

            #[bench]
            fn external_bench(_b: &mut test::Bencher) {}
        "#)
        .build();

    assert_that(p.cargo("bench"),
                execs().with_stderr(&format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]
[RUNNING] target[/]release[/]deps[/]bench-[..][EXE]", p.url()))
                       .with_stdout_contains("test internal_bench ... bench: [..]")
                       .with_stdout_contains("test external_bench ... bench: [..]"));
}

#[test]
fn external_bench_implicit() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/lib.rs", r#"
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;

            pub fn get_hello() -> &'static str { "Hello" }

            #[bench]
            fn internal_bench(_b: &mut test::Bencher) {}
        "#)
        .file("benches/external.rs", r#"
            #![feature(test)]
            #[allow(unused_extern_crates)]
            extern crate foo;
            extern crate test;

            #[bench]
            fn external_bench(_b: &mut test::Bencher) {}
        "#)
        .build();

    assert_that(p.cargo("bench"),
                execs().with_stderr(&format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]
[RUNNING] target[/]release[/]deps[/]external-[..][EXE]", p.url()))
                       .with_stdout_contains("test internal_bench ... bench: [..]")
                       .with_stdout_contains("test external_bench ... bench: [..]"));
}

#[test]
fn dont_run_examples() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/lib.rs", r"")
        .file("examples/dont-run-me-i-will-fail.rs", r#"
            fn main() { panic!("Examples should not be run by 'cargo test'"); }
        "#)
        .build();
    assert_that(p.cargo("bench"),
                execs().with_status(0));
}

#[test]
fn pass_through_command_line() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/lib.rs", "
            #![feature(test)]
            #[cfg(test)]
            extern crate test;

            #[bench] fn foo(_b: &mut test::Bencher) {}
            #[bench] fn bar(_b: &mut test::Bencher) {}
        ")
        .build();

    assert_that(p.cargo("bench").arg("bar"),
                execs().with_status(0)
                .with_stderr(&format!("\
[COMPILING] foo v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", dir = p.url()))
                .with_stdout_contains("test bar ... bench: [..]"));

    assert_that(p.cargo("bench").arg("foo"),
                execs().with_status(0)
                       .with_stderr("[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                       .with_stdout_contains("test foo ... bench: [..]"));
}

// Regression test for running cargo-bench twice with
// tests in an rlib
#[test]
fn cargo_bench_twice() {
    if !is_nightly() { return }

    let p = project("test_twice")
        .file("Cargo.toml", &basic_lib_manifest("test_twice"))
        .file("src/test_twice.rs", r#"
            #![crate_type = "rlib"]
            #![feature(test)]
            #[cfg(test)]
            extern crate test;

            #[bench]
            fn dummy_bench(b: &mut test::Bencher) { }
            "#)
        .build();

    p.cargo("build");

    for _ in 0..2 {
        assert_that(p.cargo("bench"),
                    execs().with_status(0));
    }
}

#[test]
fn lib_bin_same_name() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.0.1"
            authors = []

            [lib]
            name = "foo"
            [[bin]]
            name = "foo"
        "#)
        .file("src/lib.rs", "
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;
            #[bench] fn lib_bench(_b: &mut test::Bencher) {}
        ")
        .file("src/main.rs", "
            #![cfg_attr(test, feature(test))]
            #[allow(unused_extern_crates)]
            extern crate foo;
            #[cfg(test)]
            extern crate test;

            #[bench]
            fn bin_bench(_b: &mut test::Bencher) {}
        ")
        .build();

    assert_that(p.cargo("bench"),
                execs().with_stderr(&format!("\
[COMPILING] foo v0.0.1 ({})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", p.url()))
                       .with_stdout_contains_n("test [..] ... bench: [..]", 2));
}

#[test]
fn lib_with_standard_name() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "syntax"
            version = "0.0.1"
            authors = []
        "#)
        .file("src/lib.rs", "
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;

            /// ```
            /// syntax::foo();
            /// ```
            pub fn foo() {}

            #[bench]
            fn foo_bench(_b: &mut test::Bencher) {}
        ")
        .file("benches/bench.rs", "
            #![feature(test)]
            extern crate syntax;
            extern crate test;

            #[bench]
            fn bench(_b: &mut test::Bencher) { syntax::foo() }
        ")
        .build();

    assert_that(p.cargo("bench"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[COMPILING] syntax v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]syntax-[..][EXE]
[RUNNING] target[/]release[/]deps[/]bench-[..][EXE]", dir = p.url()))
                       .with_stdout_contains("test foo_bench ... bench: [..]")
                       .with_stdout_contains("test bench ... bench: [..]"));
}

#[test]
fn lib_with_standard_name2() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "syntax"
            version = "0.0.1"
            authors = []

            [lib]
            name = "syntax"
            bench = false
            doctest = false
        "#)
        .file("src/lib.rs", "
            pub fn foo() {}
        ")
        .file("src/main.rs", "
            #![feature(test)]
            #[cfg(test)]
            extern crate syntax;
            #[cfg(test)]
            extern crate test;

            fn main() {}

            #[bench]
            fn bench(_b: &mut test::Bencher) { syntax::foo() }
        ")
        .build();

    assert_that(p.cargo("bench"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[COMPILING] syntax v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]syntax-[..][EXE]", dir = p.url()))
                       .with_stdout_contains("test bench ... bench: [..]"));
}

#[test]
fn bench_dylib() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []

            [lib]
            name = "foo"
            crate_type = ["dylib"]

            [dependencies.bar]
            path = "bar"
        "#)
        .file("src/lib.rs", r#"
            #![cfg_attr(test, feature(test))]
            extern crate bar as the_bar;
            #[cfg(test)]
            extern crate test;

            pub fn bar() { the_bar::baz(); }

            #[bench]
            fn foo(_b: &mut test::Bencher) {}
        "#)
        .file("benches/bench.rs", r#"
            #![feature(test)]
            extern crate foo as the_foo;
            extern crate test;

            #[bench]
            fn foo(_b: &mut test::Bencher) { the_foo::bar(); }
        "#)
        .file("bar/Cargo.toml", r#"
            [package]
            name = "bar"
            version = "0.0.1"
            authors = []

            [lib]
            name = "bar"
            crate_type = ["dylib"]
        "#)
        .file("bar/src/lib.rs", "
             pub fn baz() {}
        ")
        .build();

    assert_that(p.cargo("bench").arg("-v"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[COMPILING] bar v0.0.1 ({dir}/bar)
[RUNNING] [..] -C opt-level=3 [..]
[COMPILING] foo v0.0.1 ({dir})
[RUNNING] [..] -C opt-level=3 [..]
[RUNNING] [..] -C opt-level=3 [..]
[RUNNING] [..] -C opt-level=3 [..]
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] `[..]target[/]release[/]deps[/]foo-[..][EXE] --bench`
[RUNNING] `[..]target[/]release[/]deps[/]bench-[..][EXE] --bench`", dir = p.url()))
                       .with_stdout_contains_n("test foo ... bench: [..]", 2));

    p.root().move_into_the_past();
    assert_that(p.cargo("bench").arg("-v"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[FRESH] bar v0.0.1 ({dir}/bar)
[FRESH] foo v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] `[..]target[/]release[/]deps[/]foo-[..][EXE] --bench`
[RUNNING] `[..]target[/]release[/]deps[/]bench-[..][EXE] --bench`", dir = p.url()))
                       .with_stdout_contains_n("test foo ... bench: [..]", 2));
}

#[test]
fn bench_twice_with_build_cmd() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]
            name = "foo"
            version = "0.0.1"
            authors = []
            build = "build.rs"
        "#)
        .file("build.rs", "fn main() {}")
        .file("src/lib.rs", "
            #![feature(test)]
            #[cfg(test)]
            extern crate test;
            #[bench]
            fn foo(_b: &mut test::Bencher) {}
        ")
        .build();

    assert_that(p.cargo("bench"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[COMPILING] foo v0.0.1 ({dir})
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]", dir = p.url()))
                       .with_stdout_contains("test foo ... bench: [..]"));

    assert_that(p.cargo("bench"),
                execs().with_status(0)
                       .with_stderr("[FINISHED] release [optimized] target(s) in [..]
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                       .with_stdout_contains("test foo ... bench: [..]"));
}

#[test]
fn bench_with_examples() {
    if !is_nightly() { return }

    let p = project("testbench")
        .file("Cargo.toml", r#"
            [package]
            name = "testbench"
            version = "6.6.6"
            authors = []

            [[example]]
            name = "teste1"

            [[bench]]
            name = "testb1"
        "#)
        .file("src/lib.rs", r#"
            #![cfg_attr(test, feature(test))]
            #[cfg(test)]
            extern crate test;
            #[cfg(test)]
            use test::Bencher;

            pub fn f1() {
                println!("f1");
            }

            pub fn f2() {}

            #[bench]
            fn bench_bench1(_b: &mut Bencher) {
                f2();
            }
        "#)
        .file("benches/testb1.rs", "
            #![feature(test)]
            extern crate testbench;
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_bench2(_b: &mut Bencher) {
                testbench::f2();
            }
        ")
        .file("examples/teste1.rs", r#"
            extern crate testbench;

            fn main() {
                println!("example1");
                testbench::f1();
            }
        "#)
        .build();

    assert_that(p.cargo("bench").arg("-v"),
                execs().with_status(0)
                       .with_stderr(&format!("\
[COMPILING] testbench v6.6.6 ({url})
[RUNNING] `rustc [..]`
[RUNNING] `rustc [..]`
[RUNNING] `rustc [..]`
[FINISHED] release [optimized] target(s) in [..]
[RUNNING] `{dir}[/]target[/]release[/]deps[/]testbench-[..][EXE] --bench`
[RUNNING] `{dir}[/]target[/]release[/]deps[/]testb1-[..][EXE] --bench`",
                dir = p.root().display(), url = p.url()))
                       .with_stdout_contains("test bench_bench1 ... bench: [..]")
                       .with_stdout_contains("test bench_bench2 ... bench: [..]"));
}

#[test]
fn test_a_bench() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            authors = []
            version = "0.1.0"

            [lib]
            name = "foo"
            test = false
            doctest = false

            [[bench]]
            name = "b"
            test = true
        "#)
        .file("src/lib.rs", "")
        .file("benches/b.rs", r#"
            #[test]
            fn foo() {}
        "#)
        .build();

    assert_that(p.cargo("test"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.1.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
[RUNNING] target[/]debug[/]deps[/]b-[..][EXE]")
                       .with_stdout_contains("test foo ... ok"));
}

#[test]
fn test_bench_no_run() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            authors = []
            version = "0.1.0"
        "#)
        .file("src/lib.rs", "")
        .file("benches/bbaz.rs", r#"
            #![feature(test)]

            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_baz(_: &mut Bencher) {}
        "#)
        .build();

    assert_that(p.cargo("bench").arg("--no-run"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.1.0 ([..])
[FINISHED] release [optimized] target(s) in [..]
"));
}

#[test]
fn test_bench_no_fail_fast() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", r#"
            #![feature(test)]
            #[cfg(test)]
            extern crate test;
            fn hello() -> &'static str {
                "hello"
            }

            pub fn main() {
                println!("{}", hello())
            }

            #[bench]
            fn bench_hello(_b: &mut test::Bencher) {
                assert_eq!(hello(), "hello")
            }

            #[bench]
            fn bench_nope(_b: &mut test::Bencher) {
                assert_eq!("nope", hello())
            }"#)
        .build();

    assert_that(p.cargo("bench").arg("--no-fail-fast"),
                execs().with_status(101)
                    .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                    .with_stdout_contains("running 2 tests")
                    .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                    .with_stdout_contains("test bench_hello [..]")
                    .with_stdout_contains("test bench_nope [..]"));
}

#[test]
fn test_bench_multiple_packages() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            authors = []
            version = "0.1.0"

            [dependencies.bar]
            path = "../bar"

            [dependencies.baz]
            path = "../baz"
        "#)
        .file("src/lib.rs", "")
        .build();

    let _bar = project("bar")
        .file("Cargo.toml", r#"
            [project]
            name = "bar"
            authors = []
            version = "0.1.0"

            [[bench]]
            name = "bbar"
            test = true
        "#)
        .file("src/lib.rs", "")
        .file("benches/bbar.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_bar(_b: &mut Bencher) {}
        "#)
        .build();

    let _baz = project("baz")
        .file("Cargo.toml", r#"
            [project]
            name = "baz"
            authors = []
            version = "0.1.0"

            [[bench]]
            name = "bbaz"
            test = true
        "#)
        .file("src/lib.rs", "")
        .file("benches/bbaz.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_baz(_b: &mut Bencher) {}
        "#)
        .build();


    assert_that(p.cargo("bench").arg("-p").arg("bar").arg("-p").arg("baz"),
                execs().with_status(0)
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]bbaz-[..][EXE]")
                       .with_stdout_contains("test bench_baz ... bench: [..]")
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]bbar-[..][EXE]")
                       .with_stdout_contains("test bench_bar ... bench: [..]"));
}

#[test]
fn bench_all_workspace() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"

            [dependencies]
            bar = { path = "bar" }

            [workspace]
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .file("benches/foo.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_foo(_: &mut Bencher) -> () { () }
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
        "#)
        .file("bar/src/lib.rs", r#"
            pub fn bar() {}
        "#)
        .file("bar/benches/bar.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_bar(_: &mut Bencher) -> () { () }
        "#)
        .build();

    assert_that(p.cargo("bench")
                 .arg("--all"),
                execs().with_status(0)
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]bar-[..][EXE]")
                       .with_stdout_contains("test bench_bar ... bench: [..]")
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                       .with_stdout_contains("test bench_foo ... bench: [..]"));
}

#[test]
fn bench_all_exclude() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"

            [workspace]
            members = ["bar", "baz"]
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
        "#)
        .file("bar/src/lib.rs", r#"
            #![feature(test)]
            #[cfg(test)]
            extern crate test;

            #[bench]
            pub fn bar(b: &mut test::Bencher) {
                b.iter(|| {});
            }
        "#)
        .file("baz/Cargo.toml", r#"
            [project]
            name = "baz"
            version = "0.1.0"
        "#)
        .file("baz/src/lib.rs", r#"
            #[test]
            pub fn baz() {
                break_the_build();
            }
        "#)
        .build();

    assert_that(p.cargo("bench")
                    .arg("--all")
                    .arg("--exclude")
                    .arg("baz"),
                execs().with_status(0)
                    .with_stdout_contains("\
running 1 test
test bar ... bench:           [..] ns/iter (+/- [..])"));
}

#[test]
fn bench_all_virtual_manifest() {
    if !is_nightly() { return }

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
        .file("foo/benches/foo.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_foo(_: &mut Bencher) -> () { () }
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
        "#)
        .file("bar/src/lib.rs", r#"
            pub fn bar() {}
        "#)
        .file("bar/benches/bar.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_bar(_: &mut Bencher) -> () { () }
        "#)
        .build();

    // The order in which foo and bar are built is not guaranteed
    assert_that(p.cargo("bench")
                 .arg("--all"),
                execs().with_status(0)
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]bar-[..][EXE]")
                       .with_stdout_contains("test bench_bar ... bench: [..]")
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                       .with_stdout_contains("test bench_foo ... bench: [..]"));
}

// https://github.com/rust-lang/cargo/issues/4287
#[test]
fn legacy_bench_name() {
    if !is_nightly() { return }

    let p = project("foo")
        .file("Cargo.toml", r#"
            [project]
            name = "foo"
            version = "0.1.0"

            [[bench]]
            name = "bench"
        "#)
        .file("src/lib.rs", r#"
            pub fn foo() {}
        "#)
        .file("src/bench.rs", r#"
            #![feature(test)]
            extern crate test;

            use test::Bencher;

            #[bench]
            fn bench_foo(_: &mut Bencher) -> () { () }
        "#)
        .build();

    assert_that(p.cargo("bench"), execs().with_status(0).with_stderr_contains("\
[WARNING] path `[..]src[/]bench.rs` was erroneously implicitly accepted for benchmark `bench`,
please set bench.path in Cargo.toml"));
}

#[test]
fn bench_virtual_manifest_all_implied() {
    if !is_nightly() { return }

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
        .file("foo/benches/foo.rs", r#"
            #![feature(test)]
            extern crate test;
            use test::Bencher;
            #[bench]
            fn bench_foo(_: &mut Bencher) -> () { () }
        "#)
        .file("bar/Cargo.toml", r#"
            [project]
            name = "bar"
            version = "0.1.0"
        "#)
        .file("bar/src/lib.rs", r#"
            pub fn bar() {}
        "#)
        .file("bar/benches/bar.rs", r#"
            #![feature(test)]
            extern crate test;
            use test::Bencher;
            #[bench]
            fn bench_bar(_: &mut Bencher) -> () { () }
        "#)
        .build();

    // The order in which foo and bar are built is not guaranteed
    
    assert_that(p.cargo("bench"),
                execs().with_status(0)
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]bar-[..][EXE]")
                       .with_stdout_contains("test bench_bar ... bench: [..]")
                       .with_stderr_contains("\
[RUNNING] target[/]release[/]deps[/]foo-[..][EXE]")
                       .with_stdout_contains("test bench_foo ... bench: [..]"));
}
