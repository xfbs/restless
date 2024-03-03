use crate::{Decodable, Encodable};
use miniserde::{Serialize, Deserialize, json::{from_str, to_string}, Error};
use std::borrow::Cow;

/// Encode and decode data as JSON using `serde_json`.
#[derive(Clone, Debug)]
pub struct MiniserdeJson<T>(pub T);

impl<T: Serialize> Encodable for MiniserdeJson<T> {
    fn encode(&self) -> Vec<u8> {
        to_string(&self.0).into()
    }

    fn content_type(&self) -> Option<Cow<'_, str>> {
        Some("application/json".into())
    }
}

impl<T: Deserialize> Decodable for MiniserdeJson<T> {
    type Target = T;
    type Error = Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        from_str(std::str::from_utf8(data).unwrap())
    }
}
