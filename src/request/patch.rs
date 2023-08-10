use super::*;

pub trait PatchRequest: Sized {
    type Request: RequestEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn body(&self) -> Self::Request;

    fn request(self) -> Patch<Self> {
        Patch(self)
    }
}

impl<T: PatchRequest> PatchRequest for &T {
    type Request = T::Request;

    fn path(&self) -> Cow<'_, str> {
        <T as PatchRequest>::path(self)
    }

    fn body(&self) -> Self::Request {
        <T as PatchRequest>::body(self)
    }
}

impl<T: PatchRequest> Request for Patch<T> {
    type Request = T::Request;
    type Response = ();
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        self.0.path()
    }

    fn body(&self) -> Self::Request {
        self.0.body()
    }

    fn query(&self) -> Self::Query {}

    fn method(&self) -> Method {
        Method::Patch
    }
}
