extern crate restless;
extern crate serde;
use restless::{GetRequest, RequestMethod, methods::Get, data::Json, query::Qs};
use std::borrow::Cow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrateSearchResponse {
    string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrateSearchRequest {
    string: String,
}

impl GetRequest for CrateSearchRequest {
    type Response = Json<CrateSearchResponse>;
    type Query = Qs<Self>;

    fn path(&self) -> Cow<'_, str> {
        "/api/v1/search".into()
    }

    fn query(&self) -> Self::Query {
        Qs(self.clone())
    }
}

impl RequestMethod for CrateSearchRequest {
    type Method = Get<Self>;
}
