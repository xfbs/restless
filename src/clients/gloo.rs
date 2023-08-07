use crate::*;
use gloo_net::{
    http::{Request as GlooRequest, RequestBuilder, Response},
    Error as GlooError,
};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::borrow::Cow;
use url::Url;

pub trait GlooEncodable {
    fn encode(&self, request: RequestBuilder) -> Result<GlooRequest, GlooError>;
}

impl GlooEncodable for () {
    fn encode(&self, builder: RequestBuilder) -> Result<GlooRequest, GlooError> {
        builder.build()
    }
}

impl<T: Serialize> GlooEncodable for Json<T> {
    fn encode(&self, builder: RequestBuilder) -> Result<GlooRequest, GlooError> {
        builder.json(&self.0)
    }
}

pub enum Base<'a> {
    Url(Url),
    Root,
    Path(Cow<'a, str>),
}

/// Build a gloo [`RequestBuilder`] for a given HTTP [`Request`].
pub fn request_builder<R: Request>(base: Base<'_>, request: R) -> Result<RequestBuilder, ()> {
    let uri = request.uri();
    let request = RequestBuilder::new(&uri);
    Ok(request)
}

/// Decode a [`Response`] into the target type.
pub async fn decode<T: ResponseEncoding>(response: Response) -> Result<T::Target, ()> {
    todo!()
}
