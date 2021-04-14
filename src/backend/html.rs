use super::{Callbacks, Event, Generator, Method, Property, Resolver};
use std::{collections::HashMap, path::PathBuf};

const PRISM_CSS: (&str, &str) = ("prism.css", include_str!("../../html/prism.css"));
const PRISM_JS: (&str, &str) = ("prism.js", include_str!("../../html/prism.js"));
const STYLE_CSS: (&str, &str) = ("style.css", include_str!("../../html/style.css"));

/// Implementation of [`Callbacks`] for html.
#[derive(Default)]
pub(crate) struct HtmlCallbacks {}

impl HtmlCallbacks {
    /// Generate an opening comment if `generator.opening_comment` is `true`.
    ///
    /// Else, returns an empty `String`.
    fn make_opening_comment(generator: &Generator, source_file: &dyn std::fmt::Display) -> String {
        if generator.opening_comment {
            format!(
                r"<!-- 
This file was automatically generated using [gdnative-doc-rs](https://github.com/arnaudgolfouse/gdnative-doc-rs)

Crate: {}
Source file: {}
-->

",
                generator.documentation.name, source_file,
            )
        } else {
            String::new()
        }
    }
}

impl Callbacks for HtmlCallbacks {
    fn extension(&self) -> &'static str {
        "html"
    }

    fn generate_files(&mut self, generator: Generator) -> HashMap<String, String> {
        const HTML_START: &str = r#"<!DOCTYPE HTML>
<html>

<head>
<meta charset="utf-8" />
<link rel="stylesheet" href="./prism.css"/>
<link rel="stylesheet" href="./style.css"/>
</head>

<body>
"#;
        const HTML_END: &str = r#"
<script src="./prism.js"></script>
</body>

</html>"#;

        let mut files = HashMap::new();

        generator
            .documentation
            .root_file
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();

        let index_content = format!(
            r"{}{}{}{}",
            Self::make_opening_comment(
                &generator,
                &generator
                    .documentation
                    .root_file
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default(),
            ),
            HTML_START,
            generator.generate_root_file("html", self),
            HTML_END
        );

        files.insert(String::from("index.html"), index_content);

        // directory that contains the root file
        let root_dir = generator.documentation.root_file.parent();
        for (name, class) in &generator.documentation.classes {
            let content = generator.generate_file(name, class, self);
            let file_content = format!(
                r"{}{}{}{}",
                Self::make_opening_comment(
                    &generator,
                    &root_dir
                        .and_then(|root_dir| class.file.strip_prefix(root_dir).ok())
                        .unwrap_or(&PathBuf::new())
                        .display(),
                ),
                HTML_START,
                content,
                HTML_END
            );
            let name = format!("{}.html", name);
            files.insert(name.clone(), file_content);
        }

        for (name, content) in &[PRISM_CSS, PRISM_JS, STYLE_CSS] {
            files.insert(name.to_string(), content.to_string());
        }

        files
    }

    fn start_method(&mut self, s: &mut String, resolver: &Resolver, method: &Method) {
        (self as &mut dyn Callbacks).start_method_default(s, resolver, method)
    }

    fn start_property(&mut self, s: &mut String, resolver: &Resolver, property: &Property) {
        (self as &mut dyn Callbacks).start_property_default(s, resolver, property)
    }

    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>) {
        pulldown_cmark::html::push_html(s, events.into_iter())
    }
}
