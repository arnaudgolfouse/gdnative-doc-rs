# gdnative-doc-cli

Command-line alternative of [gdnative-doc](https://github.com/arnaudgolfouse/gdnative-doc-rs).

**Note**: It should soon be on [crates.io](https://crates.io), but for now the described installation doesn't work.

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