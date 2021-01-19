use std::collections::HashMap;

use super::Backend;
use crate::config::UserConfig;

/// Configuration options for [Generator](super::Generator).
pub struct Config {
    /// Link to godot classes' documentation
    pub(crate) godot_classes: HashMap<String, String>,
    /// Mapping from Rust types to gdscript types
    pub(crate) rust_to_godot: HashMap<String, String>,
    /// User-defined overrides
    pub(crate) overrides: HashMap<String, String>,
    /// Markdown options
    pub(crate) options: pulldown_cmark::Options,
    pub backend: Backend,
}

// TODO: get these from godot's documentation ?
// https://github.com/godotengine/godot-docs/tree/master/classes
// OR search https://github.com/godot-rust/godot-rust, they may have found a better way
const GODOT_CLASSES: &[&str] = &[
    "Array",
    "AStar",
    "AStar2D",
    "bool",
    "Dictionary",
    "float",
    "int",
    "PoolIntArray",
    "PoolRealArray",
    "Rect2",
    "Transform",
    "Transform2D",
    "Variant",
    "Vector2",
];

const RUST_TO_GODOT: &[(&str, &str)] = &[
    ("i32", "int"),
    ("i64", "int"),
    ("f64", "float"),
    ("VariantArray", "Array"),
    ("Int32Array", "PoolIntArray"),
    ("Float32Array", "PoolRealArray"),
];

impl Default for Config {
    fn default() -> Self {
        let mut godot_classes = HashMap::new();
        let mut rust_to_godot = HashMap::new();

        for class in GODOT_CLASSES {
            godot_classes.insert(
                class.to_string(),
                format!(
                    "https://docs.godotengine.org/en/stable/classes/class_{}.html",
                    class.to_lowercase()
                ),
            );
        }

        for (rust, godot) in RUST_TO_GODOT {
            rust_to_godot.insert(rust.to_string(), godot.to_string());
        }

        Self {
            godot_classes,
            rust_to_godot,
            overrides: HashMap::new(),
            options: pulldown_cmark::Options::ENABLE_STRIKETHROUGH,
            backend: Backend::Markdown,
        }
    }
}

impl Config {
    pub fn with_user_config(user_config: UserConfig) -> Self {
        let mut config = Self::default();
        if let Some(markdown_options) = user_config.markdown_options() {
            config.options = markdown_options;
        }
        if let Some(overrides) = user_config.overrides {
            config.overrides = overrides;
        }
        if let Some(backend) = user_config.backend {
            config.backend = match backend.as_str() {
                "markdown" => Backend::Markdown,
                "html" => Backend::Html,
                _ => {
                    log::error!("unknown backend: {}", backend);
                    panic!("unknown backend: {}", backend)
                }
            }
        }

        config
    }

    pub fn backend_extension(&self) -> &'static str {
        self.backend.extension()
    }
}
