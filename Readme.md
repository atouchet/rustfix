# rustfix

The goal of this tool is to read and apply the suggestions made by rustc.

## Current status

Currently, rustfix is split into two crates:

- `rustfix`, a library for consuming and applying suggestions in the format that `rustc` outputs
- `cargo-fix`, a binary that works as a Cargo subcommand and that end users will use to fix their code

The magic of rustfix is entirely dependent on the diagnostics implemented in the Rust compiler (and external lints, like [Clippy]).

[Clippy]: https://github.com/rust-lang/rust-clippy

## Installation

To use the rustfix library, add it to your `Cargo.toml`.

To get the tool to automatically fix warnings in, run `cargo install cargo-fix`. This will give you `cargo fix`.

## Using `cargo fix` to transition to Rust 2018

Instructions on how to use this tool to transition a crate to Rust 2018 can be
found [in the Rust Edition Guide.](https://doc.rust-lang.org/nightly/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
