use async_trait::async_trait;
use axum::Router;
use restless_core::{Decodable, Request};
use restless_hyper::{hyper, HyperRequest};
use tower::ServiceExt;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[async_trait(?Send)]
trait RouterExt {
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
        let status = response.status();
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        Ok(R::Response::decode(&body[..]).unwrap())
    }
}
