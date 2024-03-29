use crate::{Decodable, Encodable};
use bincode::{deserialize, serialize, Error};
use serde::{de::DeserializeOwned, Serialize};

/// Encode and decode data as bincode using `bincode`.
#[derive(Clone, Debug)]
pub struct Bincode<T>(pub T);

impl<T: Serialize> Encodable for Bincode<T> {
    type Error = Error;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        serialize(&self.0)
    }
}

impl<T: DeserializeOwned> Decodable for Bincode<T> {
    type Target = T;
    type Error = Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        deserialize(data)
    }
}
