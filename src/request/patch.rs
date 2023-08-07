use super::*;

pub trait PatchRequest: Sized {
    type Request: RequestEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn body(&self) -> Self::Request;

    fn request(self) -> Patch<Self> {
        Patch(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Patch<T: PatchRequest>(T);

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
