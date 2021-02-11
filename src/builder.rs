use crate::{
    backend, documentation::Documentation, Backend, Callbacks, ConfigFile, Error, Resolver, Result,
};
use std::{fs, path::PathBuf};

const PRISM_CSS: (&str, &str) = ("prism.css", include_str!("../html/prism.css"));
const PRISM_JS: (&str, &str) = ("prism.js", include_str!("../html/prism.js"));
const STYLE_CSS: (&str, &str) = ("style.css", include_str!("../html/style.css"));

/// Used to specify a crate in [`Builder::package`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Package {
    /// Specify the crate by name
    Name(String),
    /// Specify the crate by the path of its root file
    Root(PathBuf),
}

#[derive(Debug)]
/// A builder for generating godot documentation in various formats.
pub struct Builder {
    resolver: Resolver,
    backends: Vec<(Backend, Box<dyn Callbacks>)>,
    /// Configuration file
    user_config: Option<PathBuf>,
    /// Used to disambiguate which crate to use.
    package: Option<Package>,
    /// Markdown options
    markdown_options: pulldown_cmark::Options,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create a default `Builder` with no backends.
    pub fn new() -> Self {
        Self {
            resolver: Resolver::default(),
            backends: Vec::new(),
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
        if let Some(path) = self.user_config.take() {
            self.load_user_config(path)?
        }

        let documentation = self.build_documentation()?;
        for (backend, callbacks) in self.backends {
            log::debug!("generating documentation for backend {:?}", backend);
            let extension = callbacks.extension();
            let mut generator = backend::Generator::new(
                &self.resolver,
                &documentation,
                callbacks,
                self.markdown_options,
            );
            let files = generator.generate_files();
            let root_file = generator.generate_root_file(extension);

            match &backend {
                Backend::Markdown { output_dir }
                | Backend::Html { output_dir }
                | Backend::Gut { output_dir } => {
                    if let Err(err) = fs::create_dir_all(&output_dir) {
                        return Err(Error::Io(output_dir.clone(), err));
                    }
                    if !matches!(&backend, Backend::Gut { .. }) {
                        let out_file = output_dir.join("index").with_extension(extension);
                        if let Err(err) = fs::write(&out_file, root_file) {
                            return Err(Error::Io(out_file, err));
                        }
                    }
                    if matches!(&backend, Backend::Html { .. }) {
                        for (file_name, file_content) in &[PRISM_CSS, PRISM_JS, STYLE_CSS] {
                            let file = output_dir.join(file_name);
                            if let Err(err) = fs::write(&file, file_content) {
                                return Err(Error::Io(file, err));
                            }
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

    #[allow(clippy::or_fun_call)]
    fn load_user_config(&mut self, path: PathBuf) -> Result<()> {
        log::debug!("loading user config at {:?}", path);
        let user_config = ConfigFile::read_from(&match fs::read_to_string(&path) {
            Ok(config) => config,
            Err(err) => return Err(Error::Io(path, err)),
        })?;

        self.markdown_options = user_config
            .markdown_options()
            .unwrap_or(pulldown_cmark::Options::empty());
        self.resolver.apply_user_config(user_config);
        Ok(())
    }

    /// Build documentation from a root file.
    ///
    /// The root file is either stored in `self`, or autmatically discovered using
    /// [`find_root_file`].
    fn build_documentation(&mut self) -> Result<Documentation> {
        log::debug!("building documentation");
        let root_file = match self.package.take() {
            Some(Package::Root(root_file)) => root_file,
            Some(Package::Name(name)) => find_root_file(Some(&name))?,
            None => find_root_file(None)?,
        };

        let mut documentation = Documentation::from_root_file(root_file)?;
        self.resolver.rename_classes(&mut documentation);
        Ok(documentation)
        /*let package = CrateTree::from_root_file(root_file)?;

        let mut documentation = Documentation::new();
        for (module_id, module) in package.modules {
            documentation.parse_from_module(&module, module_id == package.root_module);
        }
        self.resolver.rename_classes(&mut documentation);
        Ok(documentation)*/
    }
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
