//! Module for implementing your own backend.
//!
//! To implement your own backend:
//! 1. Create a structure that represent your backend, and implement [`Callbacks`] on
//! it.
//!
//!     You can look in the source code of this crate to get examples of what that
//!     would look like.
//! 2. Add your backend to the `Builder` via the [`add_backend_with_callbacks`]
//! method.
//!
//! [`add_backend_with_callbacks`]: crate::Builder::add_backend_with_callbacks

mod callbacks;
mod gut;
mod html;
mod markdown;
mod resolve;

use crate::documentation::{Method, Property};
use pulldown_cmark::{Alignment, CowStr, Event, LinkType, Options as MarkdownOptions, Parser, Tag};

pub(super) use gut::GutCallbacks;
pub(super) use html::HtmlCallbacks;
pub(super) use markdown::MarkdownCallbacks;

pub use crate::documentation::Documentation;
pub use callbacks::Callbacks;
pub use resolve::Resolver;

/// Generate a callback to resolve broken links.
///
/// We have to generate a new one for each use because the lifetimes on
/// `pulldown_cmark::Parser::new_with_broken_link_callback` are not yet
/// refined enough.
macro_rules! broken_link_callback {
    ($resolver:expr) => {
        move |broken_link: ::pulldown_cmark::BrokenLink| {
            use ::pulldown_cmark::CowStr;

            let mut link = broken_link.reference;
            if link.starts_with('`') && link.ends_with('`') && link.len() > 1 {
                link = &link[1..link.len() - 1];
            }
            $resolver
                .resolve(link)
                .map(|string| (CowStr::from(string), CowStr::Borrowed("")))
        }
    };
}

/// Backend already implemented by this library.
///
/// This must be used in the [`Builder::add_backend`] method.
///
/// [`Builder::add_backend`]: crate::Builder::add_backend
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BuiltinBackend {
    /// Markdown backend
    ///
    /// This generates a file for every structure that implements `NativeClass` + an
    /// `index.md` file that contains the crate's documentation.
    Markdown,
    /// Html backend
    ///
    /// This generates a file for every structure that implements `NativeClass` + an
    /// `index.html` file that contains the crate's documentation.
    ///
    /// Also generates css and javascript files (for styling and code highlighting).
    Html,
    /// Gut backend
    /// 
    /// This generates a file for every structure that implements `NativeClass`, 
    /// generating tests from `gdscript` code blocks:
    /// ```
    /// # use gdnative::prelude::*;
    /// #[derive(NativeClass)]
    /// #[inherit(Node)]
    /// pub struct MyClass {}
    /// 
    /// #[methods]
    /// impl MyClass {
    ///     /// ```gdscript
    ///     /// var x = 0
    ///     /// assert_eq(x, 0)
    ///     /// ```
    ///     pub fn new(_: &Node) -> Self {
    ///         // ...
    /// # todo!()
    ///     }
    /// }
    /// ```
    /// Will generates the following in `MyClass.gd`:
    /// ```gdscript
    /// extends "res://addons/gut/test.gd"
    /// 
    /// func test_new():
    ///     var x = 0
    ///     assert_eq(x, 0)
    /// ```
    Gut,
}

