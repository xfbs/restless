use super::*;

/// HEAD [`Request`] method.
pub trait HeadRequest: Sized {
    type Query: ToQuery;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;

    fn request(self) -> Head<Self> {
        self.into()
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
