//! Generating documentation for gdnative.
//!
//! # Example
//! ```rust,no_run
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use gdnative_doc::{backend::BuiltinBackend, Builder, LevelFilter, init_logger};
//! use std::path::PathBuf;
//!
//! init_logger(LevelFilter::Info)?;
//! Builder::new()
//!     .add_backend(BuiltinBackend::Markdown, PathBuf::from("doc"))
//!     .add_backend(BuiltinBackend::Gut, PathBuf::from("addons/gut"))
//!     .build()?;
//! # Ok(()) }
//! ```

pub mod backend;
mod builder;
mod config;
pub mod documentation;

pub use builder::{Builder, Package};
pub use config::ConfigFile;
pub use simplelog::LevelFilter;

/// Type of errors emitted by this library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// [`toml`] parsing error.
    #[error("{0}")]
    Toml(#[from] toml::de::Error),
    /// IO error (usually caused by non-existent or non-readable files).
    #[error("Error at {0}: {1}")]
    Io(std::path::PathBuf, std::io::Error),
    /// [`syn`] parsing error.
    #[error("{0}")]
    Syn(#[from] syn::Error),
    /// Error while running `cargo metadata`.
    #[error("{0}")]
    Metadata(#[from] cargo_metadata::Error),
    #[error("No crate matched the name '{0}'")]
    /// When trying to determine a root file, no suitable crate matched the expected name.
    NoMatchingCrate(String),
    /// When trying to determine a root file, multiple suitable candidates were found.
    #[error(
        r"Multiple crates were found with a 'cdylib' target: {0:?}
Please select the one you want via either:
  - The '-p' flag on the command line
  - The `package` method of `Builder`
"
    )]
    MultipleCandidateCrate(Vec<String>),
    /// When trying to determine a root file, no suitable candidate was found.
    #[error("No crate was found with a 'cdylib' target")]
    NoCandidateCrate,
    #[error("Logger initialization failed: {0}")]
    InitLogger(#[from] log::SetLoggerError),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Initialize the logger with the specified logging level.
///
/// The default recommended level is [`LevelFilter::Info`].
pub fn init_logger(level: LevelFilter) -> Result<()> {
    simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
    )?;
    Ok(())
}
