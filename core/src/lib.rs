//! # restless core
//!
//! Core traits used by restless, crate which allows you to express HTTP request using the Rust
//! type system.
//!
//! If you are using restless, then you should be using the `restless` crate and not this one. This
//! crate is useful if you want to build plugins for `restless`.
#![warn(missing_docs)]

use std::borrow::Cow;

pub mod query;
use query::ToQuery;

pub mod methods;
use methods::*;

pub mod data;
pub use data::{Decodable, Encodable};

mod get;
pub use get::*;
mod head;
pub use head::*;
mod post;
pub use post::*;
mod patch;
pub use patch::*;
mod delete;
pub use delete::*;

/// HTTP Method.
///
/// HTTP defines a set of request methods. These define if a request mutates something, if it can
/// be cached, if it is idempotent and the semantics of the request.  See [HTTP request
/// methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods) for an explainer of these.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    /// Requests a representation of the specified resource. Should only retrieve data.
    Get,
    /// Similar to a [`Get`] request, but without the response body. Only retrieves headers.
    Head,
    /// Submits an entry to the specified resource. Often causes a change in state or side-effects.
    Post,
    /// Replaces the target resource with the request payload. Is idempotent.
    Put,
    /// Deletes the specified resource on the server.
    Delete,
    /// Establishes a tunnel to the server identified by the target resource.
    Connect,
    /// Describes the communication options for the target resource.
    Options,
    /// Performs a message loop-back test to the target resource.
    Trace,
    /// Applies partial modifications to the target resource.
    Patch,
}

#[cfg(feature = "http")]
impl From<Method> for http::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => http::Method::GET,
            Method::Head => http::Method::HEAD,
            Method::Post => http::Method::POST,
            Method::Put => http::Method::PUT,
            Method::Delete => http::Method::DELETE,
            Method::Connect => http::Method::CONNECT,
            Method::Options => http::Method::OPTIONS,
            Method::Trace => http::Method::TRACE,
            Method::Patch => http::Method::PATCH,
        }
    }
}

/// HTTP Request.
///
/// This trait defines an arbitrary HTTP request.
pub trait Request {
    /// Request body.
    type Request: Encodable;
    /// Response body.
    type Response: Decodable;
    /// Query string.
    type Query: ToQuery;

    /// URI path.
    fn path(&self) -> Cow<'_, str>;
    /// Request body.
    fn body(&self) -> Self::Request;
    /// Query.
    fn query(&self) -> Self::Query;
    /// HTTP method.
    fn method(&self) -> Method;

    /// Build URI for this request.
    fn uri(&self) -> String {
        let mut path = self.path().into_owned();
        let query = self.query();
        let query_string = query.encode();
        if !query_string.is_empty() {
            path.push('?');
            path.push_str(&query_string);
        }
        path
    }
}

/// Request type trait.
///
/// This allows types to implement [`Request`] by a proxy type.
pub trait RequestType: Sized {
    /// Underlying type that implements [`Request`].
    type Request: Request;

    /// Borrow a reference to the underlying [`Request`] type.
    fn borrow(&self) -> &Self::Request;
}

/// Specify the [`Request`] method that should be used for a type.
///
/// Due to the way the Rust type system works, you can implement both [`PostRequest`] and
/// [`GetRequest`] for the same type. To be able to implement [`Request`] for your type, this trait
/// helps by allowing you to specify which request type should be used.
///
/// For example:
///
/// ```rust
/// use restless_core::*;
/// use std::borrow::Cow;
///
/// struct MyRequest;
///
/// impl GetRequest for MyRequest {
///     type Response = Vec<u8>;
///     type Query = ();
///
///     fn path(&self) -> Cow<'_, str> {
///         "api/v1/request".into()
///     }
///
///     fn query(&self) -> Self::Query {}
/// }
///
/// impl RequestMethod for MyRequest {
///     type Method = methods::Get<Self>;
/// }
/// ```
pub trait RequestMethod: Sized
where
    Self::Method: From<Self>,
    for<'a> &'a Self::Method: From<&'a Self>,
{
    /// Method to use for this request.
    type Method: Request;

    /// Borrow a [`Request`].
    fn as_request(&self) -> &Self::Method {
        self.into()
    }

    /// Turn this into an owned [`Request`].
    fn into_request(self) -> Self::Method {
        self.into()
    }
}

impl<T: RequestMethod> RequestType for T
where
    for<'a> &'a <T as RequestMethod>::Method: From<&'a T>,
{
    type Request = <T as RequestMethod>::Method;

    fn borrow(&self) -> &Self::Request {
        self.into()
    }
}

impl<T: RequestType> Request for T {
    type Request = <<T as RequestType>::Request as Request>::Request;
    type Response = <<T as RequestType>::Request as Request>::Response;
    type Query = <<T as RequestType>::Request as Request>::Query;

    fn path(&self) -> Cow<'_, str> {
        self.borrow().path()
    }

    fn body(&self) -> Self::Request {
        self.borrow().body()
    }

    fn query(&self) -> Self::Query {
        self.borrow().query()
    }

    fn method(&self) -> Method {
        self.borrow().method()
    }
}
