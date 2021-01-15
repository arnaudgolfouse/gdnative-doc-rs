use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum ParameterAttribute {
    None,
    /// `#[opt]`
    Opt,
}

/// Most type are simply `String`, but not all (e.g. return type)
#[derive(Clone, Debug)]
pub enum Type {
    Option(String),
    Named(String),
    Unit,
}

#[derive(Clone, Debug)]
pub struct Method {
    has_self: bool,
    name: String,
    self_type: String,
    parameters: Vec<(String, Type, ParameterAttribute)>,
    return_type: Type,
    documentation: String,
}

#[derive(Clone, Debug)]
pub struct GdnativeClass {
    name: String,
    inherit: String,
    documentation: String,
    methods: Vec<Method>,
}

#[derive(Clone, Debug)]
pub struct Documentation {
    root_documentation: String,
    classes: HashMap<String, GdnativeClass>,
}

impl Documentation {
    pub fn new() -> Self {
        Self {
            root_documentation: String::new(),
            classes: HashMap::new(),
        }
    }

    pub fn parse_from_module(
        &mut self,
        items: &[syn::Item],
        root_module: Option<&[syn::Attribute]>,
    ) -> syn::Result<()> {
        if let Some(attrs) = root_module {
            self.root_documentation = get_docs(attrs);
        }

        for item in items {
            match item {
                syn::Item::Impl(impl_block) => {
                    // check that this block has the #[methods] attribute
                    if attributes_contains(&impl_block.attrs, "methods") {
                        let self_type = match get_type_name(*impl_block.self_ty.clone()) {
                            Some(Type::Named(self_type)) => self_type,
                            _ => {
                                eprintln!("Unknown type in 'impl' block");
                                continue;
                            }
                        };
                        let class =
                            self.classes
                                .entry(self_type.clone())
                                .or_insert(GdnativeClass {
                                    name: self_type,
                                    inherit: String::new(),
                                    documentation: String::new(),
                                    methods: Vec::new(),
                                });
                        for item in &impl_block.items {
                            if let syn::ImplItem::Method(method) = item {
                                class.add_method(method);
                            }
                        }
                    }
                }
                syn::Item::Struct(strukt) => {
                    if let Some(inherit) = gdnative_class(&strukt) {
                        let self_type = strukt.ident.to_string();
                        let class =
                            self.classes
                                .entry(self_type.clone())
                                .or_insert(GdnativeClass {
                                    name: self_type,
                                    inherit: String::new(),
                                    documentation: String::new(),
                                    methods: Vec::new(),
                                });
                        class.inherit = inherit;
                        class.documentation = get_docs(&strukt.attrs);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

impl GdnativeClass {
    pub fn add_method(&mut self, method: &syn::ImplItemMethod) {
        let syn::ImplItemMethod {
            vis, attrs, sig, ..
        } = method;

        // not public
        if !matches!(vis, syn::Visibility::Public(_)) {
            return;
        }
        // not exported nor a constructor
        if !(attrs
            .iter()
            .any(|attr| attr.path.is_ident("export") && attr.tokens.is_empty())
            || sig.ident == "new")
        {
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
            let mut v = Vec::new();
            for arg in parameters {
                if let syn::FnArg::Typed(syn::PatType { attrs, pat, ty, .. }) = arg {
                    let arg_name = {
                        if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = pat.as_ref() {
                            ident.to_string()
                        } else {
                            String::new()
                        }
                    };

                    v.push((
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
            v
        };

        self.methods.push(Method {
            has_self,
            name: method_name.to_string(),
            self_type: self.name.clone(),
            parameters,
            return_type: match output {
                syn::ReturnType::Default => Type::Unit,
                syn::ReturnType::Type(_, typ) => get_type_name(*typ.clone()).unwrap_or(Type::Unit),
            },
            documentation: get_docs(&attrs),
        })
    }
}

/// Returns wether or not `attr` contains `#[attribute]`.
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

/// If this structure derives `NativeClass`, returns the name in `#[inherit(...)]`
pub(super) fn gdnative_class(strukt: &syn::ItemStruct) -> Option<String> {
    let mut implement_native_class = false;
    let mut inherit = None;
    for attr in &strukt.attrs {
        if let Ok(syn::Meta::List(syn::MetaList { path, nested, .. })) = attr.parse_meta() {
            if path.is_ident("inherit") && nested.len() == 1 {
                if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested.first().unwrap() {
                    if let Some(class) = path.get_ident() {
                        inherit = Some(class.to_string())
                    }
                }
            } else if path.is_ident("derive") && nested.len() == 1 {
                if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = nested.first().unwrap() {
                    if path.is_ident("NativeClass") {
                        implement_native_class = true;
                    }
                }
            }
        }
    }
    if implement_native_class {
        inherit
    } else {
        None
    }
}
