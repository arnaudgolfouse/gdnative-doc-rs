use super::Type;
use crate::{Error, Result};

/// Read and parse the file at the given `path` with `syn`, reporting any error.
pub(super) fn read_file_at(path: &std::path::Path) -> Result<syn::File> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(syn::parse_file(&content)?),
        Err(err) => Err(Error::Io(path.to_path_buf(), err)),
    }
}

/// Returns whether or not `attr` contains `#[attribute]`.
pub(super) fn attributes_contains(attrs: &[syn::Attribute], attribute: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.is_ident(attribute) && attr.tokens.is_empty())
}

/// Get this type's base name if it has one.
pub(super) fn get_type_name(typ: syn::Type) -> Option<Type> {
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
pub(super) fn get_docs(attrs: &[syn::Attribute]) -> String {
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
