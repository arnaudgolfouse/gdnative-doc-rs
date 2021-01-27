mod gut;
mod html;
mod markdown;
mod resolve;

use std::path::PathBuf;

pub(super) use gut::GutCallbacks;
pub(super) use html::HtmlCallbacks;
pub(super) use markdown::MarkdownCallbacks;
pub use resolve::Resolver;

use crate::documentation::{Documentation, GdnativeClass, Method};
use pulldown_cmark::{Alignment, CowStr, Event, LinkType, Options as MarkdownOptions, Parser, Tag};

/// Generate a callback to resolve broken links.
///
/// We have to generate a new one for each use because the lifetimes on
/// `pulldown_cmark::Parser::new_with_broken_link_callback` are not yet
/// refined enough.
macro_rules! broken_link_callback {
    ($config:expr) => {
        move |broken_link: ::pulldown_cmark::BrokenLink| {
            use ::pulldown_cmark::CowStr;

            let mut link = broken_link.reference;
            if link.starts_with('`') && link.ends_with('`') && link.len() > 1 {
                link = &link[1..link.len() - 1];
            }
            $config
                .resolve(link)
                .map(|string| (CowStr::from(string), CowStr::Borrowed("")))
        }
    };
}

/// Kind of files generated by the crate
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Backend {
    Markdown { output_dir: PathBuf },
    Html { output_dir: PathBuf },
    Gut { output_dir: PathBuf },
}

/// Callbacks to encode markdown input in a given format.
pub trait Callbacks {
    /// File extension for the files generated by this callback.
    fn extension(&self) -> &'static str;
    /// Called before each class.
    ///
    /// **Default**: does nothing
    fn start_class(&mut self, _s: &mut String, _config: &Resolver, _class: &GdnativeClass) {}
    /// Called before each method.
    ///
    /// **Default**: does nothing
    fn start_method(&mut self, _s: &mut String, _config: &Resolver, _method: &Method) {}
    /// Encode the stream of `events` in `s`.
    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>);
    /// Called at the end of the processing for a given file.
    ///
    /// **Default**: does nothing
    fn finish_encoding(&mut self, _s: &mut String) {}
}

impl dyn Callbacks {
    /// Default start_method implementation, implemented on `dyn Callbacks` to avoid
    /// code duplication.
    ///
    /// This will create a level 3 section that looks like:
    ///
    /// `func name(arg1: type, ...) -> type`
    ///
    /// With appropriate linking, and a html link to this named `func-name`
    pub fn start_method_default(&mut self, s: &mut String, config: &Resolver, method: &Method) {
        let link = &format!("<a id=\"func-{}\"></a>", method.name);
        self.encode(
            s,
            vec![
                Event::Start(Tag::Heading(3)),
                Event::Html(CowStr::Borrowed(link)),
            ],
        );
        let mut method_section = String::from("func ");
        method_section.push_str(&method.name);
        method_section.push('(');
        for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
            method_section.push_str(&name);
            method_section.push_str(": ");
            self.encode(s, vec![Event::Text(CowStr::Borrowed(&method_section))]);
            method_section.clear();
            self.encode(s, config.encode_type(typ));
            if index + 1 != method.parameters.len() {
                method_section.push_str(", ");
            }
        }
        method_section.push_str(") -> ");
        let mut last_events = vec![Event::Text(CowStr::Borrowed(&method_section))];
        last_events.extend(config.encode_type(&method.return_type));
        last_events.push(Event::End(Tag::Heading(3)));
        last_events.push(Event::Rule);
        self.encode(s, last_events);
    }
}

