mod config;
mod markdown;

pub use config::Config;
pub use markdown::encode_markdown;

use crate::documentation::{Documentation, Type};
use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Parser, Tag};

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Markdown,
    Html,
}

impl Backend {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Markdown => "md",
            Self::Html => "html",
        }
    }
}

pub struct Generator {
    config: Config,
    encoder: Box<dyn Fn(&mut String, Vec<Event<'_>>)>,
    documentation: Documentation,
}

impl Generator {
    pub fn new(
        config: Config,
        documentation: Documentation,
        encoder: Box<dyn Fn(&mut String, Vec<Event<'_>>)>,
    ) -> Self {
        Self {
            config,
            encoder,
            documentation,
        }
    }

    /// Generate the root documentation file of the crate.
    pub fn generate_root_file(&self) -> String {
        let mut root_file = String::new();
        let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
            self.broken_link_callback(broken_link.reference)
                .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
        };
        let class_iterator = EventIterator {
            context: self,
            parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                &self.documentation.root_documentation,
                self.config.options,
                Some(&mut broken_link_callback),
            ),
        };
        let mut events: Vec<_> = class_iterator.into_iter().collect();
        events.extend(vec![
            Event::Start(Tag::Heading(1)),
            Event::Text(CowStr::Borrowed("Classes:")),
            Event::End(Tag::Heading(1)),
            Event::Start(Tag::List(None)),
        ]);
        for (class_name, _) in &self.documentation.classes {
            let link = Tag::Link(
                LinkType::Inline,
                format!("./{}.{}", class_name, self.config.backend_extension()).into(),
                CowStr::Borrowed(""),
            );
            events.extend(vec![
                Event::Start(Tag::Item),
                Event::Start(link.clone()),
                Event::Text(CowStr::Borrowed(&class_name)),
                Event::End(link.clone()),
                Event::End(Tag::Item),
            ])
        }
        events.push(Event::End(Tag::List(None)));
        (self.encoder)(&mut root_file, events);
        root_file
    }

    /// Generate pairs of (class_name, file_content).
    pub fn generate_files(&self) -> Vec<(String, String)> {
        // TODO: this is kind of a mess, need to cleanup
        let mut results = Vec::new();
        for (name, class) in &self.documentation.classes {
            let mut class_file = String::new();

            (self.encoder)(
                &mut class_file,
                vec![
                    Event::Start(Tag::Heading(1)),
                    Event::Text(CowStr::Borrowed(&name)),
                    Event::End(Tag::Heading(1)),
                ],
            );
            let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                self.broken_link_callback(broken_link.reference)
                    .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
            };
            let class_iterator = EventIterator {
                context: self,
                parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                    &class.documentation,
                    self.config.options,
                    Some(&mut broken_link_callback),
                ),
            };

            (self.encoder)(&mut class_file, class_iterator.into_iter().collect());
            for method in &class.methods {
                (self.encoder)(&mut class_file, vec![Event::Start(Tag::Heading(2))]);
                let mut function_section = String::from("func ");
                function_section.push_str(&method.name);
                function_section.push('(');
                for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
                    function_section.push_str(&name);
                    function_section.push_str(": ");
                    (self.encoder)(
                        &mut class_file,
                        vec![Event::Text(CowStr::Borrowed(&function_section))],
                    );
                    function_section.clear();
                    let (typ, optional) = match typ {
                        Type::Option(typ) => (self.config.godot_name(typ.as_str()), true),
                        Type::Named(typ) => (self.config.godot_name(typ.as_str()), false),
                        Type::Unit => ("void", false),
                    };
                    match self.resolve(typ) {
                        Some(link) => {
                            let link = Tag::Link(
                                LinkType::Inline,
                                CowStr::Borrowed(&link),
                                CowStr::Borrowed(""),
                            );
                            (self.encoder)(
                                &mut class_file,
                                vec![
                                    Event::Start(link.clone()),
                                    Event::Text(CowStr::Borrowed(self.config.godot_name(typ))),
                                    Event::End(link),
                                ],
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
                (self.encoder)(
                    &mut class_file,
                    vec![Event::Text(CowStr::Borrowed(&function_section))],
                );

                let return_type = match &method.return_type {
                    Type::Option(typ) | Type::Named(typ) => self.config.godot_name(typ.as_str()),
                    Type::Unit => "void",
                };
                let resolve_return = self.resolve(return_type);
                let return_events = match resolve_return.as_ref().map(|return_link| {
                    Tag::Link(
                        LinkType::Inline,
                        CowStr::Borrowed(&return_link),
                        CowStr::Borrowed(""),
                    )
                }) {
                    Some(link) => {
                        vec![
                            Event::Start(link.clone()),
                            Event::Text(CowStr::Borrowed(self.config.godot_name(return_type))),
                            Event::End(link),
                            Event::End(Tag::Heading(2)),
                        ]
                    }
                    None => {
                        vec![
                            Event::Text(CowStr::Borrowed(self.config.godot_name(return_type))),
                            Event::End(Tag::Heading(2)),
                        ]
                    }
                };

                (self.encoder)(&mut class_file, return_events);
                (self.encoder)(&mut class_file, vec![Event::Rule]);
                let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                    self.broken_link_callback(broken_link.reference)
                        .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
                };
                let method_iterator = EventIterator {
                    context: self,
                    parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                        &method.documentation,
                        self.config.options,
                        Some(&mut broken_link_callback),
                    ),
                };
                (self.encoder)(&mut class_file, method_iterator.into_iter().collect());
            }
            results.push((name.clone(), class_file))
        }
        results
    }

    fn broken_link_callback(&self, mut link: &str) -> Option<String> {
        if link.starts_with('`') && link.ends_with('`') && link.len() > 1 {
            link = &link[1..link.len() - 1];
        }
        self.resolve(link)
    }

    /// Resolve a name to the class it must link to.
    fn resolve(&self, link: &str) -> Option<String> {
        if let Some(link) = self.config.overrides.get(link).cloned() {
            return Some(link);
        }
        if let Ok(link) = syn::parse_str::<syn::Path>(link) {
            let mut base = match link.segments.last() {
                None => return None,
                Some(base) => base.ident.to_string(),
            };
            if let Some(path) = self.config.overrides.get(&base).cloned() {
                Some(path)
            } else {
                base = match self.config.rust_to_godot.get(&base).cloned() {
                    Some(base) => base,
                    None => base,
                };
                if let Some(path) = self.config.godot_classes.get(&base).cloned() {
                    Some(path)
                } else {
                    None
                }
            }
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
    context: &'a Generator,
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

pub fn encode_html(s: &mut String, events: Vec<Event<'_>>) {
    pulldown_cmark::html::push_html(s, events.into_iter())
}
