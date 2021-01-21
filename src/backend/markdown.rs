use std::collections::HashMap;

use pulldown_cmark::{CodeBlockKind, Event, LinkType, Tag};

#[derive(Default)]
pub struct MarkdownCallbacks {
    /// The same name can be used for multiple shortcut links, because they
    /// are not all defined in the same place.
    ///
    /// So we keep them all, and disambiguate via `name`, `name-1`,
    /// `name-2`, ...
    links: HashMap<String, Vec<String>>,
    shortcut_link: Option<String>,
}

impl super::Callbacks for MarkdownCallbacks {
    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>) {
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
                    Tag::Link(link_type, _, _) => {
                        if link_type == LinkType::Shortcut {
                            if self.shortcut_link.is_some() {
                                log::error!("Nested links will break !")
                            }
                            self.shortcut_link = Some("".to_string());
                        }
                        s.push('[')
                    }
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
                    Tag::Link(link_type, dest, title) => {
                        s.push(']');
                        match link_type {
                            LinkType::Shortcut => {
                                s.push('[');
                                if let Some(mut shortcut) = self.shortcut_link.take() {
                                    self.add_shortcut_link(&mut shortcut, &dest);
                                    s.push_str(&shortcut)
                                }
                            }
                            _ => {
                                s.push('(');
                                s.push_str(&dest)
                            }
                        }

                        if !title.is_empty() {
                            s.push_str(" \"");
                            s.push_str(&title);
                            s.push('"');
                        }
                        if link_type == LinkType::Shortcut {
                            s.push(']');
                        } else {
                            s.push(')');
                        }
                    }
                    Tag::Image(_, _, _) => {}
                },
                Event::Text(text) => self.push_str(s, &text),
                Event::Code(code) => {
                    self.push_str(s, "`");
                    self.push_str(s, &code);
                    self.push_str(s, "`");
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

    fn finish_encoding(&mut self, s: &mut String) {
        self.shortcut_link.take();
        let links = std::mem::take(&mut self.links);
        for (shortcut, links) in links {
            for (index, link) in links.into_iter().enumerate() {
                s.push_str("\n[");
                s.push_str(&shortcut);
                if index != 0 {
                    s.push_str(&format!("-{}", index));
                }
                s.push_str("]: ");
                s.push_str(&link)
            }
        }
    }
}

impl MarkdownCallbacks {
    /// Push `string` in both `s` and `self.shortcut_link` if is is `Some`.
    fn push_str(&mut self, s: &mut String, string: &str) {
        s.push_str(string);
        if let Some(shortcut) = &mut self.shortcut_link {
            shortcut.push_str(string)
        }
    }

    fn add_shortcut_link(&mut self, shortcut: &mut String, link: &str) {
        if let Some(links) = self.links.get_mut(shortcut) {
            if let Some((index, _)) = links.iter().enumerate().find(|(_, l)| l == &link) {
                if index > 0 {
                    shortcut.push_str(&format!("-{}", index));
                }
            } else {
                let index = links.len();
                links.push(link.to_string());
                if index > 0 {
                    shortcut.push_str(&format!("-{}", index));
                }
            }
        } else {
            self.links
                .insert(shortcut.to_string(), vec![link.to_string()]);
        }
    }
}