/// Generate files given an encoding
pub(crate) struct Generator<'a> {
    /// Used to resolve links.
    resolver: &'a Resolver,
    /// Encoding functions.
    callbacks: Box<dyn Callbacks>,
    /// Data to encode.
    documentation: &'a Documentation,
    /// Markdown options
    markdown_options: MarkdownOptions,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(
        resolver: &'a Resolver,
        documentation: &'a Documentation,
        callbacks: Box<dyn Callbacks>,
        markdown_options: MarkdownOptions,
    ) -> Self {
        Self {
            resolver,
            callbacks,
            documentation,
            markdown_options,
        }
    }

    /// Generate the root documentation file of the crate.
    pub(crate) fn generate_root_file(&mut self, extension: &str) -> String {
        let mut root_file = String::new();
        let config = self.resolver;
        let mut broken_link_callback = broken_link_callback!(config);
        let class_iterator = EventIterator {
            context: config,
            parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                &self.documentation.root_documentation,
                self.markdown_options,
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
                format!("./{}.{}", class_name, extension).into(),
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
    pub(crate) fn generate_files(&mut self) -> Vec<(String, String)> {
        let mut results = Vec::new();
        for (name, class) in &self.documentation.classes {
            let mut class_file = String::new();
            let callbacks = &mut self.callbacks;
            callbacks.start_class(&mut class_file, self.resolver, class);
            let mut encode = |events| callbacks.encode(&mut class_file, events);
            let inherit_link = self.resolver.resolve(&class.inherit);

            // Name of the class + inherit
            let mut events = vec![
                Event::Start(Tag::Heading(1)),
                Event::Text(CowStr::Borrowed(&name)),
                Event::End(Tag::Heading(1)),
                Event::Start(Tag::Paragraph),
                Event::Start(Tag::Strong),
                Event::Text(CowStr::Borrowed("Inherit:")),
                Event::End(Tag::Strong),
                Event::Text(CowStr::Borrowed(" ")),
            ];
            if let Some(inherit_link) = inherit_link.as_ref() {
                events.extend(vec![
                    Event::Start(Tag::Link(
                        LinkType::Shortcut,
                        CowStr::Borrowed(&inherit_link),
                        CowStr::Borrowed(""),
                    )),
                    Event::Text(CowStr::Borrowed(&class.inherit)),
                    Event::End(Tag::Link(
                        LinkType::Shortcut,
                        CowStr::Borrowed(&inherit_link),
                        CowStr::Borrowed(""),
                    )),
                ])
            } else {
                events.push(Event::Text(CowStr::Borrowed(&class.inherit)))
            }
            events.extend(vec![
                Event::End(Tag::Paragraph),
                Event::Start(Tag::Heading(2)),
                Event::Text(CowStr::Borrowed("Description")),
                Event::End(Tag::Heading(2)),
            ]);
            encode(events);

            // Class description
            let config = self.resolver;
            let mut broken_link_callback = broken_link_callback!(config);
            let class_documentation = EventIterator {
                context: config,
                parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                    &class.documentation,
                    self.markdown_options,
                    Some(&mut broken_link_callback),
                ),
            };
            encode(class_documentation.into_iter().collect());

            // Methods table
            let mut events = vec![
                Event::Start(Tag::Heading(2)),
                Event::Text(CowStr::Borrowed("Methods")),
                Event::End(Tag::Heading(2)),
                Event::Start(Tag::Table(vec![Alignment::Left, Alignment::Left])),
                Event::Start(Tag::TableHead),
                Event::Start(Tag::TableCell),
                Event::Text(CowStr::Borrowed("returns")),
                Event::End(Tag::TableCell),
                Event::Start(Tag::TableCell),
                Event::Text(CowStr::Borrowed("method")),
                Event::End(Tag::TableCell),
                Event::End(Tag::TableHead),
            ];
            encode(std::mem::take(&mut events));

            for method in &class.methods {
                let link = format!("#func-{}", method.name);
                encode(vec![
                    Event::Start(Tag::TableRow),
                    Event::Start(Tag::TableCell),
                ]);
                encode(config.encode_type(&method.return_type));
                encode(vec![
                    Event::End(Tag::TableCell),
                    Event::Start(Tag::TableCell),
                ]);

                let link = Tag::Link(
                    LinkType::Reference,
                    link.into(),
                    method.name.as_str().into(),
                );
                encode(vec![
                    Event::Start(link.clone()),
                    Event::Text(CowStr::Borrowed(&method.name)),
                    Event::End(link),
                    Event::Text(CowStr::Borrowed("( ")),
                ]);
                for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
                    encode(vec![Event::Text(format!("{}: ", name).into())]);
                    encode(config.encode_type(typ));
                    if index + 1 != method.parameters.len() {
                        encode(vec![Event::Text(CowStr::Borrowed(", "))]);
                    }
                }

                encode(vec![
                    Event::Text(CowStr::Borrowed(" )")),
                    Event::End(Tag::TableCell),
                    Event::End(Tag::TableRow),
                ]);
            }

            events.extend(vec![
                Event::End(Tag::Table(vec![Alignment::Left, Alignment::Left])),
                Event::Start(Tag::Heading(2)),
                Event::Text(CowStr::Borrowed("Methods Descriptions")),
                Event::End(Tag::Heading(2)),
            ]);

            encode(events);

            // Methods
            for method in &class.methods {
                self.callbacks.start_method(&mut class_file, config, method);
                let callbacks = &mut self.callbacks;
                Self::generate_method(
                    |events| callbacks.encode(&mut class_file, events),
                    config,
                    method,
                    self.markdown_options
                )
            }
            self.callbacks.finish_encoding(&mut class_file);
            results.push((name.clone(), class_file))
        }
        results
    }

    /// Encode the documentation for `method`.
    fn generate_method(
        mut encode: impl FnMut(Vec<Event>),
        config: &Resolver,
        method: &Method,
        markdown_options: MarkdownOptions,
    ) {
        let config = config;
        let mut broken_link_callback = broken_link_callback!(config);
        let method_iterator = EventIterator {
            context: config,
            parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                &method.documentation,
                markdown_options,
                Some(&mut broken_link_callback),
            ),
        };
        encode(method_iterator.into_iter().collect());
    }
}

/// Iterate over [events](Event), resolving links and changing the resolved
/// broken links types.
struct EventIterator<'config, 'parser> {
    context: &'config Resolver,
    parser: Parser<'parser>,
}

impl<'config, 'parser> Iterator for EventIterator<'config, 'parser> {
    type Item = Event<'parser>;

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
