#[cfg(feature = "bytes")]
use bytes::Bytes;
#[cfg(feature = "serde")]
use serde::Serialize;

pub trait RequestEncoding {
    fn encode(&self) -> Vec<u8>;

    #[cfg(feature = "bytes")]
    fn encode_bytes(&self) -> Bytes {
        self.encode().into()
    }
}

impl RequestEncoding for () {
    fn encode(&self) -> Vec<u8> {
        Vec::new()
    }
}

#[cfg(feature = "bytes")]
impl RequestEncoding for Bytes {
    fn encode(&self) -> Vec<u8> {
        self.to_vec()
    }

    fn encode_bytes(&self) -> Bytes {
        self.clone()
    }
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::Json;

    impl<T: Serialize> RequestEncoding for Json<T> {
        fn encode(&self) -> Vec<u8> {
            serde_json::to_vec(&self.0).unwrap()
        }
    }

    impl RequestEncoding for serde_json::Value {
        fn encode(&self) -> Vec<u8> {
            serde_json::to_vec(&self).unwrap()
        }
    }
}

#[cfg(feature = "bincode")]
impl<T: Serialize> RequestEncoding for super::Bincode<T> {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self.0).unwrap()
    }
}

#[cfg(feature = "yaml")]
mod yaml {
    use super::*;
    use crate::Yaml;

    impl<T: Serialize> RequestEncoding for Yaml<T> {
        fn encode(&self) -> Vec<u8> {
            serde_yaml::to_string(&self.0).unwrap().into_bytes()
        }
    }

    impl RequestEncoding for serde_yaml::Value {
        fn encode(&self) -> Vec<u8> {
            serde_yaml::to_string(&self).unwrap().into_bytes()
        }
    }
}

#[cfg(feature = "postcard")]
impl<T: Serialize> RequestEncoding for super::Postcard<T> {
    fn encode(&self) -> Vec<u8> {
        postcard::to_allocvec(&self.0).unwrap()
    }
}
