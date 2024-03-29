//! Traits to encode and decode request and response bodies.

#[cfg(feature = "bytes")]
use bytes::Bytes;
use std::borrow::Cow;
use std::{convert::Infallible, error::Error};

/// Determines how a response is decoded.
pub trait Decodable: Sized {
    /// Target type to decode to.
    type Target;

    /// Error which can occur when attempting to decode data.
    type Error: Error + Sync + Send + 'static;

    /// Decode data from a byte slice.
    fn decode(data: &[u8]) -> Result<Self::Target, Self::Error>;

    #[cfg(feature = "bytes")]
    /// Decode data from [`Bytes`].
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

/// Error decoding a body which should be empty.
#[derive(thiserror::Error, Debug)]
pub enum EmptyDecodeError {
    /// Body is not empty.
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
    /// Error that can be returned encoding data.
    type Error: Error + Sync + Send + 'static;

    /// Encode self into a byte array.
    fn encode(&self) -> Result<Vec<u8>, Self::Error>;

    #[cfg(feature = "bytes")]
    /// Encode self into [`Bytes`].
    fn encode_bytes(&self) -> Result<Bytes, Self::Error> {
        self.encode().map(Into::into)
    }

    /// Content type of this data.
    fn content_type(&self) -> Option<Cow<'_, str>> {
        None
    }
}

impl Encodable for Vec<u8> {
    type Error = Infallible;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.clone())
    }
}

impl Encodable for () {
    type Error = Infallible;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(Vec::new())
    }
}

#[cfg(feature = "bytes")]
impl Encodable for Bytes {
    type Error = Infallible;

    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.to_vec())
    }

    fn encode_bytes(&self) -> Result<Bytes, Self::Error> {
        Ok(self.clone())
    }
}
