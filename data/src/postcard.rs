use crate::{Decodable, Encodable};
use postcard::{from_bytes, to_allocvec, Error};
use serde::{de::DeserializeOwned, Serialize};

/// Encode and decode data as postcard using `postcard`.
#[derive(Clone, Debug)]
pub struct Postcard<T>(pub T);

impl<T: Serialize> Encodable for Postcard<T> {
    fn encode(&self) -> Vec<u8> {
        to_allocvec(&self.0).unwrap()
    }
}

impl<T: DeserializeOwned> Decodable for Postcard<T> {
    type Target = T;
    type Error = Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        from_bytes(data)
    }
}
