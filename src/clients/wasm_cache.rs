use crate::clients::gloo::GlooRequest;
use async_trait::async_trait;
use std::{borrow::Cow, fmt::Debug, hash::Hash};
use wasm_cache::{Invalidatable, CacheItem};

/// Cached request.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct CachedRequest<R: GlooRequest + Clone + Ord + Eq + Hash + Debug> {
    /// Prefix of request URI.
    pub prefix: Cow<'static, str>,
    /// Request to send.
    pub request: R,
}

impl<R: GlooRequest + Clone + Ord + Eq + Hash + Debug, M: 'static> Invalidatable<M> for CachedRequest<R> where R: Invalidatable<M> {
    fn invalidated_by(&self, value: &M) -> bool {
        self.request.invalidated_by(value)
    }
}

#[async_trait(?Send)]
impl<R: GlooRequest + PartialEq + Eq + PartialOrd + Ord + Hash + Clone + Debug + 'static, M: 'static>
    CacheItem<M> for CachedRequest<R>
where
    R::Response: Clone + Debug + PartialEq,
    R: Invalidatable<M>,
{
    type Value = R::Response;
    type Error = R::Error;

    async fn send(&self) -> Result<Self::Value, Self::Error> {
        self.request.send_prefix(&self.prefix).await
    }
}
