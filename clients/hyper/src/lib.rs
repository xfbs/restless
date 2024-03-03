//! # restless hyper
//!
//! Integration of restless with the hyper crate.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

use ::http::{Error, Method, Request as HttpRequest};
pub use hyper;
use hyper::body::Body;
use restless_core::{Encodable, Request};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Error turning a restless [`Request`] into a hyper [`Request`](http::Request).
#[derive(thiserror::Error, Debug)]
pub enum HyperRequestError {
    /// Error in body
    #[error(transparent)]
    Body(#[from] Error),
    /// Error encoding
    #[error(transparent)]
    Encoding(BoxError),
}

/// Turn a restless [`Request`] into a hyper [`Request`](http::Request).
pub trait HyperRequest {
    /// Turn into hyper request with a custom prefix.
    fn to_hyper_request_prefix(&self, prefix: &str)
        -> Result<HttpRequest<Body>, HyperRequestError>;

    /// Turn into hyper request using `/` as the prefix.
    fn to_hyper_request(&self) -> Result<HttpRequest<Body>, HyperRequestError> {
        self.to_hyper_request_prefix("/")
    }
}

fn to_hyper_request<T: Request>(
    request: &T,
    prefix: &str,
) -> Result<HttpRequest<Body>, HyperRequestError> {
    let builder = HttpRequest::builder()
        .uri(&format!("{prefix}{}", request.uri()))
        .method::<Method>(request.method().into());
    let body = request.body();
    let builder = match body.content_type() {
        Some(content_type) => builder.header("content-type", &*content_type),
        None => builder,
    };
    let request = builder.body::<Body>(
        request
            .body()
            .encode_bytes()
            .map_err(|error| Box::new(error) as BoxError)
            .map_err(HyperRequestError::Encoding)?
            .into(),
    )?;
    Ok(request)
}

impl<T: Request> HyperRequest for T {
    fn to_hyper_request_prefix(
        &self,
        prefix: &str,
    ) -> Result<HttpRequest<Body>, HyperRequestError> {
        to_hyper_request(self, prefix)
    }
}
