pub use restless_core::{Decodable, Encodable};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::Json;

#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "yaml")]
pub use yaml::Yaml;

#[cfg(feature = "postcard")]
mod postcard;
#[cfg(feature = "postcard")]
pub use postcard::Postcard;

#[cfg(feature = "bincode")]
mod bincode;
#[cfg(feature = "bincode")]
pub use bincode::Bincode;
