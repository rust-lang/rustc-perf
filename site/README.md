# rustc-perf Site

This is the website for Rust compiler performance monitoring. The website
contains a backend to process the raw data and expose it to the frontend,
which displays graphs to the user.

This also contains a GitHub bot to trigger on-demand benchmarking.

## Setup
You can build the website in multiple ways:
- Download a precompiled [nightly build](https://github.com/rust-lang/rustc-perf/releases).
- Use the provided Docker image:
  ```console
  $ git clone https://github.com/rust-lang/rustc-perf.git
  $ docker build -t rustc-perf .
  ```
- Build it yourself:
  ```console
  $ git clone https://github.com/rust-lang/rustc-perf.git
  
  # Build frontend
  $ cd site/frontend
  $ npm install
  $ npm run build
  
  # Build website binary
  $ cd ../..
  $ cargo build --bin site --release
  ```
  The `--release` flag is on purpose, reducing startup time 15x from roughly 15
  seconds to 1; this is on the production database and smaller data sets are
  likely to load considerably faster.

For more information about working with the frontend, see [this README](frontend/README.md).

## Launching

If you've collected data locally, you will likely want to point the site at a
local database. By default, a database called results.db located at the root of 
the project will be used. You can optionally pass a path to a database
if you don't want to use the default.

The site launches on port 2346 by default, which can be overridden by setting
the `PORT` environment variable.

```console
$ ./target/release/site <database>
```
