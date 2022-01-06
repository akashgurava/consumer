use std::sync::Arc;

use http::{Request, Response};
use hyper::client::{Client, HttpConnector};
use hyper::Body;
use hyper_tls::HttpsConnector;

use crate::error::ConsumerError;

use super::consumer::{Consumer, ConsumerTrait};

pub type HyperClient = Client<HttpsConnector<HttpConnector>>;

#[async_trait]
impl ConsumerTrait for HyperClient {
    type Body = Body;

    fn empty_body(&self) -> Self::Body {
        Body::empty()
    }

    async fn fetch(&self, request: Request<Body>) -> Result<Response<Body>, ConsumerError> {
        self.request(request).await.map_err(ConsumerError::from)
    }
}

impl Default for Consumer<HyperClient> {
    fn default() -> Self {
        let client = Client::builder().build(HttpsConnector::new());
        Consumer::new_hyper(client)
    }
}

impl Consumer<HyperClient> {
    pub fn new_hyper(client: HyperClient) -> Self {
        Consumer {
            inner: Arc::new(client),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_new_hyper() {
        let client = Client::builder().build(HttpsConnector::new());

        Consumer::new_hyper(client);
        Consumer::<HyperClient>::default();
    }
}
