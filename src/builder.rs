use crate::{
    backend::{self, BuiltinBackend, Callbacks, Resolver},
    documentation::Documentation,
    ConfigFile, Error, Result,
};
use std::{fs, path::PathBuf};

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
///
/// For each format you want to generate, you must add a backend via [`add_backend`]
/// or [`add_backend_with_callbacks`].
///
/// [`add_backend`]: Builder::add_backend
/// [`add_backend_with_callbacks`]: Builder::add_backend_with_callbacks
pub struct Builder {
    resolver: Resolver,
    /// List of backends with their output directory
    backends: Vec<(Box<dyn Callbacks>, PathBuf)>,
    /// Configuration file
    user_config: Option<ConfigFile>,
    /// Used to disambiguate which crate to use.
    package: Option<Package>,
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
        }
    }

    /// Set user configuration options.
    ///
    /// See the `ConfigFile` documentation for information about the configuration file format.
    pub fn user_config(mut self, config: ConfigFile) -> Self {
        self.user_config = Some(config);
        self
    }

    /// Specify the crate to document.
    ///
    /// The `Builder` will try to automatically determine which crate you want to document. If this fails or you are not satisfied with its guess, you can use this function to manually specify the crate you want to refer to.
    ///
    /// This can be either the [name](Package::Name) of the crate, or directly the
    /// [path of the root file](Package::Root).
    ///
    /// Only one crate can be documented at a time: if this function is called
    /// multiple times, the last call will prevail.
    ///
    /// # Example
    /// ```
    /// # use gdnative_doc::{Builder, Package};
    /// let builder = Builder::new().package(Package::Name("my-gdnative-crate".to_string()));
    /// ```
    pub fn package(mut self, package: Package) -> Self {
        self.package = Some(package);
        self
    }

    /// Add a new builtin backend to the builder.
    ///
    /// # Example
    /// ```
    /// # use gdnative_doc::{Builder, backend::BuiltinBackend};
    /// # use std::path::PathBuf;
    /// let builder = Builder::new().add_backend(BuiltinBackend::Markdown, PathBuf::from("doc"));
    /// ```
    pub fn add_backend(mut self, backend: BuiltinBackend, output_dir: PathBuf) -> Self {
        let callbacks: Box<dyn Callbacks> = match &backend {
            BuiltinBackend::Markdown => Box::new(backend::MarkdownCallbacks::default()),
            BuiltinBackend::Html => Box::new(backend::HtmlCallbacks::default()),
            BuiltinBackend::Gut => Box::new(backend::GutCallbacks::default()),
        };
        self.backends.push((callbacks, output_dir));
        self
    }

    /// Add a new backend to the builder, with custom callbacks encoding functions.
    ///
    /// See the [`backend`](crate::backend) module for how to implement your own
    /// backend.
    pub fn add_backend_with_callbacks(
        mut self,
        callbacks: Box<dyn Callbacks>,
        output_dir: PathBuf,
    ) -> Self {
        self.backends.push((callbacks, output_dir));
        self
    }

    /// Build the documentation.
    ///
    /// This will generate the documentation for each
    /// [specified backend](Self::add_backend), creating the ouput directories if
    /// needed.
    #[allow(clippy::or_fun_call)]
    pub fn build(mut self) -> Result<()> {
        let (markdown_options, opening_comment) = if let Some(user_config) = self.user_config.take()
        {
            let opening_comment = user_config.opening_comment.unwrap_or(true);
            let markdown_options = user_config
                .markdown_options()
                .unwrap_or(pulldown_cmark::Options::empty());
            self.resolver.apply_user_config(user_config);
            (markdown_options, opening_comment)
        } else {
            (pulldown_cmark::Options::empty(), true)
        };

        let documentation = self.build_documentation()?;
        for (mut callbacks, output_dir) in self.backends {
            let generator = backend::Generator::new(
                &self.resolver,
                &documentation,
                markdown_options,
                opening_comment,
            );

            let files = callbacks.generate_files(generator);

            if let Err(err) = fs::create_dir_all(&output_dir) {
                return Err(Error::Io(output_dir, err));
            }
            for (file_name, content) in files {
                let out_file = output_dir.join(file_name);
                if let Err(err) = fs::write(&out_file, content) {
                    return Err(Error::Io(out_file, err));
                }
            }
        }

        Ok(())
    }

    /// Build documentation from a root file.
    ///
    /// The root file is either stored in `self`, or automatically discovered using
    /// [`find_root_file`].
    fn build_documentation(&mut self) -> Result<Documentation> {
        log::debug!("building documentation");
        let (name, root_file) = match self.package.take() {
            Some(Package::Root(root_file)) => ("_".to_string(), root_file),
            Some(Package::Name(name)) => find_root_file(Some(&name))?,
            None => find_root_file(None)?,
        };

        let mut documentation = Documentation::from_root_file(name, root_file)?;
        self.resolver.rename_classes(&mut documentation);
        Ok(documentation)
    }
}

/// Returns the name of the crate and the root file.
fn find_root_file(package_name: Option<&str>) -> Result<(String, PathBuf)> {
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    let mut root_files = Vec::new();
    for package in metadata.packages {
        if metadata.workspace_members.contains(&package.id) {
            if let Some(target) = package
                .targets
                .into_iter()
                .find(|target| target.kind.iter().any(|kind| kind == "cdylib"))
            {
                root_files.push((package.name, target.src_path.into()))
            }
        }
    }

    if let Some(package_name) = package_name {
        match root_files
            .into_iter()
            .find(|(name, _)| name == package_name)
        {
            Some((_, root_file)) => Ok((package_name.to_string(), root_file)),
            None => Err(Error::NoMatchingCrate(package_name.to_string())),
        }
    } else {
        if root_files.len() > 1 {
            return Err(Error::MultipleCandidateCrate(
                root_files.into_iter().map(|(name, _)| name).collect(),
            ));
        }
        if let Some((name, root_file)) = root_files.pop() {
            Ok((name, root_file))
        } else {
            Err(Error::NoCandidateCrate)
        }
    }
}
