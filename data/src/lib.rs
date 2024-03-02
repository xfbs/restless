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
