use serde::Serialize;
use restless_core::ToQuery;
use std::borrow::Cow;

/// Encode and decode query strings using `serde_qs`.
#[derive(Clone, Debug)]
pub struct Qs<T: Serialize>(pub T);

impl<T: Serialize> ToQuery for Qs<T> {
    fn encode(&self) -> Cow<'_, str> {
        serde_qs::to_string(&self.0).unwrap().into()
    }
}

impl<T: Serialize> From<T> for Qs<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
