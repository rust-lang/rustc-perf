# The Benchmark Suite

This file describes the programs in the benchmark suite and explains why they
were included.

The suite changes over time. Sometimes the code for a benchmark is updated, in
which case a small suffix will be added (starting with "-2", then "-3", and so
on.)

There are three categories of benchmarks, **Primary**, **Secondary**, and
**Stable**.

## Primary

These are real programs that are important in some way, and worth tracking.
They mostly consist of real-world crates.

- **cargo-0.60.0**: The Rust package manager. A large program, and an important
  one in the Rust ecosystem.
- **clap-3.1.6**: A command line argument parser. A crate used by many Rust
  programs.
- **cranelift-codegen**: The largest crate from a code generator. Used by
  Firefox.
- **cranelift-codegen-0.82.1**: The largest crate from a code generator. Used by
  wasmtime.
- **diesel-1.4.8**: A type safe SQL query builder. Utilizes the type system to
  ensure a lot of invariants. Stresses anything related to resolving
  trait bounds, by having a lot of trait impls for a large number of different
  types.
- **helloworld**: A trivial program. Gives a lower bound on compile time.
- **html5ever**: An HTML parser. Stresses macro parsing code significantly.
- **hyper-2**: A fairly large crate. Utilizes async/await, and used by
  many Rust programs.
- **image-0.24.1**: Basic image processing functions and methods for 
  converting to and from various image formats. Used often in graphics 
  programming.
- **regex-1.5.5**: A regular expression parser. Used by many Rust programs.
- **ripgrep-13.0.0**: A line-oriented search tool. A widely-used utility.
- **serde-1.0.136**: A serialization/deserialization crate. Used by many other
  Rust programs.
- **stm32f4**: A crate that has many thousands of blanket impl blocks.
- **stm32f4-0.14.0**: A crate that has many thousands of blanket impl blocks.
- **syn-1.0.89**: A library for parsing Rust code. An important part of the Rust
  ecosystem.
- **unicode_normalization**: Unicode character composition and decomposition
  utilities. Uses huge `match` statements that stress the compiler in unusual
  ways.
- **unicode-normalization-0.1.19**: Unicode character composition and decomposition
  utilities. Uses huge `match` statements that stress the compiler in unusual
  ways.
- **webrender**: A web renderer. Used by Firefox and Servo.

## Secondary

These are either artificial programs or real crates that stress one particular aspect of the
compiler in interesting ways.

- **await-call-tree**: A tree of async fns that await each other, creating a
  large type composed of many repeated `impl Future` types. Such types caused
  [poor performance](https://github.com/rust-lang/rust/issues/65147) in the
  past.
- **coercions**: Contains a static array with 65,536 string literals, which
  caused [poor performance](https://github.com/rust-lang/rust/issues/32278) in
  the past.
- **ctfe-stress-4**: A stress test for compile-time function evaluation.
- **deeply-nested-multi**: A small program containing multiple examples 
  ([one](https://github.com/rust-lang/rust/issues/38528),
  [two](https://github.com/rust-lang/rust/issues/72408),
  [three](https://github.com/rust-lang/rust/issues/75992))
  of code that caused exponential behavior in the past.
- **deep-vector**: A test containing a single large vector of zeroes, which
  caused [poor performance](https://github.com/rust-lang/rust/issues/20936) in
  the past.
- **derive**: A large number of simple structs with a `#[derive]` attribute for common built-in traits such as Copy and Debug.
- **externs**: A large number of extern functions has caused [slowdowns in the past](https://github.com/rust-lang/rust/pull/78448).
- **issue-46449**: A small program that caused [poor
  performance](https://github.com/rust-lang/rust/issues/46449) in the past.
- **issue-58319**: A small program that caused [poor
  performance](https://github.com/rust-lang/rust/issues/58319) in the past.
- **issue-88862**: A MCVE of a program that had a
  [severe performance regression](https://github.com/rust-lang/rust/issues/88862)
  when trying to normalize large opaque types with late-bound regions.
- **keccak**: A cryptography algorithm. Contains a huge function with a very
  high number of locals and basic blocks.
- **many-assoc-items**: Contains a struct with many associated items, which
  caused [quadratic behavior](https://github.com/rust-lang/rust/issues/68957)
  in the past.
- **match-stress**: Contains examples 
  (one involving [a huge enum](https://github.com/rust-lang/rust/issues/7462),
  one involving
  [`exhaustive_patterns`](https://github.com/rust-lang/rust/pull/79394)) of
  `match` code that caused bad performance in the past.
- **projection-caching**: A small program that causes extremely, deeply nested
  types which stress the trait system's projection cache. Removing that cache
  resulted in hours long compilations for some programs using futures,
  actix-web and other libraries with similarly nested type combinators.
- **regression-31157**: A small program that caused a [large performance
  regression](https://github.com/rust-lang/rust/issues/31157) from the past.
- **token-stream-stress**: Constructs a long token stream much like the `quote`
  crate does, which caused [quadratic
  behavior](https://github.com/rust-lang/rust/issues/65080) in the past.
- **tuple-stress**: Contains a single array of 65,535 nested `(i32, (f64, f64,
  f64))` tuples. The data was extracted and reduced from a [program dealing
  with grid coordinates](https://github.com/urschrei/ostn15_phf) that was
  causing rustc to [run out of
  memory](https://github.com/rust-lang/rust/issues/36799).
- **ucd**: A Unicode crate. Contains large statics that
  [stress](https://github.com/rust-lang/rust/issues/53643) the borrow checker's
  implementation of NLL.
- **unify-linearly**: Contains many variables that all have equality relations
  between them, which caused [exponential
  behavior](https://github.com/rust-lang/rust/pull/32062) in the past.
- **unused-warnings**: Contains many unused imports, which caused [quadratic
  behavior](https://github.com/rust-lang/rust/issues/43572) in the past.
- **wf-projection-stress-65510**: A stress test which showcases [quadratic
  behavior](https://github.com/rust-lang/rust/issues/65510) (in the number of
  associated type bounds).
- **wg-grammar**: A parser generator.
  [Stresses](https://github.com/rust-lang/rust/issues/58178) the borrow
  checker's implementation of NLL.

**Stable**

These are benchmarks used in the
[dashboard](https://perf.rust-lang.org/dashboard.html). They provide the
longest continuous data set for compiler performance. As a result, they are
quite old (e.g. 2017 or earlier), and not necessarily reflective of typical
Rust code being written today.

- **encoding**: An old crate providing character encoding support. Contains
  some large tables.
- **futures**: v0.1.0 of the popular `futures` crate, which was used by many
  Rust programs. Newer versions of this crate (e.g. v0.3.21 from February 2021)
  contain very little code, instead relying on sub-crates. This makes them less
  interesting as benchmarks, because we only measure final crate compilation.
  This is why there is no futures crate among the primary benchmarks.
- **html5ever**: See above.
- **inflate**: An old implementation of the DEFLATE algorithm. Contains
  a very large function containing many locals and basic blocks, similar to
  `keccak` but less extreme.
- **regex**: See above. This is an older version of the crate.
- **piston-image**: See above. This is an older version of the `image` crate.
- **style-servo**: An old version of Servo's `style` crate. A large crate, and
  one used by old versions of Firefox.
- **syn**: See above. This is an older version (0.11.11) of the crate.
- **tokio-webpush-simple**: A simple web server built with a very old version
  of tokio. Uses futures a lot, but doesn't use `async`/`await`.
