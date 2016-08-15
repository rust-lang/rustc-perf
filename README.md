rustc-perf
----------

This is the website for Rust compiler performance monitoring. The website
contains a backend to process the raw data and expose it to the frontend,
which displays graphs to the user.

https://github.com/nrc/rustc-timing contains the raw data from which rustc-perf
pulls.

https://github.com/nrc/benchmarks runs elsewhere and performs the tests whose
results rustc-perf displays, then pushes them to the benchmarks repo.

Setup
-----

```
sudo apt-get install npm apache2 nodejs
git clone https://github.com/nrc/rustc-perf.git /var/www/html/
cd /var/www/html/rustc-perf; ./init.sh
```

The following lines in `/etc/apache2/sties-enabled/000-default.conf` allow the
frontend in its current configuration to get data from the API (served by
`backend/perf.js`)

```
    ProxyPass /perf http://localhost:2346
    ProxyPassReverse /perf http://localhost:2346
```

Launching
---------

It can be run with:

```
forever backend/perf.js ../data
```

Rust backend
------------

You can find a new, work-in-progress Rust backend in rs-backend. You can build
using `cargo build` and test with `cargo test`.

