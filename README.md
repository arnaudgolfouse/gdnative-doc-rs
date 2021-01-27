# gdnative-doc

This is a documentation tool for [gdnative](https://github.com/godot-rust/godot-rust) projects.

**WARNING:** very unstable at the moment.

## Usage

There are 2 ways to use this:

### Command line

By executing `cargo build --release`, you should get an executable at `target/release/gdnative-doc-cli`. Then, to test it on a rust crate:
```
cd <path-to-my-crate>
gdnative-doc-cli --md <path-to-markdown-output>
```

To get more options, run `gdnative-doc-cli --help`.

### Build script (soon)

You can also set
```
[build-dependencies]
gdnative-doc = "*"
```
in your `Cargo.toml`, and then use the `Builder` structure to build the documentation:
```rust
// build.rs
use gdnative_doc::{Builder, Backend};
use std::path::PathBuf;

fn main() {
    Builder::new()
        .user_config(PathBuf::from("config.toml"))
        .add_backend(Backend::Markdown {
            output_dir: PathBuf::from("doc/markdown"),
        })
        .build()
        .unwrap();
}
```

More informations can be found in the [documentation](TODO).

The format of the configuration file can be found [here](configuration_file-format.md).

## Example

An example of the output can be found in `example/dijkstra-map`. It can be re-built by 
simply `cd`ing into the `example/dijkstra-map` directory, and running `cargo build`.

## Limitations

At the moment, [syn](https://crates.io/crates/syn) is used to parse rust and search for the `struct` and `impl`s. This is not optimal however and might sometime mess up link resolution.
[rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) libraries will probably be used in the future to avoid this.