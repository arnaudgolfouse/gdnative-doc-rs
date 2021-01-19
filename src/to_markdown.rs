use crate::documentation::{Documentation, Type};
use pulldown_cmark::{BrokenLink, CodeBlockKind, CowStr, Event, LinkType, Parser, Tag};

pub struct MarkdownContext<Encode>
where
    Encode: Fn(&mut String, Vec<Event<'_>>),
{
    encoder: Encode,
    godot_documentation_path: String,
    //gdnative_classes: Vec<String>,
    //added_classes: Vec<String>,
    documentation: Documentation,
}

impl<Encode> MarkdownContext<Encode>
where
    Encode: Fn(&mut String, Vec<Event<'_>>),
{
    pub fn new(documentation: Documentation, encoder: Encode) -> Self {
        Self {
            encoder,
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

            (self.encoder)(
                &mut html_result,
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
                    pulldown_cmark::Options::ENABLE_STRIKETHROUGH,
                    Some(&mut broken_link_callback),
                ),
            };

            (self.encoder)(&mut html_result, class_iterator.into_iter().collect());
            for method in &class.methods {
                (self.encoder)(&mut html_result, vec![Event::Start(Tag::Heading(2))]);
                let mut function_section = String::from("func ");
                function_section.push_str(&method.name);
                function_section.push('(');
                for (index, (name, typ, _)) in method.parameters.iter().enumerate() {
                    function_section.push_str(&name);
                    function_section.push_str(": ");
                    (self.encoder)(
                        &mut html_result,
                        vec![Event::Text(CowStr::Borrowed(&function_section))],
                    );
                    function_section.clear();
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
                                CowStr::Borrowed(""),
                            );
                            (self.encoder)(
                                &mut html_result,
                                vec![
                                    Event::Start(link.clone()),
                                    Event::Text(CowStr::Borrowed(self.godot_name(typ))),
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
                    &mut html_result,
                    vec![Event::Text(CowStr::Borrowed(&function_section))],
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
                    CowStr::Borrowed(""),
                );

                (self.encoder)(
                    &mut html_result,
                    vec![
                        Event::Start(link.clone()),
                        Event::Text(CowStr::Borrowed(
                            self.godot_name(self.godot_name(return_type)),
                        )),
                        Event::End(link),
                        Event::End(Tag::Heading(2)),
                    ],
                );
                (self.encoder)(&mut html_result, vec![Event::Rule]);
                let mut broken_link_callback = |broken_link: BrokenLink<'_>| {
                    self.broken_link_callback(broken_link.reference)
                        .map(|string| (CowStr::from(string.clone()), CowStr::Borrowed("")))
                };
                let method_iterator = EventIterator {
                    context: self,
                    parser: pulldown_cmark::Parser::new_with_broken_link_callback(
                        &method.documentation,
                        pulldown_cmark::Options::ENABLE_STRIKETHROUGH,
                        Some(&mut broken_link_callback),
                    ),
                };
                (self.encoder)(&mut html_result, method_iterator.into_iter().collect());
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

struct EventIterator<'a, Encode>
where
    Encode: Fn(&mut String, Vec<Event<'_>>),
{
    context: &'a MarkdownContext<Encode>,
    parser: Parser<'a>,
}

impl<'a, Encode> Iterator for EventIterator<'a, Encode>
where
    Encode: Fn(&mut String, Vec<Event<'_>>),
{
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

pub fn encode_markdown(s: &mut String, events: Vec<Event<'_>>) {
    let mut indentation: u32 = 0;
    for event in events {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    for _ in 0..indentation {
                        s.push_str("  ")
                    }
                }
                Tag::Heading(level) => {
                    for _ in 0..level {
                        s.push('#');
                    }
                    s.push(' ');
                }
                Tag::BlockQuote => {
                    eprintln!("BlockQuote: Unsupported at the moment")
                }
                Tag::CodeBlock(kind) => match kind {
                    CodeBlockKind::Indented => {
                        indentation += 1;
                    }
                    CodeBlockKind::Fenced(lang) => {
                        s.push_str("```");
                        s.push_str(&lang);
                        s.push('\n');
                    }
                },
                Tag::List(_) => {
                    indentation += 1;
                }
                Tag::Item => {
                    for _ in 0..(indentation.saturating_sub(1)) {
                        s.push_str("  ")
                    }
                    s.push_str("- ")
                }
                Tag::FootnoteDefinition(_) => {}
                Tag::Table(_) => {
                    eprintln!("Table: Unsupported at the moment")
                }
                Tag::TableHead => {}
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Emphasis => s.push('*'),
                Tag::Strong => s.push_str("**"),
                Tag::Strikethrough => s.push_str("~~"),
                Tag::Link(_, _, _) => s.push('['),
                Tag::Image(_, dest, title) => {
                    s.push_str("![](");
                    s.push_str(&dest);
                    if !title.is_empty() {
                        s.push_str(" \"");
                        s.push_str(&title);
                        s.push('"');
                    }
                    s.push(')');
                }
            },
            Event::End(tag) => match tag {
                Tag::Paragraph => s.push_str("\n\n"),
                Tag::Heading(_) => s.push('\n'),
                Tag::BlockQuote => {
                    eprintln!("BlockQuote: Unsupported at the moment")
                }
                Tag::CodeBlock(kind) => match kind {
                    CodeBlockKind::Indented => {
                        indentation -= 1;
                    }
                    CodeBlockKind::Fenced(_) => {
                        s.push_str("```");
                        s.push('\n');
                    }
                },
                Tag::List(_) => {
                    indentation -= 1;
                }
                Tag::Item => s.push('\n'),
                Tag::FootnoteDefinition(_) => {}
                Tag::Table(_) => {
                    eprintln!("Table: Unsupported at the moment")
                }
                Tag::TableHead => {}
                Tag::TableRow => {}
                Tag::TableCell => {}
                Tag::Emphasis => s.push('*'),
                Tag::Strong => s.push_str("**"),
                Tag::Strikethrough => s.push_str("~~"),
                Tag::Link(_, dest, title) => {
                    s.push_str("](");
                    s.push_str(&dest);
                    if !title.is_empty() {
                        s.push_str(" \"");
                        s.push_str(&title);
                        s.push('"');
                    }
                    s.push(')');
                }
                Tag::Image(_, _, _) => {}
            },
            Event::Text(text) => s.push_str(&text),
            Event::Code(code) => {
                s.push('`');
                s.push_str(&code);
                s.push('`');
            }
            Event::Html(html) => s.push_str(&html), // ???
            Event::FootnoteReference(_) => {}
            Event::SoftBreak => s.push('\n'),
            Event::HardBreak => s.push_str("\n\n"),
            Event::Rule => s.push_str("________\n"),
            Event::TaskListMarker(checked) => s.push_str(if checked { "[X]" } else { "[ ]" }),
        }
    }
}
