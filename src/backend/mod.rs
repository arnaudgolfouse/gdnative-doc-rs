mod config;
mod markdown;

pub use config::Config;
pub use markdown::MarkdownCallbacks;

use crate::documentation::{Documentation, Type};
use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Parser, Tag};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

pub trait Callbacks {
    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>);
    fn finish_encoding(&mut self, s: &mut String);
}

#[derive(Default)]
pub struct HtmlCallbacks {}

impl Callbacks for HtmlCallbacks {
    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>) {
        pulldown_cmark::html::push_html(s, events.into_iter())
    }
    fn finish_encoding(&mut self, _s: &mut String) {}
}

pub struct Generator<'a> {
    config: &'a Config,
    callbacks: Box<dyn Callbacks>,
    documentation: &'a Documentation,
}

impl<'a> Generator<'a> {
    pub fn new(
        config: &'a Config,
        documentation: &'a Documentation,
        callbacks: Box<dyn Callbacks>,
    ) -> Self {
        Self {
            config,
            callbacks,
            documentation,
        }
    }

    /// Generate the root documentation file of the crate.
    pub fn generate_root_file(&mut self, backend: Backend) -> String {
        let mut root_file = String::new();
        let config = &self.config;
        let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
            Self::broken_link_callback(config, broken_link.reference)
                .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
        };
        let class_iterator = EventIterator {
            context: config,
            parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                &self.documentation.root_documentation,
                self.config.markdown_options,
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
                format!("./{}.{}", class_name, backend.extension()).into(),
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
        self.callbacks.encode(&mut root_file, events);
        self.callbacks.finish_encoding(&mut root_file);
        root_file
    }

    /// Generate pairs of (class_name, file_content).
    pub fn generate_files(&mut self) -> Vec<(String, String)> {
        // TODO: this is kind of a mess, need to cleanup
        let mut results = Vec::new();
        for (name, class) in &self.documentation.classes {
            let mut class_file = String::new();

            self.callbacks.encode(
                &mut class_file,
                vec![
                    Event::Start(Tag::Heading(1)),
                    Event::Text(CowStr::Borrowed(&name)),
                    Event::End(Tag::Heading(1)),
                ],
            );
            let config = &self.config;
            let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                Self::broken_link_callback(config, broken_link.reference)
                    .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
            };
            let class_iterator = EventIterator {
                context: config,
                parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                    &class.documentation,
                    self.config.markdown_options,
                    Some(&mut broken_link_callback),
                ),
            };

            self.callbacks
                .encode(&mut class_file, class_iterator.into_iter().collect());
            for method in &class.methods {
                self.callbacks
                    .encode(&mut class_file, vec![Event::Start(Tag::Heading(2))]);
                let mut function_section = String::from("func ");
                function_section.push_str(&method.name);
                function_section.push('(');
                for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
                    function_section.push_str(&name);
                    function_section.push_str(": ");
                    self.callbacks.encode(
                        &mut class_file,
                        vec![Event::Text(CowStr::Borrowed(&function_section))],
                    );
                    function_section.clear();
                    let (typ, optional) = match typ {
                        Type::Option(typ) => (self.config.godot_name(typ.as_str()), true),
                        Type::Named(typ) => (self.config.godot_name(typ.as_str()), false),
                        Type::Unit => ("void", false),
                    };
                    match self.config.resolve(typ) {
                        Some(link) => {
                            let link = Tag::Link(
                                LinkType::Shortcut,
                                CowStr::Borrowed(&link),
                                CowStr::Borrowed(""),
                            );
                            self.callbacks.encode(
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
                self.callbacks.encode(
                    &mut class_file,
                    vec![Event::Text(CowStr::Borrowed(&function_section))],
                );

                let return_type = match &method.return_type {
                    Type::Option(typ) | Type::Named(typ) => self.config.godot_name(typ.as_str()),
                    Type::Unit => "void",
                };
                let resolve_return = self.config.resolve(return_type);
                let return_events = match resolve_return.as_ref().map(|return_link| {
                    Tag::Link(
                        LinkType::Shortcut,
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

                self.callbacks.encode(&mut class_file, return_events);
                self.callbacks.encode(&mut class_file, vec![Event::Rule]);
                let config = &self.config;
                let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                    Self::broken_link_callback(config, broken_link.reference)
                        .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
                };
                let method_iterator = EventIterator {
                    context: config,
                    parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                        &method.documentation,
                        self.config.markdown_options,
                        Some(&mut broken_link_callback),
                    ),
                };
                self.callbacks
                    .encode(&mut class_file, method_iterator.into_iter().collect());
            }
            self.callbacks.finish_encoding(&mut class_file);
            results.push((name.clone(), class_file))
        }
        results
    }

    fn broken_link_callback(config: &Config, mut link: &str) -> Option<String> {
        if link.starts_with('`') && link.ends_with('`') && link.len() > 1 {
            link = &link[1..link.len() - 1];
        }
        config.resolve(link)
    }
}

struct EventIterator<'a> {
    context: &'a Config,
    parser: Parser<'a>,
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_event = self.parser.next()?;
        next_event = match next_event {
            // matches broken reference links that have been restored by the callback
            // and replaces them by shortcut variants
            Event::Start(Tag::Link(LinkType::ShortcutUnknown, dest, title)) => {
                Event::Start(Tag::Link(LinkType::Shortcut, dest, title))
            }
            Event::End(Tag::Link(LinkType::ShortcutUnknown, dest, title)) => {
                Event::End(Tag::Link(LinkType::Shortcut, dest, title))
            }
            _ => next_event,
        };
        self.context.resolve_event(&mut next_event);
        Some(next_event)
    }
}
