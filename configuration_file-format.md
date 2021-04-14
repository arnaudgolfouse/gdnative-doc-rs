`gdnative-doc` allows for an optional `toml` configuration file. The current valid entries are :

- `url_overrides: HashMap<String, String>`:
  List of items for which the linking url should be overriden.
- `rename_classes: HashMap<String, String>`:
  Renaming of types when going from Rust to Godot.

  This is useful because GDNative allows defining a `script_class_name` in the
  `.gdns` file.

- `markdown_options: Vec<String>`:
  Optional markdown options.

  ### Valid options

  - FOOTNOTES
  - SMART_PUNCTUATION
  - STRIKETHROUGH
  - TABLES
  - TASKLISTS

  ### Default

  No option enabled.

- `opening_comment: bool`:
  Control whether or not to include commentary in the generated files.

  The commentary includes information such that the file was automatically
  generated, the name of the source file it originated from...

  ### Default

  `true`
