mod request;
pub use request::*;

mod response;
pub use response::*;

mod query;
pub use query::*;

#[cfg(feature = "json")]
#[derive(Clone, Debug)]
pub struct Json<T>(pub T);

#[cfg(feature = "yaml")]
#[derive(Clone, Debug)]
pub struct Yaml<T>(pub T);

#[cfg(feature = "postcard")]
#[derive(Clone, Debug)]
pub struct Postcard<T>(pub T);

#[cfg(feature = "bincode")]
#[derive(Clone, Debug)]
pub struct Bincode<T>(pub T);