/// Generate files given an encoding
#[derive(Debug)]
pub struct Generator<'a> {
    /// Used to resolve links.
    pub resolver: &'a Resolver,
    /// Holds the crate's documentation.
    pub documentation: &'a Documentation,
    /// Enabled markdown options
    pub markdown_options: MarkdownOptions,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(
        resolver: &'a Resolver,
        documentation: &'a Documentation,
        markdown_options: MarkdownOptions,
    ) -> Self {
        Self {
            resolver,
            documentation,
            markdown_options,
        }
    }

    /// Generate the root documentation file of the crate.
    ///
    /// This is a decent default: it generate a description based on
    pub fn generate_root_file(&mut self, extension: &str, callbacks: &mut dyn Callbacks) -> String {
        let mut root_file = String::new();
        let resolver = self.resolver;
        let mut broken_link_callback = broken_link_callback!(resolver);
        let class_iterator = EventIterator {
            context: resolver,
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
        for class_name in self.documentation.classes.keys() {
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
        callbacks.encode(&mut root_file, events);
        root_file
    }

    /// Generate pairs of (class_name, file_content).
    pub fn generate_files(&mut self, callbacks: &mut dyn Callbacks) -> Vec<(String, String)> {
        let mut results = Vec::new();
        for (name, class) in &self.documentation.classes {
            let mut class_file = String::new();
            let resolver = &self.resolver;

            let inherit_link = resolver.resolve(&class.inherit);

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
            callbacks.encode(&mut class_file, events);

            // Class description
            let mut broken_link_callback = broken_link_callback!(resolver);
            let class_documentation = EventIterator {
                context: resolver,
                parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                    &class.documentation,
                    self.markdown_options,
                    Some(&mut broken_link_callback),
                ),
            }
            .into_iter()
            .collect();
            callbacks.encode(&mut class_file, class_documentation);

            // Properties table
            if !class.properties.is_empty() {
                callbacks.encode(
                    &mut class_file,
                    Self::properties_table(&class.properties, resolver),
                )
            }

            // Methods table
            callbacks.encode(
                &mut class_file,
                Self::methods_table(&class.methods, resolver),
            );

            // Properties descriptions
            if !class.properties.is_empty() {
                callbacks.encode(
                    &mut class_file,
                    vec![
                        Event::Start(Tag::Heading(2)),
                        Event::Text(CowStr::Borrowed("Properties Descriptions")),
                        Event::End(Tag::Heading(2)),
                    ],
                );
                for property in &class.properties {
                    callbacks.start_property(&mut class_file, resolver, property);
                    let mut broken_link_callback = broken_link_callback!(resolver);
                    let property_documentation = EventIterator {
                        context: resolver,
                        parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                            &property.documentation,
                            self.markdown_options,
                            Some(&mut broken_link_callback),
                        ),
                    }
                    .into_iter()
                    .collect();
                    callbacks.encode(&mut class_file, property_documentation);
                }
            }

            // Methods descriptions
            callbacks.encode(
                &mut class_file,
                vec![
                    Event::Start(Tag::Heading(2)),
                    Event::Text(CowStr::Borrowed("Methods Descriptions")),
                    Event::End(Tag::Heading(2)),
                ],
            );
            for method in &class.methods {
                callbacks.start_method(&mut class_file, resolver, method);
                let mut broken_link_callback = broken_link_callback!(resolver);
                let method_documentation = EventIterator {
                    context: resolver,
                    parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                        &method.documentation,
                        self.markdown_options,
                        Some(&mut broken_link_callback),
                    ),
                }
                .into_iter()
                .collect();
                callbacks.encode(&mut class_file, method_documentation);
            }
            results.push((name.clone(), class_file))
        }
        results
    }

    /// Create a table summarizing the `properties`.
    fn properties_table<'ev>(
        properties: &'ev [Property],
        resolver: &'ev Resolver,
    ) -> Vec<Event<'ev>> {
        let mut events = vec![
            Event::Start(Tag::Heading(2)),
            Event::Text(CowStr::Borrowed("Properties")),
            Event::End(Tag::Heading(2)),
            Event::Start(Tag::Table(vec![Alignment::Left, Alignment::Left])),
            Event::Start(Tag::TableHead),
            Event::Start(Tag::TableCell),
            Event::Text(CowStr::Borrowed("type")),
            Event::End(Tag::TableCell),
            Event::Start(Tag::TableCell),
            Event::Text(CowStr::Borrowed("property")),
            Event::End(Tag::TableCell),
            Event::End(Tag::TableHead),
        ];

        for property in properties {
            let link = Tag::Link(
                LinkType::Reference,
                format!("#property-{}", property.name).into(),
                property.name.as_str().into(),
            );
            events.push(Event::Start(Tag::TableRow));
            events.push(Event::Start(Tag::TableCell));
            events.extend(resolver.encode_type(&property.typ));
            events.extend(vec![
                Event::End(Tag::TableCell),
                Event::Start(Tag::TableCell),
                Event::Start(link.clone()),
                Event::Text(CowStr::Borrowed(property.name.as_str())),
                Event::End(link),
                Event::End(Tag::TableCell),
                Event::End(Tag::TableRow),
            ]);
        }

        events.push(Event::End(Tag::Table(vec![
            Alignment::Left,
            Alignment::Left,
        ])));

        events
    }

    fn methods_table<'ev>(methods: &'ev [Method], resolver: &'ev Resolver) -> Vec<Event<'ev>> {
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

        for method in methods {
            let link = format!("#func-{}", method.name);
            events.push(Event::Start(Tag::TableRow));
            events.push(Event::Start(Tag::TableCell));
            events.extend(resolver.encode_type(&method.return_type));
            events.push(Event::End(Tag::TableCell));
            events.push(Event::Start(Tag::TableCell));

            let link = Tag::Link(
                LinkType::Reference,
                link.into(),
                method.name.as_str().into(),
            );
            events.extend(vec![
                Event::Start(link.clone()),
                Event::Text(CowStr::Borrowed(&method.name)),
                Event::End(link),
                Event::Text(CowStr::Borrowed("( ")),
            ]);
            for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
                events.push(Event::Text(format!("{}: ", name).into()));
                events.extend(resolver.encode_type(typ));
                if index + 1 != method.parameters.len() {
                    events.push(Event::Text(CowStr::Borrowed(", ")));
                }
            }

            events.extend(vec![
                Event::Text(CowStr::Borrowed(" )")),
                Event::End(Tag::TableCell),
                Event::End(Tag::TableRow),
            ]);
        }

        events.push(Event::End(Tag::Table(vec![
            Alignment::Left,
            Alignment::Left,
        ])));

        events
    }
}

/// Iterate over [events](Event), resolving links and changing the resolved
/// broken links types.
struct EventIterator<'resolver, 'parser> {
    context: &'resolver Resolver,
    parser: Parser<'parser>,
}

impl<'resolver, 'parser> Iterator for EventIterator<'resolver, 'parser> {
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
