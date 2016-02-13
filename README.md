rustc-perf
----------

This is the website for Rust compiler performance monitoring. 

https://github.com/nrc/benchmarks contains the raw data from which rustc-perf
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

The following lines in `/etc/apache2/sties-enabled/000-default.conf` proxy
traffic to the app's custom port. This is historical cruft from its previous
shared setup. 

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





