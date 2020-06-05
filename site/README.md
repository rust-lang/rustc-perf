rustc-perf
----------

This is the website for Rust compiler performance monitoring. The website
contains a backend to process the raw data and expose it to the frontend,
which displays graphs to the user.

This also contains a GitHub bot to trigger on-demand benchmarking.

Setup
-----

The site launches on port 2346 by default, which can be overridden by setting
the `PORT` environment variable.

```
sudo apt-get install git
git clone https://github.com/rust-lang/rustc-perf.git
```

Launching
---------

If you've collected data locally, you will likely want to point the site at that
local database.

Alternatively, if you want to get the data used by the server on
perf.rust-lang.org, you can download an export of the database by running `cargo
run --release --bin fetch-latest <database>`, providing the location to save the
database to (this currently just downloads and decompresses a file, but this may
become more sophisticated in the future); be warned that the database is fairly
large -- 3.8 GB as of this writing, and growing gradually.

We may eventually provide support for partial downloads; please file an issue if
you have a specific use case and we may be able to accommodate it.


```
cargo run --bin site --release <database>
```

The release flag is on purpose, reducing startup time 15x from roughly 15
seconds to 1; this is on the production database and smaller data sets are
likely to load considerably faster.
