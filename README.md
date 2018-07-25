# Rust Compiler Performance Monitoring & Benchmarking

just testing

This repository contains two primary crates: `collector` and `site`. Collector gathers data for each
bors commit and the site displays the data.

The primary required setup is to provide a folder with a `retries` file and a `times` folder. Data
is gathered into https://github.com/rust-lang-nursery/rustc-timing by Rust Infrastructure; cloning
that is the best approach for working on the frontend.

Additional documentation on running and setting up the frontend and backend can be found in README
files in the `collector` and `site` directories.

