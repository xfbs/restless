use super::*;

/// GET [`Request`] method.
///
/// The GET method requests a representation of the specified resource. Requests using GET should
/// only retrieve data.
pub trait GetRequest: Sized {
    /// Response type and encoding.
    type Response: Decodable;
    /// Query type.
    type Query: ToQuery;

    /// URI path.
    fn path(&self) -> Cow<'_, str>;

    /// Query parameters.
    fn query(&self) -> Self::Query;
}

impl<T: GetRequest> GetRequest for &T {
    type Response = T::Response;
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        <T as GetRequest>::path(self)
    }

    fn query(&self) -> Self::Query {
        <T as GetRequest>::query(self)
    }
}

impl<T: GetRequest> Request for Get<T> {
    type Request = ();
    type Response = T::Response;
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.inner.path()
    }

    fn body(&self) -> Self::Request {}

    fn query(&self) -> Self::Query {
        self.inner.query()
    }

    fn method(&self) -> Method {
        Method::Get
    }
}
