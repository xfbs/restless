//! # restless data
//!
//! This crate encodes several strategies for encoding and decoding data from requests and response
//! bodies.
//!
//! As a consumer of `restless`, you should not depend on this crate directly, but rather on the
//! main `restless` crate and turn on the prerequisite features.

#![warn(missing_docs)]
pub use restless_core::{Decodable, Encodable};

macro_rules! module {
    ($feature:expr, $name:ident) => {
        #[cfg(feature = $feature)]
        mod $name;
        #[cfg(feature = $feature)]
        pub use $name::*;
    };
}

module!("json", json);
module!("yaml", yaml);
module!("postcard", postcard);
module!("bincode", bincode);
module!("miniserde", miniserde);
