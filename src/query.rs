//! Traits to encode and decode URI query parameters.

#[cfg(feature = "serde")]
#[allow(unused_imports)]
use serde::Serialize;
use std::borrow::Cow;

/// Determines how a query string is encoded.
pub trait ToQuery {
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

/// Encode and decode query strings using `serde_urlencoded`.
#[cfg(feature = "urlencoded")]
#[derive(Clone, Debug)]
pub struct Urlencoded<T: Serialize>(pub T);

#[cfg(feature = "urlencoded")]
impl<T: Serialize> From<T> for Urlencoded<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[cfg(feature = "urlencoded")]
impl<T: Serialize> ToQuery for Urlencoded<T> {
    fn encode(&self) -> Cow<'_, str> {
        serde_urlencoded::to_string(&self.0).unwrap().into()
    }
}

/// Encode and decode query strings using `serde_qs`.
#[cfg(feature = "qs")]
#[derive(Clone, Debug)]
pub struct Qs<T: Serialize>(pub T);

#[cfg(feature = "qs")]
impl<T: Serialize> ToQuery for Qs<T> {
    fn encode(&self) -> Cow<'_, str> {
        serde_qs::to_string(&self.0).unwrap().into()
    }
}

#[cfg(feature = "qs")]
impl<T: Serialize> From<T> for Qs<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
