use super::{
    attributes_contains, get_docs, get_type_name, read_file_at, Documentation, GdnativeClass, Type,
};
use crate::Error;
use std::{mem, path::PathBuf};
use syn::{
    visit::{self, Visit},
    ItemImpl, ItemMod, ItemStruct,
};

/// Structure that builds the [`Documentation`] by visiting source files.
///
/// It uses the visitor pattern implemented by [`syn::Visit`].
pub(super) struct DocumentationBuilder {
    /// Documentation we are building along the way.
    pub(super) documentation: Documentation,
    /// Current path that is being explored on-disk.
    ///
    /// The second member is `true` if the path is `module/mod.rs` rather that `module.rs`.
    ///
    /// It is also `true` for the root file.
    pub(super) current_file: (PathBuf, bool),
    /// Path of the current module in `current_file`.
    pub(super) current_module: Vec<String>,
    /// Error encountered.
    ///
    /// If it is some, the exploration will stop prematuraly and return it.
    pub(super) error: Option<Error>,
}

impl DocumentationBuilder {
    /// Given the current context and a module name, returns the 2 possible files
    /// corresponding to the module (aka `module/mod.rs` and `module.rs`).
    fn get_module_path(&self, module: &str) -> (PathBuf, PathBuf) {
        let mut path = self.current_file.0.clone();
        if self.current_file.1 {
            path.pop();
        } else {
            path.set_extension("");
        }
        for module in &self.current_module {
            path.push(module);
        }
        path.push(module);
        (path.join("mod.rs"), {
            path.set_extension("rs");
            path
        })
    }

    /// Inner function for Visit::visit_item_impl
    ///
    /// Used for the early return
    #[inline]
    fn visit_item_impl_inner(&mut self, impl_block: &ItemImpl) {
        if attributes_contains(&impl_block.attrs, "methods") {
            let self_type = match get_type_name(&impl_block.self_ty) {
                Some(Type::Named(self_type)) => self_type,
                _ => {
                    log::error!("Unknown type in 'impl' block");
                    return;
                }
            };
            log::trace!("found #[methods] impl block for '{}'", self_type);
            let class = self
                .documentation
                .classes
                .entry(self_type.clone())
                .or_insert(GdnativeClass {
                    name: self_type,
                    inherit: String::new(),
                    documentation: String::new(),
                    properties: Vec::new(),
                    methods: Vec::new(),
                    file: PathBuf::new(),
                });
            for item in &impl_block.items {
                if let syn::ImplItem::Method(method) = item {
                    class.add_method(method, self.current_file.0.clone());
                }
            }
        }
    }
}

impl<'ast> Visit<'ast> for DocumentationBuilder {
    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        if self.error.is_some() {
            return;
        }

        let file_module: ItemMod;

        let (module, old_data) = match &module.content {
            Some(_) => (module, None),
            None => {
                let module_name = module.ident.to_string();
                let (mod_rs, file_rs) = self.get_module_path(&module_name);
                let (path, mod_rs) = if mod_rs.exists() {
                    (mod_rs, true)
                } else {
                    (file_rs, false)
                };
                let file = match read_file_at(&path) {
                    Ok(file) => file,
                    Err(err) => {
                        self.error = Some(err);
                        return;
                    }
                };
                file_module = ItemMod {
                    attrs: file.attrs,
                    vis: module.vis.clone(),
                    mod_token: module.mod_token,
                    ident: module.ident.clone(),
                    content: Some((syn::token::Brace::default(), file.items)),
                    semi: None,
                };
                let old_data = (
                    mem::take(&mut self.current_file),
                    mem::take(&mut self.current_module),
                );
                self.current_file = (path, mod_rs);
                (&file_module, Some(old_data))
            }
        };

        visit::visit_item_mod(self, module);
        if let Some((old_file, old_module)) = old_data {
            self.current_file = old_file;
            self.current_module = old_module;
        }
    }

    fn visit_item_struct(&mut self, strukt: &'ast ItemStruct) {
        if self.error.is_some() {
            return;
        }
        let mut implement_native_class = false;
        let mut inherit = String::from("Reference");
        for attr in &strukt.attrs {
            if let Ok(syn::Meta::List(syn::MetaList { path, nested, .. })) = attr.parse_meta() {
                if path.is_ident("inherit") && nested.len() == 1 {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = nested.first() {
                        // TODO: support path of the form "gdnative::Class"
                        if let Some(class) = path.get_ident() {
                            inherit = class.to_string();
                        }
                    }
                } else if path.is_ident("derive") && nested.len() == 1 {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = nested.first() {
                        if path.is_ident("NativeClass") {
                            implement_native_class = true;
                        }
                    }
                }
            }
        }

        if !implement_native_class {
            return;
        }

        let self_type = strukt.ident.to_string();
        log::trace!("found GDNative class '{self_type}' that inherits '{inherit}'");
        // FIXME: warn or error if we already visited a struct with the same name
        // But be careful ! We *could* have encountered the name in an `impl` block, in which case no warning is warranted.
        let class = self
            .documentation
            .classes
            .entry(self_type.clone())
            .or_insert(GdnativeClass {
                name: self_type,
                inherit: String::new(),
                documentation: String::new(),
                properties: Vec::new(),
                methods: Vec::new(),
                file: self.current_file.0.clone(),
            });
        if let syn::Fields::Named(fields) = &strukt.fields {
            class.get_properties(fields)
        }
        class.inherit = inherit;
        class.documentation = get_docs(&strukt.attrs);
    }

    fn visit_item_impl(&mut self, impl_block: &'ast ItemImpl) {
        if self.error.is_some() {
            return;
        }
        self.visit_item_impl_inner(impl_block);

        visit::visit_item_impl(self, impl_block)
    }
}
