use super::{Callbacks, Event, Generator, Method, Property, Resolver};
use std::collections::HashMap;

const PRISM_CSS: (&str, &str) = ("prism.css", include_str!("../../html/prism.css"));
const PRISM_JS: (&str, &str) = ("prism.js", include_str!("../../html/prism.js"));
const STYLE_CSS: (&str, &str) = ("style.css", include_str!("../../html/style.css"));

/// Implementation of [`Callbacks`] for html.
#[derive(Default)]
pub(crate) struct HtmlCallbacks {}

impl Callbacks for HtmlCallbacks {
    fn extension(&self) -> &'static str {
        "html"
    }

    fn generate_files(&mut self, mut generator: Generator) -> HashMap<String, String> {
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

        files.insert(
            String::from("index.html"),
            HTML_START.to_string() + &generator.generate_root_file("html", self) + HTML_END,
        );
        for (mut name, content) in generator.generate_files(self) {
            name.push_str(".html");
            files.insert(name, HTML_START.to_string() + &content + HTML_END);
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
