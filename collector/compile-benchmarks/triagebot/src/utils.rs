use std::borrow::Cow;

/// Pluralize (add an 's' sufix) to `text` based on `count`.
pub fn pluralize(text: &str, count: usize) -> Cow<str> {
    if count == 1 {
        text.into()
    } else {
        format!("{}s", text).into()
    }
}
