use super::*;

/// POST [`Request`] method.
pub trait PostRequest: Sized {
    type Request: Encodable;

    fn path(&self) -> Cow<'_, str>;
    fn body(&self) -> Self::Request;
}

impl<T: PostRequest> PostRequest for &T {
    type Request = T::Request;

    fn path(&self) -> Cow<'_, str> {
        <T as PostRequest>::path(self)
    }

    fn body(&self) -> Self::Request {
        <T as PostRequest>::body(self)
    }
}

impl<T: PostRequest> Request for Post<T> {
    type Request = T::Request;
    type Response = ();
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        self.inner.path()
    }

    fn body(&self) -> Self::Request {
        self.inner.body()
    }

    fn query(&self) -> Self::Query {}

    fn method(&self) -> Method {
        Method::Post
    }
}
