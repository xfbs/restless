use crate::encoding::*;
use std::borrow::Cow;

mod post;
pub use post::*;
mod get;
pub use get::*;
mod patch;
pub use patch::*;
mod delete;
pub use delete::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    Post,
    Get,
    Delete,
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
