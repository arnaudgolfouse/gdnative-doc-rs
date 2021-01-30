#[cfg(test)]
mod tests;

use super::{Callbacks, Method, Property, Resolver};
use pulldown_cmark::{Alignment, CodeBlockKind, Event, LinkType, Tag};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum Nesting {
    /// Tracks the index of the current list
    ListLevel(Option<u64>),
    /// First item after the `"- "`
    StartListItem,
    /// Member of a list item
    ListItem,
    /// Quoted text: `"> "`
    Quote,
    /// Indented code: add 4 spaces
    IndentedCode,
}

/// Implementation of [`Callbacks`] for markdown.
#[derive(Default)]
pub(crate) struct MarkdownCallbacks {
    /// The same name can be used for multiple shortcut links, because they
    /// are not all defined in the same place.
    ///
    /// So we keep them all, and disambiguate via `name`, `name-1`,
    /// `name-2`, ...
    links: HashMap<String, Vec<String>>,
    shortcut_link: Option<String>,
    /// Stack of tables alignment
    tables_alignements: Vec<Vec<Alignment>>,
    nesting: Vec<Nesting>,
}

impl Callbacks for MarkdownCallbacks {
    fn extension(&self) -> &'static str {
        "md"
    }

    fn start_method(&mut self, s: &mut String, resolver: &Resolver, method: &Method) {
        (self as &mut dyn Callbacks).start_method_default(s, resolver, method)
    }

    fn start_property(&mut self, s: &mut String, resolver: &Resolver, property: &Property) {
        (self as &mut dyn Callbacks).start_property_default(s, resolver, property)
    }

    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>) {
        for event in events {
            match event {
                Event::Start(tag) => match tag {
                    Tag::Paragraph => self.apply_nesting(s),
                    Tag::Heading(level) => {
                        self.apply_nesting(s);
                        for _ in 0..level {
                            s.push('#');
                        }
                        s.push(' ');
                    }
                    Tag::BlockQuote => self.nesting.push(Nesting::Quote),
                    Tag::CodeBlock(kind) => match kind {
                        CodeBlockKind::Indented => {
                            self.apply_nesting(s);
                            trim(s);
                            self.nesting.push(Nesting::IndentedCode);
                            self.apply_nesting(s);
                        }
                        CodeBlockKind::Fenced(lang) => {
                            self.apply_nesting(s);
                            s.push_str("```");
                            s.push_str(&lang);
                            self.apply_nesting(s);
                        }
                    },
                    Tag::List(level) => self.nesting.push(Nesting::ListLevel(level)),
                    Tag::Item => {
                        self.apply_nesting(s);
                        self.start_new_item(s);
                        self.nesting.push(Nesting::StartListItem);
                    }
                    Tag::FootnoteDefinition(_) => {
                        log::warn!("FootnoteDefinition: Unsupported at the moment")
                    }
                    Tag::Table(alignment) => {
                        self.tables_alignements.push(alignment);
                    }
                    Tag::TableHead => self.apply_nesting(s),
                    Tag::TableRow => self.apply_nesting(s),
                    Tag::TableCell => s.push_str("| "),
                    Tag::Emphasis => s.push('*'),
                    Tag::Strong => s.push_str("**"),
                    Tag::Strikethrough => s.push_str("~~"),
                    Tag::Link(link_type, _, _) => {
                        if link_type == LinkType::Shortcut {
                            if self.shortcut_link.is_some() {
                                log::error!("Links are not supposed to be nested")
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
                    Tag::Paragraph => s.push('\n'),
                    Tag::Heading(_) => {}
                    Tag::BlockQuote => {
                        self.nesting.pop();
                    }
                    Tag::CodeBlock(kind) => match kind {
                        CodeBlockKind::Indented => {
                            self.nesting.pop();
                        }
                        CodeBlockKind::Fenced(_) => {
                            trim(s);
                            self.apply_nesting(s);
                            s.push_str("```");
                        }
                    },
                    Tag::List(_) => {
                        self.nesting.pop();
                    }
                    Tag::Item => {
                        self.nesting.pop();
                    }
                    Tag::FootnoteDefinition(_) => {}
                    Tag::Table(_) => s.push('\n'),
                    Tag::TableHead => {
                        if let Some(alignement) = self.tables_alignements.pop() {
                            self.apply_nesting(s);
                            for align in alignement {
                                s.push_str("| ");
                                match align {
                                    Alignment::None => s.push_str("--- "),
                                    Alignment::Left => s.push_str(":--- "),
                                    Alignment::Center => s.push_str(":---: "),
                                    Alignment::Right => s.push_str("---: "),
                                }
                            }
                        }
                    }
                    Tag::TableRow => {}
                    Tag::TableCell => {}
                    Tag::Emphasis => s.push('*'),
                    Tag::Strong => s.push_str("**"),
                    Tag::Strikethrough => s.push_str("~~"),
                    Tag::Link(link_type, dest, title) => {
                        s.push(']');
                        let closing_character = match link_type {
                            LinkType::Shortcut => {
                                if let Some(shortcut) = self.shortcut_link.take() {
                                    self.add_shortcut_link(shortcut, &dest);
                                }
                                None
                            }
                            _ => {
                                s.push('(');
                                s.push_str(&dest);
                                Some(')')
                            }
                        };

                        if !title.is_empty() {
                            s.push_str(" \"");
                            s.push_str(&title);
                            s.push('"');
                        }
                        if let Some(closing) = closing_character {
                            s.push(closing);
                        }
                    }
                    Tag::Image(_, _, _) => {}
                },
                Event::Text(text) => {
                    self.remove_top_most_start_item();
                    self.push_str(s, &text)
                }
                Event::Code(code) => {
                    self.remove_top_most_start_item();
                    self.push_str(s, "`");
                    self.push_str(s, &code);
                    self.push_str(s, "`");
                }
                Event::Html(html) => {
                    self.remove_top_most_start_item();
                    s.push_str(&html)
                }
                Event::FootnoteReference(_) => {
                    log::warn!("FootnoteReference: Unsupported at the moment")
                }
                Event::SoftBreak => self.apply_nesting(s),
                Event::HardBreak => {
                    s.push_str(" \\");
                    self.apply_nesting(s)
                }
                Event::Rule => {
                    self.apply_nesting(s);
                    s.push_str("________\n");
                }
                Event::TaskListMarker(checked) => {
                    self.remove_top_most_start_item();
                    s.push_str(if checked { "[X] " } else { "[ ] " })
                }
            }
        }
    }

    fn finish_encoding(&mut self, s: &mut String) {
        s.push('\n');
        let mut link_lines = Vec::new();
        self.shortcut_link.take();
        let links = std::mem::take(&mut self.links);
        for (shortcut, links) in links {
            for (index, link) in links.into_iter().enumerate() {
                let mut line = String::new();
                line.push_str("[");
                line.push_str(&shortcut);
                if index != 0 {
                    line.push_str(&format!("-{}", index));
                }
                line.push_str("]: ");
                line.push_str(&link);
                link_lines.push(line);
            }
        }
        link_lines.sort_unstable();
        for line in link_lines {
            s.push('\n');
            s.push_str(&line)
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

    /// Tries to add the `shortcut` to the list.
    ///
    /// - If it is not present, add it as-is.
    /// - If it is already present with the same `link`, at index:
    ///   - `0`: does nothing.
    ///   - `> 0`: change `shortcut` to `shortcut-index`.
    /// - If it is already present, but none of the `n` links associated
    /// with it correspond to `link`, add `link` to its list and change
    /// `shortcut` to `shortcut-n`.
    fn add_shortcut_link(&mut self, mut shortcut: String, link: &str) {
        if let Some(links) = self.links.get_mut(&shortcut) {
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
            self.links.insert(shortcut, vec![link.to_string()]);
        }
    }

    /// Start a new list item, like `"- "` or `"2. "`.
    fn start_new_item(&mut self, s: &mut String) {
        if let Some(Nesting::ListLevel(Some(index))) = self.nesting.last_mut() {
            *index += 1;
            s.push_str(&format!("{}. ", *index - 1))
        } else {
            s.push_str("- ");
        }
    }

    /// - If the last item in `self.nesting` is `Nesting::StartListItem`, replace it
    /// with `Nesting::ListItem` and returns.
    /// - Else, push a new line in `s` with indentation given by `self.nesting`.
    fn apply_nesting(&mut self, s: &mut String) {
        if self.remove_top_most_start_item() {
            return;
        }
        s.push('\n');
        for nesting in &mut self.nesting {
            match nesting {
                Nesting::ListLevel(_) => {}
                Nesting::ListItem => s.push_str("    "),
                Nesting::Quote => s.push_str("> "),
                Nesting::IndentedCode => s.push_str("    "),
                Nesting::StartListItem => {}
            }
        }
    }

    /// If the last item in `self.nesting` is `Nesting::StartListItem`, replace it
    /// with `Nesting::ListItem` and returns `true`.
    fn remove_top_most_start_item(&mut self) -> bool {
        if let Some(start_item @ Nesting::StartListItem) = self.nesting.last_mut() {
            *start_item = Nesting::ListItem;
            true
        } else {
            false
        }
    }
}

/// Remove trailing whitespace.
fn trim(s: &mut String) {
    while let Some(c) = s.pop() {
        if !c.is_whitespace() {
            s.push(c);
            break;
        }
    }
}
