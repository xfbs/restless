use super::*;

/// GET request.
///
/// The GET method requests a representation of the specified resource. Requests using GET should
/// only retrieve data.
pub trait GetRequest: Sized {
    /// Response type and encoding.
    type Response: ResponseEncoding;
    /// Query type.
    type Query: QueryEncoding;

    /// URI path.
    fn path(&self) -> Cow<'_, str>;

    /// Query parameters.
    fn query(&self) -> Self::Query;

    /// Turn this into a [`Request`].
    fn into_request(self) -> Get<Self> {
        Get(self)
    }

    /// Turn this into a borrowed [`Request`].
    fn as_request(&self) -> Get<&Self> {
        Get(self)
    }
}

impl<T: GetRequest> GetRequest for &T {
    type Response = T::Response;
    type Query = T::Query;

    fn path(&self) -> Cow<'_, str> {
        self.path()
    }

    fn query(&self) -> Self::Query {
        self.query()
    }
}

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
