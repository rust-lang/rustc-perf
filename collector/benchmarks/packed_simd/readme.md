# `Simd<[T; N]>` - Implementation of [RFC2366: `std::simd`][rfc2366]

[![Travis-CI Status]][travis] [![Appveyor Status]][appveyor] [![Latest Version]][crates.io] [![docs]][docs.rs]

> This aims to be a 100% conforming implementation of the RFC2366 for stabilization.


# Documentation

* [`master` branch][master_docs]
* [latest release][docs.rs]
* [RFC2366 `std::simd`][rfc2366]

# Examples

Most of the examples come with both a scalar and a vectorized implementation.

* [`aobench`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/aobench)
* [`fannkuch_redux`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/fannkuch_redux)
* [`matrix inverse`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/matrix_inverse)
* [`mandelbrot`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/mandelbrot)
* [`n-body`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/nbody)
* [`spectral_norm`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/spectral_norm)
* [`vector dot product`](https://github.com/rust-lang-nursery/packed_simd/tree/master/examples/dot_product)

# Cargo features

* `into_bits` (default: disabled): enables `FromBits`/`IntoBits` trait
  implementations for the vector types. These allow reinterpreting the bits of a
  vector type as those of another vector type safely by just using the
  `.into_bits()` method.
  
* `coresimd` (default: disabled): when targeting `arm` targets with the `v7` and
  `neon` target features enabled, the `core::arch` component in the standard
  library has typically been compiled without NEON, which means that for some
  operations, NEON won't be used. The `coresimd` feature enables NEON support by
  using `coresimd::arch` compiled with `v7` and `neon` instead of `core::arch`.

# Platform support

The following table describes the supported platforms: `build` shows whether the
library compiles without issues for a given target, while `run` shows whether
the full testsuite passes on the target.

| Linux targets:                    | build     | run     |
|-----------------------------------|-----------|---------|
| `i586-unknown-linux-gnu`          | ✓         | ✓       |
| `i686-unknown-linux-gnu`          | ✓         | ✓       |
| `x86_64-unknown-linux-gnu`        | ✓         | ✓       |
| `arm-unknown-linux-gnueabi`       | ✗         | ✗       |
| `arm-unknown-linux-gnueabihf`     | ✓         | ✓       |
| `armv7-unknown-linux-gnueabi`     | ✓         | ✓       |
| `aarch64-unknown-linux-gnu`       | ✓         | ✓       |
| `mips-unknown-linux-gnu`          | ✓         | ✓       |
| `mipsel-unknown-linux-musl`       | ✓         | ✓       |
| `mips64-unknown-linux-gnuabi64`   | ✓         | ✓       |
| `mips64el-unknown-linux-gnuabi64` | ✓         | ✓       |
| `powerpc-unknown-linux-gnu`       | ✗         | ✗       |
| `powerpc64-unknown-linux-gnu`     | ✗         | ✗       |
| `powerpc64le-unknown-linux-gnu`   | ✗         | ✗       |
| `s390x-unknown-linux-gnu`         | ✓         | ✓*      |
| `sparc64-unknown-linux-gnu`       | ✓         | ✓*      |
| **MacOSX targets:**               | **build** | **run** |
| `x86_64-apple-darwin`             | ✓         | ✓       |
| `i686-apple-darwin`               | ✓         | ✓       |
| **Windows targets:**              | **build** | **run** |
| `x86_64-pc-windows-msvc`          | ✓         | ✓       |
| `i686-pc-windows-msvc`            | ✓         | ✓       |
| `x86_64-pc-windows-gnu`           | ✓         | ✓       |
| `i686-pc-windows-gnu`             | ✓         | ✓       |
| **Android targets:**              | **build** | **run** |
| `x86_64-linux-android`            | ✓         | ✓       |
| `arm-linux-androideabi`           | ✓         | ✓       |
| `aarch64-linux-android`           | ✓         | ✗       |
| **iOS targets:**                  | **build** | **run** |
| `i386-apple-ios`                  | ✓         | ✗       |
| `x86_64-apple-ios`                | ✓         | ✗       |
| `armv7-apple-ios`                 | ✓         | ✗**     |
| `aarch64-apple-ios`               | ✓         | ✗**     |
| **xBSD targets:**                 | **build** | **run** |
| `i686-unknown-freebsd`            | ✗         | ✗**     |
| `x86_64-unknown-freebsd`          | ✗         | ✗**     |
| `x86_64-unknown-netbsd`           | ✗         | ✗**     |
| **Solaris targets:**              | **build** | **run** |
| `x86_64-sun-solaris`              | ✗         | ✗**     |
| **WASM targets:**                 | **build** | **run** |
| `wasm32-unknown-unknown`          | ✓         | ✗**     |

[*] most of the test suite passes correctly on these platform but
there are correctness bugs open in the issue tracker.

[**] it is currently not easily possible to run these platforms on CI.


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `packed_simd` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[travis]: https://travis-ci.org/rust-lang-nursery/packed_simd
[Travis-CI Status]: https://travis-ci.org/rust-lang-nursery/packed_simd.svg?branch=master
[appveyor]: https://ci.appveyor.com/project/gnzlbg/packed-simd-j9hcp/branch/master
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/6blrl5jd44idkt9i?svg=true
[Latest Version]: https://img.shields.io/crates/v/packed_simd.svg
[crates.io]: https://crates.io/crates/packed_simd
[docs]: https://docs.rs/packed_simd/badge.svg
[docs.rs]: https://docs.rs/packed_simd/
[master_docs]: https://rust-lang-nursery.github.io/packed_simd/packed_simd/
[rfc2366]: https://github.com/rust-lang/rfcs/pull/2366
