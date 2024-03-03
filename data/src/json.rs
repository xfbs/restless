use crate::{Decodable, Encodable};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_slice, to_vec, Error};
use std::borrow::Cow;

/// Encode and decode data as JSON using `serde_json`.
#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

impl<T: Serialize> Encodable for Json<T> {
    type Error = Error;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        to_vec(&self.0)
    }

    fn content_type(&self) -> Option<Cow<'_, str>> {
        Some("application/json".into())
    }
}

impl<T: DeserializeOwned> Decodable for Json<T> {
    type Target = T;
    type Error = Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        from_slice(data)
    }
}
