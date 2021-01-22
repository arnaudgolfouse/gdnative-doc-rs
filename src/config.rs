//! User configuration settings.

use crate::Result;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

/// Structure that holds user configuration settings.
///
/// Should be obtained via a `toml` [configuration file](UserConfig::read_from).
#[derive(Deserialize)]
pub struct UserConfig {
    /// List of enabled backends, with their associated output directory.
    ///
    /// # Valid backends
    /// - markdown
    /// - html
    pub backends: HashMap<String, PathBuf>,
    /// Root file of the crate.
    ///
    /// # Default
    /// Determined via `cargo` (TODO)
    pub root_file: Option<PathBuf>,
    /// List of items for which the linking url should be overriden.
    pub url_overrides: Option<HashMap<String, String>>,
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

impl UserConfig {
    /// Read `UserConfig` from the given `toml` configuration file.
    pub fn read_from(config: &str) -> Result<Self> {
        Ok(toml::from_str(config)?)
    }

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
