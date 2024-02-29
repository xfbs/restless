#[cfg(feature = "bytes")]
use bytes::Bytes;
use std::borrow::Cow;
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

#[derive(thiserror::Error, Debug)]
pub enum EmptyDecodeError {
    #[error("body is not empty")]
    NotEmpty,
}

impl Decodable for () {
    type Target = ();
    type Error = EmptyDecodeError;

    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error> {
        if !data.is_empty() {
            return Err(EmptyDecodeError::NotEmpty);
        }

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

/// Determines how a request is encoded.
pub trait Encodable {
    fn encode(&self) -> Vec<u8>;

    #[cfg(feature = "bytes")]
    fn encode_bytes(&self) -> Bytes {
        self.encode().into()
    }

    fn content_type(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl Encodable for () {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(feature = "bytes")]
impl Encodable for Bytes {
    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn encode_bytes(&self) -> Bytes {
        self.clone()
    }
}
