//! # restless axum
//!
//! Integration of restless with axum.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

use async_trait::async_trait;
use axum::Router;
use restless_core::{Decodable, Request};
use restless_hyper::{hyper, HyperRequest};
use tower::ServiceExt;

/// Error sending request.
#[derive(thiserror::Error, Debug)]
pub enum Error {}

/// Trait to extend axum's [`Router`] to be able to send `restless` requests.
#[async_trait(?Send)]
pub trait RouterExt {
    /// Send request to router.
    async fn send<R: Request>(
        &self,
        request: R,
    ) -> Result<<R::Response as Decodable>::Target, Error>;
}

#[async_trait(?Send)]
impl RouterExt for Router {
    async fn send<R: Request>(
        &self,
        request: R,
    ) -> Result<<R::Response as Decodable>::Target, Error> {
        println!("{:?} {}", request.method(), request.uri());
        let response = self
            .clone()
            .oneshot(request.to_hyper_request().unwrap())
            .await
            .unwrap();
        let _status = response.status();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        Ok(R::Response::decode(&body[..]).unwrap())
    }
}
