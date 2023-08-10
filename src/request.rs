use crate::{encoding::*, wrappers::*};
use std::borrow::Cow;

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
    /// Requests a representation of the specified resoruce. Should only retrieve data.
    Get,
    /// Similar to a [`Get`] request, but without the response body. Only retrieves headers.
    Head,
    /// Submits an entry to the specified resource. Often causes a change in state or side-effects.
    Post,
    /// Plraces the target resource with the request payload. Is idempotent.
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

pub trait Request {
    type Request: RequestEncoding;
    type Response: ResponseEncoding;
    type Query: QueryEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn body(&self) -> Self::Request;
    fn query(&self) -> Self::Query;
    fn method(&self) -> Method;

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
