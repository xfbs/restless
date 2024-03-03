//! # restless query
//!
//! This crate implements several strategies to encode query parameters mechanically from Rust
//! structs. As a consumer of `restless`, you should not be using this crate directly, but rather
//! the `restless` crate and activate the features there.

#![warn(missing_docs)]

pub use restless_core::query::ToQuery;

macro_rules! module {
    ($feature:expr, $name:ident) => {
        #[cfg(feature = $feature)]
        mod $name;
        #[cfg(feature = $feature)]
        pub use $name::*;
    };
}

module!("qs", qs);
module!("urlencoded", urlencoded);
