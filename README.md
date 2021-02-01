# gdnative-doc

[![MIT license](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/arnaudgolfouse/gdnative-doc-rs/blob/main/LICENSE)
[![Latest Version](https://img.shields.io/crates/v/gdnative-doc.svg)](https://crates.io/crates/gdnative-doc)
[![Docs Status](https://docs.rs/gdnative-doc/badge.svg)](https://docs.rs/gdnative-doc)

This is a documentation tool for [godot-rust](https://github.com/godot-rust/godot-rust) projects.

**WARNING:** very unstable at the moment.

The goal of this tool is to automate writing documentation in Rust code that will be used in gdscript.

## Features

- Keep the documentation synchronized with your code.
- Generate readable and easy to change markdown
- Build table of contents automatically.
- Automatic linking to the [godot documentation](https://docs.godotengine.org/en/stable/index.html).
- Generate [gut](https://github.com/bitwes/Gut) tests from gdscript examples.

## Example

 An example: `process` function |
 :------- |
 Input: Rust |
 ![](assets/process-function-rust.png) |
 Output: markdown |
 ![](assets/process-function-markdown-dark.png) |
 Output: gut |
 ![](assets/process-function-gut.png) |

A more complete example can be found in the [example/dijkstra-map-gd](example/dijkstra-map-gd) directory.

## Usage

This is meant to be used as a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html): Set
```toml
[build-dependencies]
gdnative-doc = "*"
```
In your Cargo.toml. Then you can drive the process with the `Builder` structure:
```rust
// in build.rs
use gdnative_doc::{Builder, Backend};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new()
        .add_backend(Backend::Markdown {
            output_dir: PathBuf::from("doc/markdown"),
        })
        .build()?;
    Ok(())
}
```

More informations can be found in the [documentation](https://docs.rs/gdnative-doc).

The format of the configuration file can be found [here](configuration_file-format.md).

You can also use the [command-line tool](gdnative-doc-cli).

## Limitations

At the moment, [syn](https://crates.io/crates/syn) is used to parse rust and search for the `struct` and `impl`s. This is not optimal however and might sometime mess up link resolution.
[rust-analyzer](https://github.com/rust-analyzer/rust-analyzer) libraries will probably be used in the future to avoid this.
