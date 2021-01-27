use crate::{
    backend, documentation::Documentation, files::Package, Backend, Callbacks, Config, ConfigFile,
    Error, LevelFilter, Result,
};
use std::{fs, path::PathBuf};

/// A builder for generating godot documentation in various formats.
pub struct Builder {
    config: Config,
    backends: Vec<(Backend, Box<dyn Callbacks>)>,
    log_level: LevelFilter,
    root_file: Option<PathBuf>,
}

impl Builder {
    /// Load a configuration from the given `path`.
    pub fn from_user_config(path: PathBuf) -> Result<Self> {
        let config_file = match fs::read_to_string(&path) {
            Ok(config) => config,
            Err(err) => return Err(Error::Io(path, err)),
        };
        let config = ConfigFile::read_from(&config_file)?;

        let root_file = config.root_file.clone();

        let backend_config = Config::from_user_config(config);
        let backends = backend_config.backends.clone();
        let mut result = Self::new(backend_config);
        result.root_file = root_file;
        for backend in backends {
            result = result.add_backend(backend);
        }
        Ok(result)
    }

    pub fn new(config: Config) -> Self {
        Self {
            config,
            backends: Vec::new(),
            log_level: LevelFilter::Info,
            root_file: None,
        }
    }

    /// Set the logging level.
    ///
    /// Defaults to [`LevelFilter::Info`].
    pub fn log_level(mut self, log_level: LevelFilter) -> Self {
        self.log_level = log_level;
        self
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

    /// Add a new backend to the builder, with custom callbacks encoding functions.
    pub fn add_backend_with_callbacks(
        mut self,
        backend: Backend,
        callbacks: Box<dyn Callbacks>,
    ) -> Self {
        self.backends.push((backend, callbacks));
        self
    }

    pub fn build(mut self) -> Result<()> {
        init_logger(self.log_level);
        let documentation = self.build_documentation()?;
        for (backend, callbacks) in self.backends {
            let extension = callbacks.extension();
            let mut generator = backend::Generator::new(&self.config, &documentation, callbacks);
            let files = generator.generate_files();
            let root_file = generator.generate_root_file(extension);

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

    /// Build documentation from a root file.
    ///
    /// The root file is either stored in `self`, or autmatically discovered using
    /// [`find_root_file`].
    fn build_documentation(&mut self) -> Result<Documentation> {
        let root_file = if let Some(file) = self.root_file.take() {
            file
        } else {
            find_root_file()?
        };
        let package = Package::from_root_file(root_file)?;

        let mut documentation = Documentation::new();
        for (module_id, module) in package.modules {
            let root_module = if module_id == package.root_module {
                match module.attributes.as_ref() {
                    Some(attrs) => Some(attrs.as_slice()),
                    None => None,
                }
            } else {
                None
            };
            documentation.parse_from_module(&module.items, root_module)?;
        }
        self.config.rename_classes(&mut documentation);
        Ok(documentation)
    }
}

/// Initialize the logger with the specified logging level.
fn init_logger(level: LevelFilter) {
    simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
    )
    .unwrap();
}

fn find_root_file() -> Result<PathBuf> {
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    let mut root_files = Vec::new();
    for package in metadata.packages {
        if metadata.workspace_members.contains(&package.id) {
            if let Some(target) = package
                .targets
                .into_iter()
                .find(|target| target.kind.iter().any(|kind| kind == "cdylib"))
            {
                root_files.push((package.name, target.src_path))
            }
        }
    }

    if root_files.len() > 1 {
        return Err(Error::MultipleCandidateCrate(
            root_files.into_iter().map(|(name, _)| name).collect(),
        ));
    }
    if let Some((_, root_file)) = root_files.pop() {
        Ok(root_file)
    } else {
        Err(Error::NoCandidateCrate)
    }
}
