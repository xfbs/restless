//! Traits to encode URI query parameters.

use std::borrow::Cow;

/// Determines how a query string is encoded.
pub trait ToQuery {
    /// Encode self into a query string.
    fn encode(&self) -> Cow<'_, str>;
}

impl ToQuery for () {
    fn encode(&self) -> Cow<'_, str> {
        "".into()
    }
}

impl ToQuery for String {
    fn encode(&self) -> Cow<'_, str> {
        self.as_str().into()
    }
}

impl ToQuery for &str {
    fn encode(&self) -> Cow<'_, str> {
        (*self).into()
    }
}
