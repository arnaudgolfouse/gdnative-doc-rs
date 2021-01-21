use super::Backend;
use crate::config::UserConfig;
use pulldown_cmark::{Event, Tag};
use std::collections::HashMap;

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

pub const GODOT_CLASSES: &[&str] = &include!("../../fetch_godot_classes/godot_classes");

const RUST_TO_GODOT: &[(&str, &str)] = &[
    ("i32", "int"),
    ("i64", "int"),
    ("f32", "float"),
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

    pub(crate) fn godot_name<'a>(&'a self, name: &'a str) -> &'a str {
        if let Some(name) = self.rust_to_godot.get(name) {
            name
        } else {
            name
        }
    }

    /// Resolve a name to the class it must link to.
    pub(super) fn resolve(&self, link: &str) -> Option<String> {
        if let Some(link) = self.overrides.get(link).cloned() {
            return Some(link);
        }
        if let Ok(link) = syn::parse_str::<syn::Path>(link) {
            let mut base = match link.segments.last() {
                None => return None,
                Some(base) => base.ident.to_string(),
            };
            if let Some(path) = self.overrides.get(&base).cloned() {
                Some(path)
            } else {
                base = match self.rust_to_godot.get(&base).cloned() {
                    Some(base) => base,
                    None => base,
                };
                if let Some(path) = self.godot_classes.get(&base).cloned() {
                    Some(path)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub(super) fn resolve_event(&self, event: &mut Event) {
        match event {
            Event::Start(Tag::Link(_, dest, _)) | Event::End(Tag::Link(_, dest, _)) => {
                match self.resolve(&dest) {
                    Some(new_dest) => *dest = new_dest.into(),
                    None => {}
                }
            }
            Event::Start(Tag::Heading(n)) | Event::End(Tag::Heading(n)) => *n += 2,
            _ => {}
        }
    }
}
