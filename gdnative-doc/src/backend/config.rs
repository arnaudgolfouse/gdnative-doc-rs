use super::Backend;
use crate::{
    config::ConfigFile,
    documentation::{self, Documentation, Type},
};
use pulldown_cmark::{CowStr, Event, Tag};
use std::collections::HashMap;

/// Configuration options for [Builder](crate::Builder).
///
/// It can be built within code, or generated via an instance of
/// [`ConfigFile`].
pub struct Config {
    /// Link to godot items' documentation
    ///
    /// Contains the link to godot classes, but also `true`, `INF`, `Err`...
    pub(crate) godot_items: HashMap<String, String>,
    /// Mapping from Rust to Godot types
    pub(crate) rust_to_godot: HashMap<String, String>,
    /// User-defined overrides
    pub(crate) url_overrides: HashMap<String, String>,
    /// User-defined Rust to Godot mapping
    pub(crate) rename_classes: HashMap<String, String>,
    /// Markdown options
    pub(crate) markdown_options: pulldown_cmark::Options,
    /// Enabled backends
    pub(crate) backends: Vec<Backend>,
}

/// Url for the godot documentation
const GODOT_DOCUMENTATION_URL: &str = "https://docs.godotengine.org/en/stable/classes";

/// List of godot classes, like `Array`, `int`, `Transform2D`...
const GODOT_CLASSES: &[&str] = &include!("../../../fetch_godot_classes/godot_classes");

/// List of some godot constants and information about where they sould link to.
///
/// link for `<constant.0>`: `<godot_doc_url>/<constant.1>.html#<constant.2>`
const GODOT_CONSTANTS: &[(&str, &str, &str)] = &[
    ("true", "class_bool", ""),
    ("false", "class_bool", ""),
    ("PI", "class_@gdscript", "constants"),
    ("TAU", "class_@gdscript", "constants"),
    ("INF", "class_@gdscript", "constants"),
    ("NAN", "class_@gdscript", "constants"),
    ("FAILED", "class_@globalscope", "enum-globalscope-error"),
    ("OK", "class_@globalscope", "class-globalscope-error"),
];

/// Mapping from Rust to Godot types.
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
        Self {
            godot_items: Self::godot_items(),
            rust_to_godot: Self::rust_to_godot(),
            url_overrides: HashMap::new(),
            rename_classes: HashMap::new(),
            markdown_options: pulldown_cmark::Options::empty(),
            backends: Vec::new(),
        }
    }
}

impl Config {
    fn godot_items() -> HashMap<String, String> {
        let mut godot_items = HashMap::new();
        for class in GODOT_CLASSES {
            godot_items.insert(
                class.to_string(),
                format!(
                    "{}/class_{}.html",
                    GODOT_DOCUMENTATION_URL,
                    class.to_lowercase()
                ),
            );
        }

        for (name, links_to, section) in GODOT_CONSTANTS {
            let mut link = format!("{}/{}.html", GODOT_DOCUMENTATION_URL, links_to);
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

    pub fn from_user_config(user_config: ConfigFile) -> Self {
        let markdown_options = user_config
            .markdown_options()
            .unwrap_or(pulldown_cmark::Options::empty());
        let url_overrides = user_config.url_overrides.unwrap_or_default();
        let name_overrides = user_config.rename_classes.unwrap_or_default();

        Self {
            godot_items: Self::godot_items(),
            rust_to_godot: Self::rust_to_godot(),
            url_overrides,
            rename_classes: name_overrides,
            markdown_options,
            backends: Vec::new(),
        }
    }

    /// Convert all type names from Rust to Godot.
    ///
    /// This will convert `i32` to `int`, `Int32Array` to `PoolIntArray`...
    ///
    /// See [`ConfigFile::rename_classes`] for user-defined renaming.
    pub(crate) fn rename_classes(&self, documentation: &mut Documentation) {
        let replace = |name: &mut String| {
            if let Some(rename) = self.rename_classes.get(name) {
                *name = rename.clone();
            } else if let Some(rename) = self.rust_to_godot.get(name) {
                *name = rename.clone();
            }
        };

        let mut renamed_classes = HashMap::new();
        let classes = std::mem::take(&mut documentation.classes);
        for (mut name, mut class) in classes {
            for method in &mut class.methods {
                for (_, typ, _) in &mut method.parameters {
                    match typ {
                        documentation::Type::Option(name) | documentation::Type::Named(name) => {
                            replace(name)
                        }
                        documentation::Type::Unit => {}
                    }
                }
                match &mut method.return_type {
                    documentation::Type::Option(name) | documentation::Type::Named(name) => {
                        replace(name)
                    }
                    documentation::Type::Unit => {}
                }
            }
            replace(&mut name);
            replace(&mut class.inherit);
            renamed_classes.insert(name, class);
        }
        documentation.classes = renamed_classes;
    }

    /// Resolve a name to the class it must link to.
    ///
    /// `link` must already have been stripped off the enclosing \`.
    pub(super) fn resolve(&self, link: &str) -> Option<&str> {
        if let Some(link) = self.url_overrides.get(link) {
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

        if let Some(path) = self.url_overrides.get(base) {
            Some(path)
        } else {
            let base = match self.rust_to_godot.get(base) {
                Some(base) => base.as_str(),
                None => base,
            };
            if let Some(path) = self.godot_items.get(base) {
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
                    Some(new_dest) => *dest = new_dest.to_string().into(),
                    None => {}
                }
            }
            Event::Start(Tag::Heading(n)) | Event::End(Tag::Heading(n)) => *n += 3,
            _ => {}
        }
    }

    pub(super) fn encode_type<'b>(&'b self, typ: &'b Type) -> Vec<Event<'b>> {
        let (type_name, optional) = match typ {
            Type::Option(typ) => (typ.as_str(), true),
            Type::Named(typ) => (typ.as_str(), false),
            Type::Unit => ("void", false),
        };
        let mut events = match self.resolve(type_name).map(|return_link| {
            Tag::Link(
                pulldown_cmark::LinkType::Shortcut,
                CowStr::Borrowed(&return_link),
                CowStr::Borrowed(""),
            )
        }) {
            Some(link) => {
                vec![
                    Event::Start(link.clone()),
                    Event::Text(CowStr::Borrowed(type_name)),
                    Event::End(link),
                ]
            }
            None => {
                vec![Event::Text(CowStr::Borrowed(type_name))]
            }
        };
        if optional {
            events.push(Event::Text(CowStr::Borrowed(" (opt)")))
        }
        events
    }
}
