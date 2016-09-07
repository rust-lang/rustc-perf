rustc-perf
----------

This is the website for Rust compiler performance monitoring. The website
contains a backend to process the raw data and expose it to the frontend,
which displays graphs to the user.

https://github.com/rust-lang-nursery/rustc-timing contains the raw data
from which rustc-perf pulls.

https://github.com/rust-lang-nursery/rustc-benchmarks runs elsewhere and
performs the tests whose results rustc-perf displays, then pushes them
to the timings repo.

Setup
-----

```
sudo apt-get install apache2 git
git clone https://github.com/rust-lang-nursery/rustc-perf.git /var/www/html/
cd /var/www/html/rustc-perf
git clone https://github.com/nrc/rustc-timing.git data
```

The following lines in `/etc/apache2/sites-enabled/000-default.conf` allow the
frontend in its current configuration to get data from the API (served by
either the JS or the Rust backend)

```
    ProxyPass /perf http://localhost:2346
    ProxyPassReverse /perf http://localhost:2346
```

The backend currently depends on a POST request being sent to the /perf/onpush
(with above configuration) when new data is made available in the timings
repo.

Launching
---------

It can be run with the following command from the rs-backend directory:

```
cargo run --release ../data
```

Rust backend
------------

The new Rust backend can be built with `cargo build` and tested with
`cargo test`.
