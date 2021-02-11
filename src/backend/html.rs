use super::{Callbacks, Event, Method, Property, Resolver};

/// Implementation of [`Callbacks`] for html.
#[derive(Default)]
pub(crate) struct HtmlCallbacks {}

impl Callbacks for HtmlCallbacks {
    fn extension(&self) -> &'static str {
        "html"
    }

    fn start_file(&mut self, s: &mut String) {
        s.push_str(
            r#"<!DOCTYPE HTML>
<html>

<head>
<meta charset="utf-8" />
<link rel="stylesheet" href="./prism.css"/>
<link rel="stylesheet" href="./style.css"/>
</head>

<body>
"#,
        )
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

    fn finish_encoding(&mut self, s: &mut String) {
        s.push_str(
            r#"<script src="./prism.js"></script>
</body>

</html>
"#,
        )
    }
}
