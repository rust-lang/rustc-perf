# wg-grammar

This is the home for the Rust Grammar Working Group. The goal of the working
group is to satisfy [RFC 1331] and produce a testable, canonical grammar for
the Rust language. The primary audiences for the grammar are:

- Rust RFC authors who wish to propose and communicate changes.
- rustc and Rust tool developers who need an authoritative definition of the
  grammar.
- To assist documentation efforts to communicate valid Rust syntax to users,
  and to facilitate the [Rust language specification].

The grammar tools produced here are not intended to be used directly within
rustc, or any other existing tools.

## Meeting Schedule

Meetings take place on the [#wg-grammar channel on Discord][discord] at [20:00
CET][time] every other Wednesday. Feel free to drop by the Discord channel if
you are interested!

[discord]: https://discord.gg/dj9NjJR
[time]: https://time.is/compare/2000_in_CET

## Test Suite

This project includes a executable binary for exercising the grammar. Use
`cargo run` to build and run it. Using `--release` is encouraged as it is many
times faster. It supports several subcommands to point it at different files
or directories to examine. For example, if you check out the submodule
(explained below), you can run the tests with the following command:

    cargo run --release -- dir external/rust/src

This repository includes a submodule to the main rust repo to use as a
collection of Rust code to test against the grammar. The command `git
submodule update --init` may be used to fetch it. However, it is not necessary
and you may run the tool against any collection of Rust code at your disposal.

## Links

- [Meeting notes](misc/meeting-notes.md)
- [Resources](misc/resources.md)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[RFC 1331]: https://github.com/rust-lang/rfcs/blob/master/text/1331-grammar-is-canonical.md
[Rust language specification]: https://github.com/rust-lang-nursery/reference/
