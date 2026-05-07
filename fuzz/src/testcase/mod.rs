use std::env;

mod html5ever;
mod old_html5gum;

pub fn run(s: &[u8]) {
    let mut did_anything = env::var("FUZZ_BASIC").unwrap_or_default() == "1";

    let testing_tokenizer = html5gum::Tokenizer::new(s).infallible();
    for _ in testing_tokenizer {}
    did_anything = true;

    if env::var("FUZZ_OLD_HTML5GUM").unwrap_or_default() == "1" {
        if let Ok(data) = std::str::from_utf8(s) {
            old_html5gum::run_old_html5gum(data);
        }
    }

    if env::var("FUZZ_HTML5EVER").unwrap_or_default() == "1" {
        if let Ok(data) = std::str::from_utf8(s) {
            html5ever::run_html5ever(data);
        }
    }

    let _ = did_anything;
}
