use pulldown_cmark::{CodeBlockKind, Event, Tag};

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
                    log::warn!("BlockQuote: Unsupported at the moment")
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
                    log::warn!("Table: Unsupported at the moment")
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
                    log::warn!("BlockQuote: Unsupported at the moment")
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
                    log::warn!("Table: Unsupported at the moment")
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
