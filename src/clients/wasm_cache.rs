use crate::clients::gloo::GlooRequest;
use async_trait::async_trait;
use std::{borrow::Cow, fmt::Debug, hash::Hash};
use wasm_cache::CacheItem;

/// Cached request.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub struct CachedRequest<R: GlooRequest + Clone + Ord + Eq + Hash + Debug> {
    /// Prefix of request URI.
    pub prefix: Cow<'static, str>,
    /// Request to send.
    pub request: R,
}

#[async_trait(?Send)]
impl<R: GlooRequest + PartialEq + Eq + PartialOrd + Ord + Hash + Clone + Debug + 'static>
    CacheItem<()> for CachedRequest<R>
where
    R::Response: Clone + Debug + PartialEq,
{
    type Value = R::Response;
    type Error = R::Error;

    async fn send(&self) -> Result<Self::Value, Self::Error> {
        self.request.send_prefix(&self.prefix).await
    }
}
