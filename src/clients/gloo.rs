use crate::*;
use async_trait::async_trait;
use gloo_net::{
    http::{self as gloo, RequestBuilder, Response},
    Error as GlooError,
};
#[cfg(feature = "serde")]
use serde::{de::DeserializeOwned, Serialize};
use std::{error::Error, fmt::Debug};

/// Error decoding a gloo [`Response`].
#[derive(thiserror::Error, Debug)]
pub enum GlooRequestError<D: Error + Debug> {
    /// Error from gloo.
    #[error("getting response")]
    Request(#[from] gloo_net::Error),
    /// Error during decoding of response body.
    #[error("decoding response")]
    Decode(#[source] D),
}

pub type ResponseError<T> = GlooRequestError<T>;

/// Decodable from a gloo [`Response`].
#[async_trait(?Send)]
pub trait GlooResponse {
    /// Type that this decodes into.
    type Target;
    /// Error that this type can return during decoding.
    type Error: Error + Debug;

    /// Method to decode a gloo [`Response`].
    async fn from_gloo_response(
        response: &Response,
    ) -> Result<Self::Target, ResponseError<Self::Error>>;
}

#[async_trait(?Send)]
impl<T: DeserializeOwned + Clone + 'static> GlooResponse for Json<T> {
    type Target = T;
    type Error = std::convert::Infallible;
    async fn from_gloo_response(
        response: &Response,
    ) -> Result<Self::Target, ResponseError<Self::Error>> {
        response.json::<T>().await.map_err(ResponseError::Request)
    }
}

/// Decode this type using the [`Decodable`] trait via bytes.
pub trait GlooResponseBinary: Decodable {}

#[async_trait(?Send)]
impl<T: GlooResponseBinary> GlooResponse for T {
    type Target = <T as Decodable>::Target;
    type Error = <T as Decodable>::Error;
    async fn from_gloo_response(
        response: &Response,
    ) -> Result<Self::Target, ResponseError<Self::Error>> {
        let data = response.binary().await.map_err(ResponseError::Request)?;
        T::decode(&data).map_err(ResponseError::Decode)
    }
}

impl GlooResponseBinary for () {}
impl GlooResponseBinary for Vec<u8> {}

pub trait ToGlooRequest {
    fn to_gloo_request(&self, request: RequestBuilder) -> Result<gloo::Request, GlooError>;
}

impl ToGlooRequest for () {
    fn to_gloo_request(&self, builder: RequestBuilder) -> Result<gloo::Request, GlooError> {
        builder.build().map_err(Into::into)
    }
}

impl<T: Serialize> ToGlooRequest for Json<T> {
    fn to_gloo_request(&self, builder: RequestBuilder) -> Result<gloo::Request, GlooError> {
        builder.json(&self.0).map_err(Into::into)
    }
}

/// Build a gloo [`RequestBuilder`] for a given HTTP [`Request`].
pub fn request_builder<R: Request>(prefix: &str, request: &R) -> RequestBuilder {
    let path = format!("{prefix}{}", request.uri());
    let method = request.method().into();
    let builder = RequestBuilder::new(&path).method(method);
    builder
}

#[async_trait(?Send)]
pub trait GlooRequest {
    type Response;
    type Error: Error + Debug;

    async fn send(&self) -> Result<Self::Response, Self::Error> {
        self.send_prefix("/").await
    }

    async fn send_prefix(&self, prefix: &str) -> Result<Self::Response, Self::Error>;
}

#[async_trait(?Send)]
impl<T: Request> GlooRequest for T
where
    T::Response: GlooResponse,
    T::Request: ToGlooRequest,
    <T::Response as GlooResponse>::Error: 'static,
{
    type Response = <T::Response as GlooResponse>::Target;
    type Error = GlooRequestError<<T::Response as GlooResponse>::Error>;
    async fn send_prefix(&self, prefix: &str) -> Result<Self::Response, Self::Error> {
        let builder = request_builder(prefix, self);
        let request = self.body().to_gloo_request(builder)?;
        let response = request.send().await?;
        <T::Response as GlooResponse>::from_gloo_response(&response).await
    }
}
