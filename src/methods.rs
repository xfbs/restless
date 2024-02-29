//! Request method wrappers.
//!
//! For a given type T that implements [`GetRequest`], [`Get<T>`] implements [`Request`]. These are
//! types that wrap request method traits and turn them into something that implements [`Request`].
//!
//! You do not need to use these directly, rather you can use the [`RequestMethod`] trait to make
//! your type implement [`Request`] directing, using one of these wrapper types.

pub use restless_core::methods::*;
