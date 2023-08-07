use super::*;

pub trait GetRequest: Sized {
    type Response: ResponseEncoding;
    type Query: QueryEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;
    fn request(self) -> Get<Self> {
        Get(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Get<T: GetRequest>(T);

impl<T: GetRequest> Request for Get<T> {
    type Request = ();
    type Response = T::Response;
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.0.path()
    }

    fn body(&self) -> Self::Request {}

    fn query(&self) -> Self::Query {
        self.0.query()
    }

    fn method(&self) -> Method {
        Method::Get
    }
}
