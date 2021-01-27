use super::{Callbacks, Resolver, Event, Method};

/// Implementation of [`Callbacks`] for html.
#[derive(Default)]
pub(crate) struct HtmlCallbacks {}

impl Callbacks for HtmlCallbacks {
    fn extension(&self) -> &'static str {
        "html"
    }

    fn start_method(&mut self, s: &mut String, config: &Resolver, method: &Method) {
        (self as &mut dyn Callbacks).start_method_default(s, config, method)
    }

    fn encode(&mut self, s: &mut String, events: Vec<Event<'_>>) {
        pulldown_cmark::html::push_html(s, events.into_iter())
    }
}
