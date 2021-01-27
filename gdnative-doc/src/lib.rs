//! Generating documentation for gdnative.
//!
//! # Example
//! ```rust,no_run
//! use gdnative_doc::{Backend, Builder, Config, LevelFilter};
//!
//! Builder::new(Config::default())
//!     .add_backend(Backend::Markdown {
//!         output_dir: "doc".into(),
//!     })
//!     .add_backend(Backend::Gut {
//!         output_dir: "addons/gut".into(),
//!     })
//!     .log_level(LevelFilter::Info)
//!     .build()
//!     .unwrap();
//! ```

mod backend;
mod builder;
mod config;
pub mod documentation;
mod files;

pub use backend::{Backend, Callbacks, Config};
pub use builder::Builder;
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
    /// When trying to determine a root file, multiple candidates were found.
    #[error(
        r"Multiple crates were found with a 'cdylib' target: {0:?}
Please select the one you want via either:
  - The '-p' flag on the command line
  - The `package` method of `Builder`
"
    )]
    MultipleCandidateCrate(Vec<String>),
    /// When trying to determine a root file, no candidate was found.
    #[error("No crate was found with a 'cdylib' target")]
    NoCandidateCrate,
}

pub type Result<T> = std::result::Result<T, Error>;
