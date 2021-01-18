# godoc-doc-rs

This is a documentation tool for [gdnative](https://github.com/godot-rust/godot-rust) projects.

**This is a prototype, nothing works at the moment**

## syn VS rust-analyzer

At the moment, [syn](https://crates.io/crates/syn) is used to parse rust and search for the `struct` and `impl`s. This is not optimal however, so another solution is being crafted on the `rust-analyzer` branch, that will use [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) libraries.