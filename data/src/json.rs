use crate::{Decodable, Encodable};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_slice, to_vec, Error};
use std::borrow::Cow;

/// Encode and decode data as JSON using `serde_json`.
#[cfg(any(doc, feature = "json"))]
#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

impl<T: Serialize> Encodable for Json<T> {
    fn encode(&self) -> Vec<u8> {
        to_vec(&self.0).unwrap()
    }

    fn content_type(&self) -> Option<Cow<'_, str>> {
        Some("application/json".into())
    }
}

impl<T: DeserializeOwned> Decodable for super::Json<T> {
    type Target = T;
    type Error = Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        from_slice(data)
    }
}
