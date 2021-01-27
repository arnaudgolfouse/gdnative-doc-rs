use crate::{
    backend, documentation::Documentation, files::CrateTree, Backend, Callbacks, ConfigFile, Error,
    LevelFilter, Resolver, Result,
};
use std::{fs, path::PathBuf};

/// Used to specify a crate in [`Builder::package`].
#[derive(Clone)]
pub enum Package {
    /// Specify the crate by name
    Name(String),
    /// Specify the crate by the path of its root file
    Root(PathBuf),
}

/// A builder for generating godot documentation in various formats.
pub struct Builder {
    config: Resolver,
    backends: Vec<(Backend, Box<dyn Callbacks>)>,
    log_level: LevelFilter,
    /// Configuration file
    user_config: Option<PathBuf>,
    /// Used to disambiguate which crate to use.
    package: Option<Package>,
    /// Markdown options
    markdown_options: pulldown_cmark::Options,
}

impl Builder {
    /// Create a default `Builder` with no backends.
    pub fn new() -> Self {
        Self {
            config: Resolver::default(),
            backends: Vec::new(),
            log_level: LevelFilter::Info,
            user_config: None,
            package: None,
            markdown_options: pulldown_cmark::Options::empty(),
        }
    }

    /// Set configuration options according to the file at `path`.
    ///
    /// See the [`ConfigFile`] documentation for information about the configuration file format.
    pub fn user_config(mut self, path: PathBuf) -> Self {
        self.user_config = Some(path);
        self
    }

    /// Specify the crate to document.
    ///
    /// This can be either the name of the crate, or directly the path of the root file.
    pub fn package(mut self, package: Package) -> Self {
        self.package = Some(package);
        self
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

    /// Build the documentation.
    ///
    /// This will generate the documentation for each [specified backend](Self::add_backend), creating the
    /// ouput directories if needed.
    pub fn build(mut self) -> Result<()> {
        init_logger(self.log_level);

        if let Some(path) = self.user_config.take() {
            self.load_user_config(path)?
        }

        let documentation = self.build_documentation()?;
        for (backend, callbacks) in self.backends {
            let extension = callbacks.extension();
            let mut generator = backend::Generator::new(
                &self.config,
                &documentation,
                callbacks,
                self.markdown_options,
            );
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

    fn load_user_config(&mut self, path: PathBuf) -> Result<()> {
        let user_config = ConfigFile::read_from(&match fs::read_to_string(&path) {
            Ok(config) => config,
            Err(err) => return Err(Error::Io(path, err)),
        })?;

        self.markdown_options = user_config
            .markdown_options()
            .unwrap_or(pulldown_cmark::Options::empty());
        self.config.apply_user_config(user_config);
        Ok(())
    }

    /// Build documentation from a root file.
    ///
    /// The root file is either stored in `self`, or autmatically discovered using
    /// [`find_root_file`].
    fn build_documentation(&mut self) -> Result<Documentation> {
        let root_file = match self.package.take() {
            Some(Package::Root(root_file)) => root_file,
            Some(Package::Name(name)) => find_root_file(Some(&name))?,
            None => find_root_file(None)?,
        };
        let package = CrateTree::from_root_file(root_file)?;

        let mut documentation = Documentation::new();
        for (module_id, module) in package.modules {
            documentation.parse_from_module(&module, module_id == package.root_module);
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

fn find_root_file(package_name: Option<&str>) -> Result<PathBuf> {
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

    if let Some(package_name) = package_name {
        match root_files
            .into_iter()
            .find(|(name, _)| name == package_name)
        {
            Some((_, root_file)) => Ok(root_file),
            None => Err(Error::NoMatchingCrate(package_name.to_string())),
        }
    } else {
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
}
