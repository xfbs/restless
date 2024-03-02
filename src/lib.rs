//! # restless
//!
//! This is a crate that helps you specify your REST API in a typesafe manner. This is somewhat
//! similar to standards such as [OpenAPI](https://www.openapis.org/), which allow you to specify
//! your API in a cross-language manner. However, if both your frontend and your backend are
//! written in Rust, it is not neccessary to go through the trouble of specifying your APIs in a
//! spec like this. Rather, you can use the trait system to define your requests like this:
//!
//! ```rust
//! use restless::{*, data::Json};
//! use std::borrow::Cow;
//!
//! struct MyRequest {
//!     name: String,
//! }
//!
//! impl GetRequest for MyRequest {
//!     type Response = Json<Vec<String>>;
//!     type Query = ();
//!
//!     fn query(&self) -> Self::Query {}
//!     fn path(&self) -> Cow<'_, str> {
//!         "api/v1/my-request".into()
//!     }
//! }
//! ```
//!
//! In this case, the response type is a JSON-encoded `Vec<String>`. Under the hood, `serde_json`
//! is used to provide this encoding. This crate ships with other encoding strategies, such as
//! using [`Yaml`](data::Yaml) or [`Bincode`](data::Bincode). You can also specify
//! [`Bytes`](bytes::Bytes) to get the raw data. By implementing the [`Decodable`](data::Decodable)
//! trait, you can augment it with your own, custom decoding strategy. The same goes for request
//! bodies with the [`Encodable`](data::Encodable) trait.
//!
//! ## Clients
//!
//! What you get "for free" from this crate is implementations of various clients. See the
//! [`clients`] module for more information.
//!
//! ## Examples
//!
//! To see some examples for how this crate may be used, refer to the `examples/` directory in the
//! [repository](https://github.com/xfbs/restless).

pub mod clients {
    #[cfg(feature = "gloo")]
    pub use restless_gloo::*;

    #[cfg(feature = "yew")]
    pub use restless_yew::*;

    #[cfg(feature = "hyper")]
    pub use restless_hyper::*;

    #[cfg(feature = "axum")]
    pub use restless_axum::*;

    #[cfg(feature = "wasm-cache")]
    pub use restless_wasm_cache::*;
}

/// Encoding traits and types.
///
/// Used to modify how request and response bodies are encoded and decoded.
pub mod data {
    pub use restless_core::{Decodable, Encodable};
    pub use restless_data::*;
}

pub mod methods {
    pub use restless_core::methods::*;
}

pub mod query {
    pub use restless_query::*;
}

pub use restless_core::{
    DeleteRequest, GetRequest, HeadRequest, Method, PatchRequest, PostRequest, Request,
    RequestMethod, RequestType,
};
