use restless_core::{Method, Request};
use std::borrow::Cow;

/// Maps a request to prefix the path with a string.
#[derive(Clone, Debug)]
pub struct PathPrefix<R: Request + ?Sized, S: AsRef<str>> {
    prefix: S,
    request: R,
}

impl<R: Request + ?Sized, S: AsRef<str>> Request for PathPrefix<R, S> {
    type Request = R::Request;
    type Response = R::Response;
    type Query = R::Query;

    fn path(&self) -> Cow<'_, str> {
        let mut prefix = self.prefix.as_ref().to_string();
        prefix.push_str(&self.request.path());
        prefix.into()
    }

    fn body(&self) -> Self::Request {
        self.request.body()
    }

    fn query(&self) -> Self::Query {
        self.request.query()
    }

    fn method(&self) -> Method {
        self.request.method()
    }
}

pub trait RequestExt: Request {
    fn prefix(self, string: &str) -> PathPrefix<Self, String>;
}

impl<R: Request> RequestExt for R {
    fn prefix(self, string: &str) -> PathPrefix<Self, String> {
        PathPrefix {
            prefix: string.into(),
            request: self,
        }
    }
}
