use super::*;

pub trait DeleteRequest: Sized {
    type Query: QueryEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;
    fn request(self) -> Delete<Self> {
        Delete(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Delete<T: DeleteRequest>(T);

impl<T: DeleteRequest> Request for Delete<T> {
    type Request = ();
    type Response = ();
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.0.path()
    }

    fn body(&self) -> Self::Request {}

    fn query(&self) -> Self::Query {
        self.0.query()
    }

    fn method(&self) -> Method {
        Method::Delete
    }
}
