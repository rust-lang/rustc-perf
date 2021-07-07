# rustc-perf Site

This is the website for Rust compiler performance monitoring. The website
contains a backend to process the raw data and expose it to the frontend,
which displays graphs to the user.

This also contains a GitHub bot to trigger on-demand benchmarking.

## Setup

The site launches on port 2346 by default, which can be overridden by setting
the `PORT` environment variable.

```
git clone https://github.com/rust-lang/rustc-perf.git
```

## Launching

If you've collected data locally, you will likely want to point the site at a
local database. By default, a database called results.db located at the root of 
the project will be used. You can optionally pass a path to a database
if you don't want to use the default.

```
cargo run --bin site --release <database>
```

The `--release` flag is on purpose, reducing startup time 15x from roughly 15
seconds to 1; this is on the production database and smaller data sets are
likely to load considerably faster.
