# godoc-doc-rs

This is a documentation tool for [gdnative](https://github.com/godot-rust/godot-rust) projects.

**This is a prototype, most things will not work properly**

## Usage

To test on a rust crate:
1. Create a `config.toml` file, with at least an `output` entry.
2. Run `cargo run -- <path-to-config.toml>`.

## Example

An example of the output can be found in `/example`.

## syn VS rust-analyzer

At the moment, [syn](https://crates.io/crates/syn) is used to parse rust and search for the `struct` and `impl`s. This is not optimal however, so another solution is being crafted on the `rust-analyzer` branch, that will use [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) libraries.
