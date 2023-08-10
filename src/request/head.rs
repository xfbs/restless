use super::*;

pub trait HeadRequest: Sized {
    type Query: QueryEncoding;

    fn path(&self) -> Cow<'_, str>;
    fn query(&self) -> Self::Query;
    fn request(self) -> Head<Self> {
        Head(self)
    }
}

impl<T: HeadRequest> Request for Head<T> {
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
        Method::Head
    }
}
