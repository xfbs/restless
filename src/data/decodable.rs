#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "serde")]
use serde::de::DeserializeOwned;
use std::{convert::Infallible, error::Error};

/// Determines how a response is decoded.
pub trait Decodable: Sized {
    type Target;
    type Error: Error + Sync + Send + 'static;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error>;

    #[cfg(feature = "bytes")]
    fn decode_bytes(data: Bytes) -> Result<Self::Target, Self::Error> {
        Self::decode(&data[..])
    }
}

#[cfg(feature = "json")]
impl<T: DeserializeOwned> Decodable for super::Json<T> {
    type Target = T;
    type Error = serde_json::Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        serde_json::from_slice(data)
    }
}

#[cfg(feature = "json")]
impl Decodable for serde_json::Value {
    type Target = Self;
    type Error = serde_json::Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        serde_json::from_slice(data)
    }
}

#[cfg(feature = "yaml")]
impl<T: DeserializeOwned> Decodable for super::Yaml<T> {
    type Target = T;
    type Error = serde_yaml::Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        serde_yaml::from_slice(data)
    }
}

#[cfg(feature = "yaml")]
impl Decodable for serde_yaml::Value {
    type Target = Self;
    type Error = serde_yaml::Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        serde_yaml::from_slice(data)
    }
}

#[cfg(feature = "bytes")]
impl Decodable for Bytes {
    type Target = Bytes;
    type Error = Infallible;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        Ok(data.to_vec().into())
    }

    fn decode_bytes(data: Bytes) -> Result<Self::Target, Self::Error> {
        Ok(data.clone())
    }
}

#[cfg(feature = "postcard")]
impl<T: DeserializeOwned> Decodable for super::Postcard<T> {
    type Target = T;
    type Error = postcard::Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        postcard::from_bytes(data)
    }
}

#[cfg(feature = "bincode")]
impl<T: DeserializeOwned> Decodable for super::Bincode<T> {
    type Target = T;
    type Error = bincode::Error;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        bincode::deserialize(data)
    }
}

impl Decodable for () {
    type Target = ();
    type Error = Infallible;

    fn decode(_data: &[u8]) -> Result<Self::Target, Self::Error> {
        // TODO: raise error on non empty body?
        Ok(())
    }
}

impl Decodable for Vec<u8> {
    type Target = Self;
    type Error = Infallible;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        Ok(data.to_vec())
    }
}
