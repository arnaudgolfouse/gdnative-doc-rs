# gdnative-doc-cli


[![MIT license](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/arnaudgolfouse/gdnative-doc-rs/blob/main/LICENSE)
[![Latest Version](https://img.shields.io/crates/v/gdnative-doc.svg)](https://crates.io/crates/gdnative-doc-cli)

Command-line alternative of [gdnative-doc](https://crates.io/crates/gdnative-doc).

## Installing

If you have cargo, simply run 
```
cargo install gdnative-doc-cli
```
This will install `gdnative-doc-cli` in `$CARGO_HOME/bin`.

## Usage

You can test it on a crate that exports gdnative structures with:
```
cd <path-to-my-crate>
gdnative-doc-cli --md <path-to-markdown-output>
```

To get more options, run `gdnative-doc-cli --help`.
