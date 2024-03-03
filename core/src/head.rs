use super::*;

/// HEAD [`Request`] method.
pub trait HeadRequest: Sized {
    /// Query data type.
    type Query: ToQuery;

    /// Path of request.
    fn path(&self) -> Cow<'_, str>;

    /// Query type.
    fn query(&self) -> Self::Query;

    /// Turn self into a [`Request`].
    fn request(self) -> Head<Self> {
        self.into()
    }
}

impl<T: HeadRequest> HeadRequest for &T {
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        <T as HeadRequest>::path(self)
    }

    fn query(&self) -> Self::Query {
        <T as HeadRequest>::query(self)
    }
}

impl<T: HeadRequest> Request for Head<T> {
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
        Method::Head
    }
}
