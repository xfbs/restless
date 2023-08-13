# restless

[![ci status](https://gitlab.com/xfbs/restless/badges/main/pipeline.svg)](https://gitlab.com/xfbs/restless/-/pipelines)
[![docs nightly](https://img.shields.io/badge/docs-nightly-blue)](https://xfbs.gitlab.io/restless/rustdoc/restless/)
[![docs latest](https://img.shields.io/badge/docs-latest-blue)](https://docs.rs/restless)
[![crate version](https://img.shields.io/crates/v/restless.svg)](https://crates.io/crates/restless)
[![test coverage](https://img.shields.io/badge/coverage-nightly-blue)](https://xfbs.gitlab.io/restless/coverage/)

REST API wrappers with less pain. This trait exposes helper traits that let you
define your API in terms of trait implementations. Useful to remove redundant
code if you use Rust on the backend and then frontend. Similar to standards such as
OpenAPI, but uses the Rust trait system. The trait implementation gives you API
clients for free.

## Examples

If you build a useful type that encapsulates all information about your REST request, such as this:

```rust
/// GET /api/v1/users/:name/documents/:id
pub struct UserDocument {
    user: String,
    document: Uuid,
}
```

Then you can implement the `GetRequest` trait for this type and use the `RequestMethod` marker.

```rust
use restless::{GetRequest, method::Get};

impl GetRequest for UserDocument {
    type Response = Json<Document>;
    type Query = ();

    fn path(&self) -> Cow<'_, str> {
        format!("api/v1/users/{}/documents/{}", self.user, self.document).into()
    }

    fn query(&self) -> Self::Query {}
}

impl RequestMethod for UserDocument {
    type Method = Get<Self>;
}
```

Now you can make this kind of request using the built-in clients.

## Clients

This crate offers integrations with the following HTTP clients and frameworks by enabling the corresponding feature:

| Name | Description |
| ---- | ----------- |
| [reqwest](https://docs.rs/reqwest/latest/reqwest/) | Make HTTP requests using the `reqwest` crate. |
| [gloo](https://docs.rs/gloo-net/latest/gloo_net/) | Make HTTP requests in WebAssembly Rust applications using the `gloo_net` crate. |
| [yew](https://docs.rs/yew/latest/yew/) | Make HTTP requests from within Yew frontend components. |

## License

MIT, see [LICENSE.md](LICENSE.md)
