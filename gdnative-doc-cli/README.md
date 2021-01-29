# gdnative-doc-cli

Command-line alternative of [gdnative-doc](https://github.com/arnaudgolfouse/gdnative-doc-rs).

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