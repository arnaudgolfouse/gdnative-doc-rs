use std::{collections::HashMap, fmt, fs, io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error at {0}: {1}")]
    Io(PathBuf, io::Error),
    #[error("{0}")]
    Syn(#[from] syn::Error),
}

pub type ModuleId = u32;

pub struct Module {
    pub path: PathBuf,
    pub internal_path: Vec<String>,
    pub visibility: syn::Visibility,
    pub submodules: Vec<ModuleId>,
    pub parent: ModuleId,
    pub items: Vec<syn::Item>,
    pub attributes: Option<Vec<syn::Attribute>>,
}

#[derive(Debug)]
pub struct Package {
    pub root_module: ModuleId,
    pub files_to_ids: HashMap<PathBuf, ModuleId>,
    pub modules: HashMap<ModuleId, Module>,
}

impl Package {
    pub fn from_root_file(path: PathBuf) -> Result<Self, Error> {
        let mut builder = PackageBuilder::default();
        let file = match fs::read_to_string(&path) {
            Ok(content) => syn::parse_file(&content)?,
            Err(io_error) => return Err(Error::Io(path, io_error)),
        };
        let internal_path = vec!["crate".to_string()];
        let root_id = builder.add_module(
            builder.next_module_id,
            path,
            internal_path,
            file.items,
            Some(file.attrs),
            syn::Visibility::Public(syn::VisPublic {
                pub_token: syn::token::Pub::default(),
            }),
        )?;

        Ok(Self {
            root_module: root_id,
            files_to_ids: builder.files_to_ids,
            modules: builder.modules,
        })
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Underscore;
        impl fmt::Debug for Underscore {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "_")
            }
        }
        formatter
            .debug_struct("Module")
            .field("path", &self.path)
            .field("internal_path", &self.internal_path)
            .field("visibility", &Underscore)
            .field("submodules", &self.submodules)
            .field("parent", &self.parent)
            .field("items", &Underscore)
            .field("attributes", &Underscore)
            .finish()
    }
}

#[derive(Default)]
struct PackageBuilder {
    next_module_id: ModuleId,
    files_to_ids: HashMap<PathBuf, ModuleId>,
    modules: HashMap<ModuleId, Module>,
}

impl PackageBuilder {
    fn next_id(&mut self) -> ModuleId {
        let id = self.next_module_id;
        self.next_module_id += 1;
        id
    }

    fn add_module(
        &mut self,
        parent: ModuleId,
        path: PathBuf,
        internal_path: Vec<String>,
        items: Vec<syn::Item>,
        attributes: Option<Vec<syn::Attribute>>,
        visibility: syn::Visibility,
    ) -> Result<ModuleId, Error> {
        let id = self.next_id();
        if internal_path.len() == 1 {
            self.files_to_ids.insert(path.clone(), id);
        }

        let submodules = self.explore_submodules(&items, id, &path, &internal_path)?;

        let module = Module {
            path,
            internal_path,
            submodules,
            parent: parent,
            items: items,
            attributes,
            visibility,
        };
        self.modules.insert(id, module);

        Ok(id)
    }

    fn explore_submodules(
        &mut self,
        items: &[syn::Item],
        parent: ModuleId,
        path: &PathBuf,
        internal_path: &[String],
    ) -> Result<Vec<ModuleId>, Error> {
        let mut submodules = Vec::new();
        for item in items {
            match item {
                syn::Item::Mod(syn::ItemMod {
                    vis,
                    ident,
                    content,
                    ..
                }) => {
                    let mut internal_path = internal_path.to_owned();
                    internal_path.push(ident.to_string());
                    let visibility = vis.clone();
                    let mut path = path.clone();
                    submodules.push(match content {
                        Some((_, items)) => self.add_module(
                            parent,
                            path,
                            internal_path,
                            items.clone(),
                            None,
                            visibility,
                        ),
                        // lib.rs
                        // a.rs
                        // a
                        //   c.rs
                        // b
                        //   mod.rs
                        //   d.rs
                        None => {
                            if let Some(last) = path.file_name() {
                                let last = last.to_str();
                                if last == Some("mod.rs") || last == Some("lib.rs") {
                                    path.pop();
                                } else {
                                    path.set_extension("");
                                }
                            } else {
                                continue;
                            }
                            for module in internal_path.iter().skip(1) {
                                path.push(module);
                            }
                            let path_mod_rs = path.join("mod.rs");
                            if path_mod_rs.exists() {
                                path = path_mod_rs;
                            } else {
                                path.set_extension("rs");
                            }
                            let file = match fs::read_to_string(&path) {
                                Ok(content) => syn::parse_file(&content)?,
                                Err(io_error) => return Err(Error::Io(path, io_error)),
                            };
                            self.add_module(
                                parent,
                                path,
                                vec![ident.to_string()], // TODO: NOPE
                                file.items,
                                Some(file.attrs),
                                visibility,
                            )
                        }
                    }?)
                }
                _ => {}
            }
        }
        Ok(submodules)
    }
}
