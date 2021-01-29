//! Structures representing the documentation of a `gdnative` package.

mod builder;

use crate::{Error, Result};
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
}

/// Property exported to godot
///
/// # Example
/// ```rust,ignore
/// #[derive(NativeClass)]
/// #[inherit(Resource)]
/// struct MyResource {
///     /// Some doc
///     #[property]
///     my_property: String,
/// }
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Documentation {
    /// Documentation of the root module.
    pub(crate) root_documentation: String,
    /// Classes, organized by name.
    ///
    /// FIXME: the name of the class is repeated all over the place.
    ///       It may be better to use identifiers
    pub(crate) classes: HashMap<String, GdnativeClass>,
}

impl Documentation {
    pub(crate) fn from_root_file(root_file: PathBuf) -> Result<Self> {
        use syn::visit::Visit;

        let root_file_content = read_file_at(&root_file)?;
        let mut builder = builder::DocumentationBuilder {
            documentation: Self {
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
    fn add_method(&mut self, method: &syn::ImplItemMethod) {
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
                        get_type_name(*ty.clone()).unwrap_or(Type::Named("{ERROR}".to_string())),
                        if attributes_contains(&attrs, "opt") {
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
            documentation: get_docs(&attrs),
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

/// Read and parse the file at the given `path` with `syn`, reporting any error.
fn read_file_at(path: &std::path::Path) -> Result<syn::File> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(syn::parse_file(&content)?),
        Err(err) => Err(Error::Io(path.to_path_buf(), err)),
    }
}

/// Returns whether or not `attr` contains `#[attribute]`.
fn attributes_contains(attrs: &[syn::Attribute], attribute: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.is_ident(attribute) && attr.tokens.is_empty())
}

/// Get this type's base name if it has one.
fn get_type_name(typ: syn::Type) -> Option<Type> {
    match typ {
        syn::Type::Path(path) => {
            let path_end = path.path.segments.last()?;
            let type_name = path_end.ident.to_string();
            match &path_end.arguments {
                syn::PathArguments::None => Some(Type::Named(type_name)),
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) => {
                    if type_name == "Option" && args.len() == 1 {
                        if let Some(syn::GenericArgument::Type(typ)) = args.first().cloned() {
                            if let Some(Type::Named(name)) = get_type_name(typ) {
                                Some(Type::Option(name))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                syn::PathArguments::Parenthesized(_) => None,
            }
        }
        syn::Type::Tuple(tuple) => {
            if tuple.elems.is_empty() {
                Some(Type::Unit)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extract '\n'-separated documentation from `attrs`.
fn get_docs(attrs: &[syn::Attribute]) -> String {
    let mut doc = String::new();
    let mut first_newline = true;
    for attr in attrs {
        if !attr.path.is_ident("doc") {
            continue;
        }
        match attr.parse_meta() {
            Ok(syn::Meta::NameValue(syn::MetaNameValue {
                lit: syn::Lit::Str(lit_str),
                ..
            })) => {
                if first_newline {
                    first_newline = false;
                } else {
                    doc.push('\n');
                }
                doc.push_str(&lit_str.value());
            }
            _ => {}
        }
    }
    doc
}
