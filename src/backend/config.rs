use super::Backend;
use crate::config::UserConfig;
use pulldown_cmark::{Event, Tag};
use std::collections::HashMap;

/// Configuration options for [Generator](super::Generator).
pub struct Config {
    /// Link to godot items' documentation
    ///
    /// Contains the link to godot classes, but also `true`, `INF`, `Err`...
    pub(crate) godot_items: HashMap<String, String>,
    /// Mapping from Rust types to gdscript types
    pub(crate) rust_to_godot: HashMap<String, String>,
    /// User-defined overrides
    pub(crate) overrides: HashMap<String, String>,
    /// Markdown options
    pub(crate) markdown_options: pulldown_cmark::Options,
    pub backends: Vec<Backend>,
}

pub const GODOT_CLASSES: &[&str] = &include!("../../fetch_godot_classes/godot_classes");

pub const GODOT_CONSTANTS: &[(&str, &str, &str)] = &[
    ("true", "class_bool", ""),
    ("false", "class_bool", ""),
    ("PI", "class_@gdscript", "constants"),
    ("TAU", "class_@gdscript", "constants"),
    ("INF", "class_@gdscript", "constants"),
    ("NAN", "class_@gdscript", "constants"),
    ("FAILED", "class_@globalscope", "enum-globalscope-error"),
    ("OK", "class_@globalscope", "class-globalscope-error"),
];

const RUST_TO_GODOT: &[(&str, &str)] = &[
    ("i32", "int"),
    ("i64", "int"),
    ("f32", "float"),
    ("f64", "float"),
    ("VariantArray", "Array"),
    ("Int32Array", "PoolIntArray"),
    ("Float32Array", "PoolRealArray"),
];

impl Config {
    fn godot_items() -> HashMap<String, String> {
        let mut godot_items = HashMap::new();
        for class in GODOT_CLASSES {
            godot_items.insert(
                class.to_string(),
                format!(
                    "https://docs.godotengine.org/en/stable/classes/class_{}.html",
                    class.to_lowercase()
                ),
            );
        }

        for (name, links_to, section) in GODOT_CONSTANTS {
            let mut link = format!(
                "https://docs.godotengine.org/en/stable/classes/{}.html",
                links_to
            );
            if !section.is_empty() {
                link.push('#');
                link.push_str(section)
            }
            godot_items.insert(name.to_string(), link);
        }
        godot_items
    }

    fn rust_to_godot() -> HashMap<String, String> {
        let mut rust_to_godot = HashMap::new();
        for (rust, godot) in RUST_TO_GODOT {
            rust_to_godot.insert(rust.to_string(), godot.to_string());
        }
        rust_to_godot
    }

    pub fn from_user_config(user_config: UserConfig) -> Self {
        let markdown_options = user_config
            .markdown_options()
            .unwrap_or(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        let overrides = user_config.overrides.unwrap_or(HashMap::new());
        let backends = if let Some(backends) = user_config.backends {
            let mut b = Vec::new();
            for backend in backends {
                match backend.as_str() {
                    "markdown" => {
                        if b.contains(&Backend::Markdown) {
                            log::warn!("Backend 'markdown' is already specified")
                        } else {
                            b.push(Backend::Markdown)
                        }
                    }
                    "html" => {
                        if b.contains(&Backend::Html) {
                            log::warn!("Backend 'html' is already specified")
                        } else {
                            b.push(Backend::Html)
                        }
                    }
                    _ => {
                        log::error!("unknown backend: {}", backend);
                    }
                }
            }
            b
        } else {
            vec![Backend::Markdown]
        };

        Self {
            godot_items: Self::godot_items(),
            rust_to_godot: Self::rust_to_godot(),
            overrides,
            markdown_options,
            backends,
        }
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
        let temporary;
        let base = if let Ok(link) = syn::parse_str::<syn::Path>(link) {
            match link.segments.last() {
                None => return None,
                Some(base) => {
                    temporary = base.ident.to_string();
                    &temporary
                }
            }
        } else {
            link
        };

        if let Some(path) = self.overrides.get(base).cloned() {
            Some(path)
        } else {
            let base = match self.rust_to_godot.get(base) {
                Some(base) => base.as_str(),
                None => base,
            };
            if let Some(path) = self.godot_items.get(base).cloned() {
                Some(path)
            } else {
                None
            }
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
