use super::*;
use crate::methods::Delete;

/// DELETE [`Request`] method.
pub trait DeleteRequest: Sized {
    type Query: ToQuery;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;

    fn request(self) -> Delete<Self> {
        self.into()
    }
}

impl<T: DeleteRequest> DeleteRequest for &T {
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        <T as DeleteRequest>::path(self)
    }

    fn query(&self) -> Self::Query {
        <T as DeleteRequest>::query(self)
    }
}

impl<T: DeleteRequest> Request for Delete<T> {
    type Request = ();
    type Response = ();
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.inner.path()
    }

    fn body(&self) -> Self::Request {}

    fn query(&self) -> Self::Query {
        self.inner.query()
    }

    fn method(&self) -> Method {
        Method::Delete
    }
}
