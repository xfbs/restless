use serde::Serialize;
use restless_core::ToQuery;
use std::borrow::Cow;

/// Encode and decode query strings using `serde_urlencoded`.
#[derive(Clone, Debug)]
pub struct Urlencoded<T: Serialize>(pub T);

impl<T: Serialize> From<T> for Urlencoded<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Serialize> ToQuery for Urlencoded<T> {
    fn encode(&self) -> Cow<'_, str> {
        serde_urlencoded::to_string(&self.0).unwrap().into()
    }
}
