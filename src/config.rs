//! User configuration settings.

use crate::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// Structure that holds user configuration settings.
///
/// Should be obtained via a `toml` [configuration file](ConfigFile::read_from).
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ConfigFile {
    /// List of items for which the linking url should be overriden.
    pub url_overrides: Option<HashMap<String, String>>,
    /// Renaming of types when going from Rust to Godot.
    ///
    /// This is useful because GDNative allows defining a `script_class_name` in the
    /// `.gdns` file.
    pub rename_classes: Option<HashMap<String, String>>,
    /// Optional markdown options.
    ///
    /// # Valid options
    /// - FOOTNOTES
    /// - SMART_PUNCTUATION
    /// - STRIKETHROUGH
    /// - TABLES
    /// - TASKLISTS
    ///
    /// # Default
    /// No option enabled.
    pub markdown_options: Option<Vec<String>>,
}

impl ConfigFile {
    /// Read `ConfigFile` from the given `toml` configuration file.
    pub fn read_from(config: &str) -> Result<Self> {
        Ok(toml::from_str(config)?)
    }

    /// Convert the `String` list of options to `pulldown_cmark::Options`, logging
    /// warnings on unrecognized options.
    pub(crate) fn markdown_options(&self) -> Option<pulldown_cmark::Options> {
        use pulldown_cmark::Options;
        if let Some(options) = &self.markdown_options {
            let mut markdown_options = Options::empty();
            for option in options {
                match option.as_str() {
                    "FOOTNOTES" => markdown_options.insert(Options::ENABLE_FOOTNOTES),
                    "SMART_PUNCTUATION" => {
                        markdown_options.insert(Options::ENABLE_SMART_PUNCTUATION)
                    }
                    "STRIKETHROUGH" => markdown_options.insert(Options::ENABLE_STRIKETHROUGH),
                    "TABLES" => markdown_options.insert(Options::ENABLE_TABLES),
                    "TASKLISTS" => markdown_options.insert(Options::ENABLE_TASKLISTS),
                    _ => log::warn!("unknown markdown option: {}", option),
                }
            }
            Some(markdown_options)
        } else {
            None
        }
    }
}
