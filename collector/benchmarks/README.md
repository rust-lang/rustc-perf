# The Benchmark Suite

This file describes the programs in the benchmark suite and explains why they
were included.

## Real programs that are important

These are real programs that are important in some way, and worth tracking.

- **cargo**: The Rust package manager. An important program in the Rust
  ecosystem.
- **clap-rs**: A command line argument parser. A crate used by many Rust
  programs.
- **cranelift-codegen**: The largest crate from a code generator. Used by
  Firefox.
- **crates.io**: Source code for `crates.io`. An important site in the
  Rust ecosystem.
- **futures**: A futures implementation. Used by many Rust programs.
- **helloworld**: A trivial program. Gives a lower bound on compile time.
- **hyper**: An HTTP library. An important part of the Rust ecosystem.
- **piston-image**: A modular game engine. An interesting Rust program.
- **regex**: A regular expression parser. Used by many Rust programs.
- **ripgrep**: A line-oriented search tool. A widely-used utility.
- **script-servo**: Servo's `script` crate. A particularly large crate.
- **serde**: A serialization/deserialization crate. Used by many other
  Rust programs.
- **style-servo**: Servo's `style` crate. A large crate, and one used by
  Firefox.
- **syn**: A library for parsing Rust code. An important part of the Rust
  ecosystem.
- **tokio-webpush-simple**: A simple web server built with tokio. Uses futures
  a lot.
- **webrender**: A web renderer. Used by Firefox and Servo.

## Real programs that stress the compiler

These are real programs that are known to stress the compiler in interesting
ways.

- **encoding**: Character encoding support. Contains some large tables.
- **html5ever**: An HTML parser. Stresses macro parsing code significantly.
- **inflate**: An old implementation of the DEFLATE algorithm. Stresses the
  compiler in certain ways.
- **keccak**: A cryptography algorithm. Contains a very high number of locals
  and basic blocks.
- **packed-simd**: Portable packed SIMD vectors. Exercises SIMD features
  heavily, and uses a compiler plugin.
- **sentry-cli**: A command line utility.
  [Reported](https://github.com/rust-lang-nursery/rustc-perf/issues/232#issuecomment-391439420)
  as being slow to compile.
- **ucd**: A Unicode crate. Contains large statics that
  [stress](https://github.com/rust-lang/rust/issues/53643) the borrow checker's
  implementation of NLL.
- **unicode_normalization**: Unicode character composition and decomposition
  utilities. Uses huge `match` statements that stress the compiler in unusual
  ways.
- **wg-grammar**: A parser generator.
  [Stresses](https://github.com/rust-lang/rust/issues/58178) the borrow
  checker's implementation of NLL.

## Artificial stress tests

These are artificial programs that stress one particular aspect of the
compiler. Some are entirely artificial, and some are extracted from real
programs.

- **await-call-tree**: A tree of async fns that await each other, creating a
  large type composed of many repeated `impl Future` types. Such types caused
  [poor performance](https://github.com/rust-lang/rust/issues/65147) in the
  past.
- **coercions**: Contains a static array with 65,536 string literals, which
  caused [poor performance](https://github.com/rust-lang/rust/issues/32278) in
  the past.
- **ctfe-stress-2**: A stress test for compile-time function evaluation.
- **ctfe-stress-uninit**: A stress test for representation of values with
  uninitialized bytes in compile-time function evaluation.
- **deeply-nested**: A small program that caused [exponential
  behavior](https://github.com/rust-lang/rust/issues/38528) in the past.
- **deep-vector**: A test containing a single large vector of zeroes, which
  caused [poor performance](https://github.com/rust-lang/rust/issues/20936) in
  the past.
- **issue-46449**: A small program that caused [poor
  performance](https://github.com/rust-lang/rust/issues/46449) in the past.
- **regression-31157**: A small program that caused a [large performance
  regression](https://github.com/rust-lang/rust/issues/31157) from the past.
- **tuple-stress**: Contains a single array of 65,535 nested `(i32, (f64, f64,
  f64))` tuples. The data was extracted and reduced from a [program dealing
  with grid coordinates](https://github.com/urschrei/ostn15_phf) that was
  causing rustc to [run out of
  memory](https://github.com/rust-lang/rust/issues/36799).
- **unify-linearly**: Contains many variables that all have equality relations
  between them, which caused [exponential
  behavior](https://github.com/rust-lang/rust/pull/32062) in the past.
- **unused-warnings**: Contains many unused imports, which caused [quadratic
  behavior](https://github.com/rust-lang/rust/issues/43572) in the past.

