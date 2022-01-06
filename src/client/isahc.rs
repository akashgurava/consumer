use std::sync::Arc;

use http::{Request, Response};
use isahc::{AsyncBody as Body, HttpClient as IsahcClient};

use crate::error::ConsumerError;

use super::consumer::{Consumer, ConsumerTrait};

#[async_trait]
impl ConsumerTrait for IsahcClient {
    type Body = Body;

    fn empty_body(&self) -> Self::Body {
        Body::empty()
    }

    #[inline]
    async fn fetch(&self, request: Request<Body>) -> Result<Response<Body>, ConsumerError> {
        self.send_async(request).await.map_err(ConsumerError::from)
    }
}

impl Default for Consumer<IsahcClient> {
    fn default() -> Self {
        let client = IsahcClient::new().unwrap();
        Consumer::new_isahc(client)
    }
}

impl Consumer<IsahcClient> {
    pub fn new_isahc(client: IsahcClient) -> Self {
        Consumer {
            inner: Arc::new(client),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_new_isahc() {
        let client = IsahcClient::new().unwrap();

        Consumer::new_isahc(client);
        Consumer::<IsahcClient>::default();
    }
}
