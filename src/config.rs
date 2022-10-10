//! User configuration settings.

use crate::Error;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::PathBuf};

/// Structure that holds user configuration settings.
///
/// Should be obtained via a `toml` configuration file.
///
/// # Example
/// ```
/// # use gdnative_doc::{Error, ConfigFile};
/// # fn main() -> Result<(), Error> {
/// const CONFIG_FILE_CONTENT: &str = r#"
/// rename_classes = { RustName = "GDScriptName" }
/// markdown_options = ["STRIKETHROUGH", "TABLES"]
/// "#;
///
/// let config_file = ConfigFile::load_from_str(CONFIG_FILE_CONTENT)?;
/// assert!(config_file.url_overrides.is_none());
/// assert_eq!(config_file.rename_classes.unwrap()["RustName"], "GDScriptName");
/// assert_eq!(
///     config_file.markdown_options.unwrap(),
///     &["STRIKETHROUGH".to_string(), "TABLES".to_string()]
/// );
/// # Ok(()) }
/// ```
///
/// Note that if you are reading the configuration file from an on-disk file, you
/// should prefer [`load_from_path`](ConfigFile::load_from_path).
// Note: any update to this structure should be documented in
// configuration_file-format.md.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
pub struct ConfigFile {
    /// Godot version used.
    ///
    /// Valid fields are "3.2", "3.3", "3.4" and "3.5".
    ///
    /// Defaults to "3.5".
    pub godot_version: Option<String>,
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
    /// Control whether or not to include a comment in the generated files.
    ///
    /// The comment includes information such that the file was automatically
    /// generated, the name of the source file it originated from...
    ///
    /// # Default
    /// `true`
    pub opening_comment: Option<bool>,
}

impl ConfigFile {
    /// Load the config file from the given `path`.
    pub fn load_from_path(path: PathBuf) -> Result<Self, Error> {
        log::debug!("loading user config at {:?}", path);
        Ok(toml::from_str(&match fs::read_to_string(&path) {
            Ok(config) => config,
            Err(err) => return Err(Error::Io(path, err)),
        })?)
    }

    /// Load the config file from the given `config` string.
    pub fn load_from_str(config: &str) -> Result<Self, Error> {
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
