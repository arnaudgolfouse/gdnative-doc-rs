use super::{Callbacks, Generator, Method};
use pulldown_cmark::{CodeBlockKind, Event, Tag};
use std::{collections::HashMap, path::PathBuf};

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

    fn generate_files(&mut self, generator: Generator) -> HashMap<String, String> {
        let mut files = HashMap::new();

        let root_dir = generator.documentation.root_file.parent();
        for (name, class) in &generator.documentation.classes {
            let content = format!(
                r"# This file was automatically generated using [gdnative-doc-rs](https://github.com/arnaudgolfouse/gdnative-doc-rs)
# 
# Crate: {}
# Source file: {}

{}",
                generator.documentation.name,
                root_dir
                    .and_then(|root_dir| class.file.strip_prefix(root_dir).ok())
                    .unwrap_or(&PathBuf::new())
                    .display(),
                generator.generate_file(name, class, self)
            );
            let name = format!("{}.gd", name);
            files.insert(
                name,
                String::from("extends \"res://addons/gut/test.gd\"\n\n") + &content,
            );
        }

        files
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
