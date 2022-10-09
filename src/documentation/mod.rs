//! Structures representing the documentation of a `gdnative` package.

mod builder;
mod helpers;

use crate::Result;
use helpers::*;
use std::{collections::HashMap, path::PathBuf};

/// Attribute in a function parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ParameterAttribute {
    /// No or unrecognized attribute
    None,
    /// `#[opt]`
    Opt,
}

/// Most type are simply `String`, but not all (e.g. return type)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    /// `Option<Type>`
    Option(String),
    /// A single-name type (like `i32`, or `MyType`)
    Named(String),
    /// `()`
    Unit,
}

/// Method in an `impl` block.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Method {
    /// Does this method have a `self` parameter ?
    pub has_self: bool,
    /// Name of the method.
    pub name: String,
    /// Name of the type that is being `impl`emented.
    pub self_type: String,
    /// Parameters of the method (excluding `self`).
    ///
    /// Contains:
    /// - the name of the parameter
    /// - it's `Type`
    /// - eventual attributes
    pub parameters: Vec<(String, Type, ParameterAttribute)>,
    /// Return type of the method.
    pub return_type: Type,
    /// Documentation associated with the method
    ///
    /// # Note
    /// This keeps the leading space in `/// doc`
    pub documentation: String,
    /// File in which the method was declared
    pub file: PathBuf,
}

/// Property exported to godot
///
/// # Example
/// ```
/// # use gdnative::prelude::*;
/// # use gdnative::api::Resource;
/// #[derive(NativeClass)]
/// #[inherit(Resource)]
/// struct MyResource {
///     /// Some doc
///     #[property]
///     my_property: String,
/// }
/// # #[methods] impl MyResource { pub fn new(_: &Resource) -> Self { todo!() } }
/// ```
/// Translates into:
/// ```text
/// name: "my_property",
/// typ: Type::Named("String"),
/// documentation: "Some doc"
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Property {
    /// Name of the property
    pub name: String,
    /// Type of the property
    pub typ: Type,
    /// Documentation associated with  the property
    pub documentation: String,
}

/// Structure that derive `NativeClass`
///
/// # Note
/// It cannot be generic.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GdnativeClass {
    /// Name of the structure
    pub name: String,
    /// Name of the type in `#[inherit(...)]`
    pub inherit: String,
    /// Documentation associated with the structure.
    pub documentation: String,
    /// Properties exported by the structure
    pub properties: Vec<Property>,
    /// Exported methods of this structure
    ///
    /// As per `gdnative`'s documentation, exported methods are
    /// - In a `#[methods]` impl block
    /// - Either `new`, or marked with `#[export]`
    pub methods: Vec<Method>,
    /// File in which the `struct` was declared
    pub file: PathBuf,
}

/// Holds the documentation for the crate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Documentation {
    /// Name of the crate.
    pub name: String,
    /// Path of the root file for the documentation.
    pub root_file: PathBuf,
    /// Documentation of the root module.
    pub root_documentation: String,
    /// Classes, organized by name.
    // FIXME: the name of the class is repeated all over the place.
    //       It may be better to use identifiers ?
    pub classes: HashMap<String, GdnativeClass>,
}

impl Documentation {
    pub(crate) fn from_root_file(name: String, root_file: PathBuf) -> Result<Self> {
        use syn::visit::Visit;

        let root_file_content = read_file_at(&root_file)?;
        let mut builder = builder::DocumentationBuilder {
            documentation: Self {
                name,
                root_file: root_file.clone(),
                root_documentation: String::new(),
                classes: HashMap::new(),
            },
            current_file: (root_file, true),
            current_module: Vec::new(),
            error: None,
        };
        let root_documentation = get_docs(&root_file_content.attrs);
        for item in root_file_content.items {
            builder.visit_item(&item);
            if let Some(error) = builder.error.take() {
                return Err(error);
            }
        }
        builder.documentation.root_documentation = root_documentation;
        Ok(builder.documentation)
    }
}

impl GdnativeClass {
    /// Check that the method is exported, parse it, and add it to the class.
    fn add_method(&mut self, method: &syn::ImplItemMethod, file: PathBuf) {
        let syn::ImplItemMethod {
            vis, attrs, sig, ..
        } = method;

        // not public
        if !matches!(vis, syn::Visibility::Public(_)) {
            return;
        }
        // not exported nor a constructor
        if !(attributes_contains(attrs, "export") || sig.ident == "new") {
            return;
        }

        let has_self = sig.receiver().is_some();
        let syn::Signature {
            ident: method_name,
            inputs,
            output,
            ..
        } = sig;

        let mut parameters = inputs.into_iter();
        if has_self {
            parameters.next();
        }
        parameters.next(); // inherit argument
        let parameters = {
            let mut params = Vec::new();
            for arg in parameters {
                if let syn::FnArg::Typed(syn::PatType { attrs, pat, ty, .. }) = arg {
                    let arg_name = {
                        if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = pat.as_ref() {
                            ident.to_string()
                        } else {
                            String::new()
                        }
                    };

                    params.push((
                        arg_name,
                        get_type_name(*ty.clone())
                            .unwrap_or_else(|| Type::Named("{ERROR}".to_string())),
                        if attributes_contains(attrs, "opt") {
                            ParameterAttribute::Opt
                        } else {
                            ParameterAttribute::None
                        },
                    ))
                }
            }
            params
        };

        let return_type = match output {
            syn::ReturnType::Default => Type::Unit,
            syn::ReturnType::Type(_, typ) => get_type_name(*typ.clone()).unwrap_or(Type::Unit),
        };
        log::trace!(
            "added method {}: parameters = {:?}, return = {:?}",
            method_name,
            parameters,
            return_type
        );
        self.methods.push(Method {
            has_self,
            name: method_name.to_string(),
            self_type: self.name.clone(),
            parameters,
            return_type,
            documentation: get_docs(attrs),
            file,
        })
    }

    /// Extract `#[property]` fields
    fn get_properties(&mut self, fields: &syn::FieldsNamed) {
        for field in &fields.named {
            if attributes_contains(&field.attrs, "property") {
                let property = Property {
                    name: field
                        .ident
                        .as_ref()
                        .map(|ident| ident.to_string())
                        .unwrap_or_default(),
                    // FIXME: log unsupported types
                    typ: get_type_name(field.ty.clone()).unwrap_or(Type::Unit),
                    documentation: get_docs(&field.attrs),
                };
                log::trace!(
                    "added property '{}' of type {:?}",
                    property.name,
                    property.typ
                );
                self.properties.push(property)
            }
        }
    }
}
