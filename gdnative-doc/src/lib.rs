//! Generating documentation for gdnative.

#![warn(clippy::unreachable_pub)]

use backend::Callbacks;
use std::{fs, path::PathBuf};

mod backend;
mod config;
mod documentation;
mod files;

pub use backend::{Backend, Config};
pub use config::ConfigFile;
pub use documentation::Documentation;
pub use simplelog::LevelFilter;

/// Type of errors emitted by this library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// [`toml`] parsing error.
    #[error("{0}")]
    Toml(#[from] toml::de::Error),
    /// IO error (usually caused by non-existent or non-readable files).
    #[error("Error at {0}: {1}")]
    Io(PathBuf, std::io::Error),
    /// [`syn`] parsing error.
    #[error("{0}")]
    Syn(#[from] syn::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Initialize the logger with the specified logging level.
pub fn init_logger(level: LevelFilter) {
    simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
    )
    .unwrap();
}

/// A builder for generating godot documentation in various formats.
pub struct Builder {
    config: backend::Config,
    documentation: documentation::Documentation,
    backends: Vec<(Backend, Box<dyn Callbacks>)>,
}

impl Builder {
    /// Load a configuration from the given `path`.
    pub fn from_user_config(path: PathBuf) -> Result<Self> {
        let config_file = match fs::read_to_string(&path) {
            Ok(config) => config,
            Err(err) => return Err(Error::Io(path, err)),
        };
        let config = config::ConfigFile::read_from(&config_file)?;

        let root_file = match &config.root_file {
            Some(path) => path.clone(),
            None => {
                PathBuf::from("./src/lib.rs") // TODO: determine the root file via cargo
            }
        };
        let package = files::Package::from_root_file(root_file).unwrap();

        let mut documentation = documentation::Documentation::new();
        for (module_id, module) in package.modules {
            let root_module = if module_id == package.root_module {
                match module.attributes.as_ref() {
                    Some(attrs) => Some(attrs.as_slice()),
                    None => None,
                }
            } else {
                None
            };
            documentation
                .parse_from_module(&module.items, root_module)
                .unwrap();
        }

        let backend_config = backend::Config::from_user_config(config);
        let backends = backend_config.backends.clone();
        let mut result = Self::new(backend_config, documentation);
        for backend in backends {
            result = result.add_backend(backend);
        }

        Ok(result)
    }

    pub fn new(config: backend::Config, mut documentation: documentation::Documentation) -> Self {
        config.rename_classes(&mut documentation);
        Self {
            config,
            documentation,
            backends: Vec::new(),
        }
    }

    /// Add a new backend to the builder.
    pub fn add_backend(mut self, backend: Backend) -> Self {
        let callbacks: Box<dyn Callbacks> = match &backend {
            Backend::Markdown { .. } => Box::new(backend::MarkdownCallbacks::default()),
            Backend::Html { .. } => Box::new(backend::HtmlCallbacks::default()),
            Backend::Gut { .. } => Box::new(backend::GutCallbacks::default()),
        };
        self.backends.push((backend, callbacks));
        self
    }

    pub fn build(self) -> Result<()> {
        for (backend, callbacks) in self.backends {
            let extension = backend.extension();
            let mut generator =
                backend::Generator::new(&self.config, &self.documentation, callbacks);
            let files = generator.generate_files();
            let root_file = generator.generate_root_file(backend.extension());

            let backend_is_gut = matches!(&backend, Backend::Gut { .. });

            match &backend {
                Backend::Markdown { output_dir }
                | Backend::Html { output_dir }
                | Backend::Gut { output_dir } => {
                    if let Err(err) = fs::create_dir_all(&output_dir) {
                        return Err(Error::Io(output_dir.clone(), err));
                    }
                    if !backend_is_gut {
                        let out_file = output_dir.join("index").with_extension(extension);
                        if let Err(err) = fs::write(&out_file, root_file) {
                            return Err(Error::Io(out_file, err));
                        }
                    }
                    for (name, content) in files {
                        let out_file = output_dir.join(name).with_extension(extension);
                        if let Err(err) = fs::write(&out_file, content) {
                            return Err(Error::Io(out_file, err));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
