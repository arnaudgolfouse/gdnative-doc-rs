//! Facilities related to link resolution.

use crate::{
    config::ConfigFile,
    documentation::{self, Documentation, Type},
    GodotVersion,
};
use pulldown_cmark::{CowStr, Event, Tag};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Information to resolve links.
pub struct Resolver {
    /// Link to godot items' documentation.
    ///
    /// Contains the link to godot classes, but also `true`, `INF`, `Err`...
    pub godot_items: HashMap<String, String>,
    /// Mapping from Rust to Godot types.
    pub rust_to_godot: HashMap<String, String>,
    /// User-defined overrides.
    ///
    /// These are defined in the [toml configuration file](crate::ConfigFile).
    pub url_overrides: HashMap<String, String>,
    /// User-defined Rust to Godot mapping.
    ///
    /// These are defined in the [toml configuration file](crate::ConfigFile).
    pub rename_classes: HashMap<String, String>,
}

/// Url for the (stable) godot documentation
const GODOT_DOCUMENTATION_URL_3_2: &str = "https://docs.godotengine.org/en/3.2/classes";
const GODOT_DOCUMENTATION_URL_3_3: &str = "https://docs.godotengine.org/en/3.3/classes";
const GODOT_DOCUMENTATION_URL_3_4: &str = "https://docs.godotengine.org/en/3.4/classes";
const GODOT_DOCUMENTATION_URL_3_5: &str = "https://docs.godotengine.org/en/3.5/classes";

/// List of godot 3.2 classes, like `Array`, `int`, `Transform2D`...
const GODOT_CLASSES_3_2: &[&str] = &include!("../../fetch_godot_classes/godot_classes-3.2.txt");
/// List of godot 3.3 classes, like `Array`, `int`, `Transform2D`...
const GODOT_CLASSES_3_3: &[&str] = &include!("../../fetch_godot_classes/godot_classes-3.3.txt");
/// List of godot 3.4 classes, like `Array`, `int`, `Transform2D`...
const GODOT_CLASSES_3_4: &[&str] = &include!("../../fetch_godot_classes/godot_classes-3.4.txt");
/// List of godot 3.5 classes, like `Array`, `int`, `Transform2D`...
const GODOT_CLASSES_3_5: &[&str] = &include!("../../fetch_godot_classes/godot_classes-3.5.txt");

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
    ("OK", "class_@globalscope", "enum-globalscope-error"),
];

/// Mapping from Rust to Godot types.
const RUST_TO_GODOT: &[(&str, &str)] = &[
    ("i32", "int"),
    ("i64", "int"),
    ("f32", "float"),
    ("f64", "float"),
    ("GodotString", "String"),
    ("VariantArray", "Array"),
    ("Int32Array", "PoolIntArray"),
    ("Float32Array", "PoolRealArray"),
];

impl Resolver {
    pub(crate) fn new(godot_version: GodotVersion) -> Self {
        Self {
            godot_items: Self::godot_items(godot_version),
            rust_to_godot: Self::rust_to_godot(),
            url_overrides: HashMap::new(),
            rename_classes: HashMap::new(),
        }
    }

    fn godot_items(godot_version: GodotVersion) -> HashMap<String, String> {
        let mut godot_items = HashMap::new();
        let classes = match godot_version {
            GodotVersion::Version32 => GODOT_CLASSES_3_2,
            GodotVersion::Version33 => GODOT_CLASSES_3_3,
            GodotVersion::Version34 => GODOT_CLASSES_3_4,
            GodotVersion::Version35 => GODOT_CLASSES_3_5,
        };
        let documentation_url = match godot_version {
            GodotVersion::Version32 => GODOT_DOCUMENTATION_URL_3_2,
            GodotVersion::Version33 => GODOT_DOCUMENTATION_URL_3_3,
            GodotVersion::Version34 => GODOT_DOCUMENTATION_URL_3_4,
            GodotVersion::Version35 => GODOT_DOCUMENTATION_URL_3_5,
        };
        for class in classes {
            godot_items.insert(
                class.to_string(),
                format!("{}/class_{}.html", documentation_url, class.to_lowercase()),
            );
        }

        for (name, links_to, section) in GODOT_CONSTANTS {
            let mut link = format!("{}/{}.html", documentation_url, links_to);
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

    pub(crate) fn apply_user_config(&mut self, user_config: &ConfigFile) {
        self.url_overrides = user_config.url_overrides.clone().unwrap_or_default();
        self.rename_classes = user_config.rename_classes.clone().unwrap_or_default();
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
            for property in &mut class.properties {
                match &mut property.typ {
                    Type::Option(name) | Type::Named(name) => replace(name),
                    Type::Unit => {}
                }
            }
            replace(&mut name);
            replace(&mut class.inherit);
            renamed_classes.insert(name, class);
        }
        documentation.classes = renamed_classes;
    }

    /// Resolve a name to the location it must link to.
    ///
    /// `link` must already have been stripped off the enclosing \`.
    pub fn resolve(&self, link: &str) -> Option<&str> {
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

    /// Increase the header count, and resolve link destinations
    pub(super) fn resolve_event(&self, event: &mut Event) {
        use pulldown_cmark::HeadingLevel;
        fn increase_heading_level(level: HeadingLevel) -> HeadingLevel {
            match level {
                HeadingLevel::H1 => HeadingLevel::H4,
                HeadingLevel::H2 => HeadingLevel::H5,
                HeadingLevel::H3 | HeadingLevel::H4 | HeadingLevel::H5 | HeadingLevel::H6 => {
                    HeadingLevel::H6
                }
            }
        }
        match event {
            Event::Start(Tag::Link(_, dest, _)) | Event::End(Tag::Link(_, dest, _)) => {
                if let Some(new_dest) = self.resolve(dest) {
                    *dest = new_dest.to_string().into()
                }
            }
            Event::Start(Tag::Heading(n, _, _)) | Event::End(Tag::Heading(n, _, _)) => {
                *n = increase_heading_level(*n);
            }
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
                CowStr::Borrowed(return_link),
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
