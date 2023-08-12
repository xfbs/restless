use restless::*;
use std::borrow::Cow;

struct MyRequest {
    pub name: String,
}

impl GetRequest for MyRequest {
    type Response = ();
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        "api/v1/request".into()
    }

    fn query(&self) -> Self::Query {}
}

impl RequestMethod for MyRequest {
    type Method = methods::Get<Self>;
}
