# Configuration file

The behaviour of `gdnative-doc` can be configured via a [toml configuration file](https://toml.io/en/).

The current options are:

- ## godot_version

  Specify a godot version.

  ### Valid versions

  Accepted versions are `"3.2"`, `"3.3"`, `"3.4"` or `"3.5"`.

  ### Default

  Defaults to `"3.5"`.

  ### Example

  ```toml
  # Use godot 3.3 for class names and documentation linking.
  godot_version = "3.3"
  ```

- ## url_overrides

  Here you can specify a list of items for which the linking url should be overriden.

  ### Example

  ```toml
  # link `bool` to the latest documentation instead.
  url_overrides = { bool = "https://docs.godotengine.org/en/latest/classes/class_bool.html" }
  ```

- ## rename_classes

  Here you can declare a list of structures that will be renamed.

  This is useful because GDNative allows defining a `script_class_name` in the `.gdns` file.

  ### Example

  ```rust
  // in lib.rs

  #[derive(NativeClass)]
  #[inherit(Reference)]
  /// My Rust interface
  pub struct RustStructure {}
  ```

  ```toml
  rename_classes = { RustStructure = "GodotClass" }
  ```

- ## markdown_options

  List of optional markdown options.

  ### Valid options

  - FOOTNOTES
  - SMART_PUNCTUATION
  - STRIKETHROUGH
  - TABLES
  - TASKLISTS

  ### Default

  No option enabled.

  ### Example

  ```toml
  markdown_options = ["STRIKETHROUGH", "TABLES", "TASKLISTS"]
  ```

- ## opening_comment

  Boolean that control whether or not to include a comment in the generated files.

  The comment includes information such that the file was automatically generated, the name of the source file it originated from...

  ### Default

  `true`

  ### Example

  ```toml
  opening_comment = false
  ```
