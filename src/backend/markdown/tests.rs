//! Snapshot tests

use super::*;

/// Encode the given text:
/// 1. Into markdown events using `pulldown_cmark::Parser`
/// 2. back to text using `MarkdownCallbacks`
fn encode(source: &str) -> String {
    let mut callbacks = MarkdownCallbacks::default();
    let mut res = String::new();
    callbacks.encode(
        &mut res,
        pulldown_cmark::Parser::new(source).into_iter().collect(),
    );
    res
}

#[test]
fn simple_text() {
    let simple = encode("hello world !");
    insta::assert_display_snapshot!(simple)
}

#[test]
fn simple_code() {
    let code = encode("`hello code !`");
    insta::assert_display_snapshot!(code)
}

#[test]
fn simple_code_block() {
    let code_block = encode(
        r#"```rust
#[test]
fn simple_code() {
    let code = encode("`hello code !`");
    insta::assert_display_snapshot!(code)
}
```"#,
    );
    insta::assert_display_snapshot!(code_block)
}

#[test]
fn simple_quote() {
    let quote = encode(
        r#"
> This is a quote
> On multiple lines.
"#,
    );
    insta::assert_display_snapshot!(quote)
}

#[test]
fn various_text_emphasis() {
    let text = encode(
        r#"
Normal text.
*Italic text*
**Bold text**
~~ Strikethrough ? ~~
"#,
    );
    insta::assert_display_snapshot!(text)
}

#[test]
fn new_paragraph() {
    let new_paragraph = encode(
        r"hello

world !",
    );
    insta::assert_display_snapshot!(new_paragraph)
}

#[test]
fn bullet_list() {
    let list = encode(
        r"
- hello
- world
    - !
- `how are` you <!--- identation can be 2 spaces -->
  - today

      Dear
  - ?",
    );
    insta::assert_display_snapshot!(list)
}

#[test]
fn numbered_list() {
    let list = encode(
        r"
1. hello
2. world
    1. !
3. `how are` you
    1. today

        Dear
    2. ?",
    );
    insta::assert_display_snapshot!(list)
}

#[test]
fn complicated_list() {
    let list = encode(
        r#"
1. Text [link-section](#section)
2. Other text:
  
    This is on a new paragraph
    - Nested bullet
    - 1. With nested
      2. Numbered list inside

    Resume on a new paragraph...
3. Last item."#,
    );
    insta::assert_display_snapshot!(list)
}

#[test]
fn quotes_and_lists() {
    let mixed = encode(
        r#"
> Quoted text
> - And now quoted list...
>     > With quoted text inside !
> - Back to lists...
>     - Nested list
- Break out
    > With a 
    > Final quote
"#,
    );
    insta::assert_display_snapshot!(mixed)
}
