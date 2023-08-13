use crate::{Encodable, Request};
use ::http::{Error, Method, Request as HttpRequest};
use hyper::body::Body;

/// Error turning a restless [`Request`] into a hyper [`Request`](http::Request).
#[derive(thiserror::Error, Debug)]
pub enum HyperRequestError {
    #[error(transparent)]
    Body(#[from] Error),
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
    let request = builder.body::<Body>(request.body().encode_bytes().into())?;
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
