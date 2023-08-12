//! Encoding traits and types.
//!
//! Used to modify how request and response bodies are encoded and decoded.

mod encodable;
pub use encodable::*;

mod decodable;
pub use decodable::*;

/// Encode and decode data as JSON using `serde_json`.
#[cfg(any(doc, feature = "json"))]
#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

/// Encode and decode data as YAML using `serde_yaml`.
#[cfg(any(doc, feature = "yaml"))]
#[derive(Clone, Debug)]
pub struct Yaml<T>(pub T);

/// Encode and decode data as postcard using `postcard`.
#[cfg(any(doc, feature = "postcard"))]
#[derive(Clone, Debug)]
pub struct Postcard<T>(pub T);

/// Encode and decode data as bincode using `bincode`.
#[cfg(any(doc, feature = "bincode"))]
#[derive(Clone, Debug)]
pub struct Bincode<T>(pub T);
