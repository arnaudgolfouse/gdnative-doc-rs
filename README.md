# godoc-doc-rs

This is a documentation tool for [gdnative](https://github.com/godot-rust/godot-rust) projects.

**This is a prototype, nothing works at the moment**

## syn VS rust-analyzer

In principle, godoc-doc-rs is made with [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) libraries. This is clearly not ready at the moment, so a fallback implementation can be found in the `syn` branch, that uses [syn](https://crates.io/crates/syn).