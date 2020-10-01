# Rust Compiler Performance Monitoring & Benchmarking

This repository contains two primary crates: `collector` and `site`. Collector gathers data for each
bors commit and the site displays the data and provides a GitHub bot for on-demand benchmarking.

The primary required setup is to provide a folder with a `retries` file and a `times` folder. Data
is gathered into https://github.com/rust-lang/rustc-timing by Rust Infrastructure; cloning
that is the best approach for working on the frontend.

Additional documentation on running and setting up the frontend and backend can
be found in the `README` files in the `collector` and `site` directories.

Additional documentation on the benchmark programs can be found in the `README`
file in the `collector/benchmarks` directory.

Rust is a multi-paradigm programming language focused on performance and safety,
especially safe concurrency. Rust is syntactically similar to C++, and provides
memory safety without using garbage collection. ... The designers refined the 
language while writing the Servo layout or browser engine, and the Rust compiler.
