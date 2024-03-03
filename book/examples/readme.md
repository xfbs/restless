# Examples

This chapter shows you, based on some examples, how you can express your HTTP APIs
using the `restless` crate.

The basic workflow is always the same: you represent every possible API request
as a data structure, and you implement the `restless::Request` trait for this
data structure. This tells `restless` how to perform the request and how to
interpret the response.

## Wrapper Traits

Typically, you do not implement `Request` directly, but you use some of the wrapper
traits for the request method you want, for example `GetRequest` for a HTTP GET
request or `PostRequest` for an HTTP POST request.

```rust
# extern crate restless;
# extern crate serde;
use restless::{GetRequest, RequestMethod, methods::Get, data::Json, query::Qs};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyResponse {
    string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyRequest {
    string: String,
}

impl GetRequest for MyRequest {
    type Response = Json<MyResponse>;
    type Query = Qs<Self>;

    fn path(&self) -> Cow<'_, str> {
        "/api/v1/path".into()
    }

    fn query(&self) -> Self::Query {
        Qs(self.clone())
    }
}

impl RequestMethod for MyRequest {
    type Method = Get<Self>;
}
```

## Implementing Request

However, if you need to you can always implement the `Request` trait directly. Here
is how that looks like:

```rust
# extern crate restless;
# extern crate serde;
# extern crate bytes;
use restless::{Request, Method, data::Json, query::Qs};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use bytes::Bytes;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MyRequest {}

impl Request for MyRequest {
    type Response = Bytes;
    type Request = ();
    type Query = String;

    fn path(&self) -> Cow<'_, str> {
        "/api/v1/path".into()
    }

    fn body(&self) -> Self::Request {
        ()
    }

    fn query(&self) -> Self::Query {
        "?query=value".into()
    }

    fn method(&self) -> Method {
        Method::Get
    }
}
```
