//! Request method wrappers.
//!
//! For a given type T that implements [`GetRequest`], [`Get<T>`] implements [`Request`]. These are
//! types that wrap request method traits and turn them into something that implements [`Request`].
//!
//! You do not need to use these directly, rather you can use the [`RequestMethod`] trait to make
//! your type implement [`Request`] directing, using one of these wrapper types.
use crate::request::*;

macro_rules! wrapper {
    ($wrapper:ident, $trait:ident) => {
        wrapper!(@struct, $wrapper, $trait);
        wrapper!(@from, $wrapper, $trait);
    };

    (@struct, $wrapper:ident, $trait:ident) => {
        #[doc = "Turn a [`"]
        #[doc = stringify!($trait)]
        #[doc = "`] into a [`Request`]."]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $wrapper<T: $trait> {
            /// Inner request
            pub inner: T
        }
    };

    (@from, $wrapper:ident, $trait:ident) => {
        impl<'a, T: $trait> From<&'a T> for &'a $wrapper<T> {
            fn from(request: &T) -> &$wrapper<T> {
                unsafe {
                    &*(request as *const T as *const $wrapper<T>)
                }
            }
        }

        impl<T: $trait> From<T> for $wrapper<T> {
            fn from(request: T) -> $wrapper<T> {
                $wrapper {
                    inner: request
                }
            }
        }
    };
}

wrapper!(Get, GetRequest);
wrapper!(Head, HeadRequest);
wrapper!(Post, PostRequest);
wrapper!(Delete, DeleteRequest);
wrapper!(Patch, PatchRequest);
