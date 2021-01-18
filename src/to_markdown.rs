use crate::documentation::{Documentation, Type};
use pulldown_cmark::{html, BrokenLink, CowStr, Event, LinkType, Parser, Tag};
use std::iter;

pub struct MarkdownContext {
    godot_documentation_path: String,
    //gdnative_classes: Vec<String>,
    //added_classes: Vec<String>,
    documentation: Documentation,
}

impl MarkdownContext {
    pub fn from_documentation(documentation: Documentation) -> MarkdownContext {
        MarkdownContext {
            godot_documentation_path: String::from(
                "https://docs.godotengine.org/en/stable/classes",
            ),
            //gdnative_classes: vec![],
            //added_classes: Vec::new(),
            documentation,
        }
    }

    pub fn generate_files(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();
        for (name, class) in &self.documentation.classes {
            let mut html_result = String::new();

            html::push_html(
                &mut html_result,
                vec![
                    Event::Start(Tag::Heading(1)),
                    Event::Text(CowStr::Borrowed(&name)),
                    Event::End(Tag::Heading(1)),
                ]
                .into_iter(),
            );
            let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                self.broken_link_callback(broken_link.reference)
                    .map(|string| (CowStr::from(string.clone()), CowStr::from(string)))
            };
            let class_iterator = EventIterator {
                context: self,
                parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                    &class.documentation,
                    pulldown_cmark::Options::ENABLE_STRIKETHROUGH,
                    Some(&mut broken_link_callback),
                ),
            };

            html::push_html(&mut html_result, class_iterator);
            for method in &class.methods {
                html::push_html(&mut html_result, iter::once(Event::Start(Tag::Heading(2))));
                let mut function_section = String::from("fn ");
                function_section.push_str(&method.name);
                function_section.push('(');
                for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
                    function_section.push_str(&name);
                    function_section.push_str(": ");
                    html::push_html(
                        &mut html_result,
                        iter::once(Event::Text(CowStr::Borrowed(&function_section))),
                    );
                    function_section.clear();
                    // TODO: type resolution here
                    let (typ, optional) = match typ {
                        Type::Option(typ) => (self.godot_name(typ.as_str()), true),
                        Type::Named(typ) => (self.godot_name(typ.as_str()), false),
                        Type::Unit => ("void", false),
                    };
                    match self.resolve(typ) {
                        Some(link) => {
                            let link = Tag::Link(
                                LinkType::Inline,
                                CowStr::Borrowed(&link),
                                CowStr::Borrowed(&link),
                            );
                            html::push_html(
                                &mut html_result,
                                vec![
                                    Event::Start(link.clone()),
                                    Event::Text(CowStr::Borrowed(self.godot_name(typ))),
                                    Event::End(link),
                                ]
                                .into_iter(),
                            );
                        }
                        None => function_section.push_str(typ),
                    }
                    if optional {
                        function_section.push_str(" (opt)");
                    }
                    if index + 1 != method.parameters.len() {
                        function_section.push_str(", ");
                    }
                }
                function_section.push_str(") -> ");
                html::push_html(
                    &mut html_result,
                    iter::once(Event::Text(CowStr::Borrowed(&function_section))),
                );

                let return_type = match &method.return_type {
                    Type::Option(typ) | Type::Named(typ) => self.godot_name(typ.as_str()),
                    Type::Unit => "void",
                };
                let resolve_return = match self.resolve(return_type) {
                    Some(resolved) => resolved,
                    None => String::new(),
                };
                let link = Tag::Link(
                    LinkType::Inline,
                    CowStr::Borrowed(&resolve_return),
                    CowStr::Borrowed(&resolve_return),
                );

                html::push_html(
                    &mut html_result,
                    vec![
                        Event::Start(link.clone()),
                        Event::Text(CowStr::Borrowed(
                            self.godot_name(self.godot_name(return_type)),
                        )),
                        Event::End(link),
                        Event::End(Tag::Heading(2)),
                    ]
                    .into_iter(),
                );
                html::push_html(&mut html_result, iter::once(Event::Rule));
                let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                    self.broken_link_callback(broken_link.reference)
                        .map(|string| (CowStr::from(string.clone()), CowStr::from(string)))
                };
                let method_iterator = EventIterator {
                    context: self,
                    parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                        &method.documentation,
                        pulldown_cmark::Options::ENABLE_STRIKETHROUGH,
                        Some(&mut broken_link_callback),
                    ),
                };
                html::push_html(&mut html_result, method_iterator);
            }
            results.push((name.clone(), html_result))
        }
        results
    }

    fn broken_link_callback(&self, mut link: &str) -> Option<String> {
        if link.starts_with('`') && link.ends_with('`') && link.len() > 1 {
            link = &link[1..link.len() - 1];
        }
        self.resolve(link)
    }

    fn godot_name<'a>(&self, name: &'a str) -> &'a str {
        match name {
            "i32" | "i64" => "int",
            "f32" => "float",
            "VariantArray" => "Array",
            "Int32Array" => "PoolIntArray",
            "Float32Array" => "PoolRealArray",
            _ => name,
        }
    }

    /// Resolve a name to the class it must link to.
    fn resolve(&self, link: &str) -> Option<String> {
        if let Ok(link) = syn::parse_str::<syn::Path>(link) {
            let base = match link.segments.last() {
                None => return None,
                Some(base) => base.ident.to_string(),
            };
            // TODO: differentiate between godot and user-defined classes
            Some(format!(
                "{}/class_{}.html",
                self.godot_documentation_path,
                base.to_lowercase()
            ))
        } else {
            None
        }
    }

    fn resolve_event(&self, event: &mut Event) {
        match event {
            Event::Start(Tag::Link(LinkType::Inline, dest, _))
            | Event::Start(Tag::Link(LinkType::Reference, dest, _))
            | Event::Start(Tag::Link(LinkType::Shortcut, dest, _))
            | Event::End(Tag::Link(LinkType::Inline, dest, _))
            | Event::End(Tag::Link(LinkType::Reference, dest, _))
            | Event::End(Tag::Link(LinkType::Shortcut, dest, _)) => match self.resolve(&dest) {
                Some(new_dest) => *dest = new_dest.into(),
                None => {}
            },
            Event::Start(Tag::Heading(n)) | Event::End(Tag::Heading(n)) => *n += 2,
            _ => {}
        }
    }
}

struct EventIterator<'a> {
    context: &'a MarkdownContext,
    parser: Parser<'a>,
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_event = self.parser.next()?;
        self.context.resolve_event(&mut next_event);
        Some(next_event)
    }
}