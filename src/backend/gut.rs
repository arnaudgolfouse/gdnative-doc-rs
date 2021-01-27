use pulldown_cmark::{CodeBlockKind, Event, Tag};

use super::{Callbacks, Method};

#[derive(Default)]
pub(crate) struct GutCallbacks {
    current_method: String,
    current_method_index: u8,
    active: bool,
}

impl Callbacks for GutCallbacks {
    fn extension(&self) -> &'static str {
        "gd"
    }

    fn start_class(
        &mut self,
        s: &mut String,
        _resolver: &super::Resolver,
        _class: &super::GdnativeClass,
    ) {
        s.push_str(r#"extends "res://addons/gut/test.gd""#);
        s.push_str("\n\n")
    }

    fn start_method(&mut self, _s: &mut String, _resolver: &super::Resolver, method: &Method) {
        self.current_method = method.name.clone();
        self.current_method_index = 0;
        self.active = false;
    }

    fn encode(&mut self, s: &mut String, events: Vec<Event>) {
        for event in events {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    if lang.as_ref() == "gdscript" {
                        self.active = true;
                        s.push_str("func test_");
                        s.push_str(&self.current_method);
                        if self.current_method_index > 0 {
                            s.push_str(&format!("_{}", self.current_method_index))
                        }
                        s.push_str("():\n");
                        self.current_method_index += 1;
                    }
                }
                Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    if lang.as_ref() == "gdscript" {
                        self.active = false;
                        s.push('\n');
                    }
                }
                Event::Text(text) => {
                    if self.active {
                        for line in text.as_ref().lines() {
                            s.push_str("    ");
                            s.push_str(line);
                            s.push('\n');
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
