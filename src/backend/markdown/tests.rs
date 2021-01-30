//! Snapshot tests

use super::*;
use pulldown_cmark::CowStr;

#[test]
fn simple_text() {
    let mut callbacks = MarkdownCallbacks::default();
    let mut res = String::new();
    callbacks.encode(
        &mut res,
        vec![Event::Text(CowStr::Borrowed("hello world !"))],
    );
    insta::assert_display_snapshot!(res)
}

#[test]
fn code_text() {
    let mut callbacks = MarkdownCallbacks::default();
    let mut res = String::new();
    callbacks.encode(
        &mut res,
        vec![Event::Code(CowStr::Borrowed("hello code !"))],
    );
    insta::assert_display_snapshot!(res)
}
