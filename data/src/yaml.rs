use crate::{Decodable, Encodable};
use serde::{de::DeserializeOwned, Serialize};
use serde_yaml::{from_slice, to_string, Error};

/// Encode and decode data as YAML using `serde_yaml`.
#[derive(Clone, Debug)]
pub struct Yaml<T>(pub T);

impl<T: Serialize> Encodable for Yaml<T> {
    fn encode(&self) -> Vec<u8> {
        to_string(&self.0).unwrap().into_bytes()
    }
}

impl<T: DeserializeOwned> Decodable for Yaml<T> {
    type Target = T;
    type Error = Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        from_slice(data)
    }
}
