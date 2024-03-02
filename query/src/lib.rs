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
