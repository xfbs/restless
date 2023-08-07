#[cfg(feature = "serde")]
#[allow(unused_imports)]
use serde::Serialize;
use std::borrow::Cow;

pub trait QueryEncoding {
    fn encode(&self) -> Cow<'_, str>;
}

impl QueryEncoding for () {
    fn encode(&self) -> Cow<'_, str> {
        "".into()
    }
}

impl QueryEncoding for String {
    fn encode(&self) -> Cow<'_, str> {
        self.as_str().into()
    }
}

impl QueryEncoding for &str {
    fn encode(&self) -> Cow<'_, str> {
        (*self).into()
    }
}

#[cfg(feature = "urlencoded")]
#[derive(Clone, Debug)]
pub struct Urlencoded<T: Serialize>(pub T);

#[cfg(feature = "urlencoded")]
impl<T: Serialize> QueryEncoding for Urlencoded<T> {
    fn encode(&self) -> Cow<'_, str> {
        serde_urlencoded::to_string(&self.0).unwrap().into()
    }
}

#[cfg(feature = "qs")]
#[derive(Clone, Debug)]
pub struct Qs<T: Serialize>(pub T);

#[cfg(feature = "qs")]
impl<T: Serialize> QueryEncoding for Qs<T> {
    fn encode(&self) -> Cow<'_, str> {
        serde_qs::to_string(&self.0).unwrap().into()
    }
}
