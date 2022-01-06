use std::sync::Arc;

use http::{Request, Response, Uri};

use crate::error::ConsumerError;

#[async_trait]
pub trait ConsumerTrait {
    type Body;

    fn empty_body(&self) -> Self::Body;

    async fn fetch(
        &self,
        request: Request<Self::Body>,
    ) -> Result<Response<Self::Body>, ConsumerError>;
}

#[derive(Clone)]
pub struct Consumer<T>
where
    T: ConsumerTrait,
{
    pub(super) inner: Arc<T>,
}

impl<T> Consumer<T>
where
    T: ConsumerTrait,
{
    pub async fn get(&self, url: &str) -> Result<Response<T::Body>, ConsumerError> {
        let client = self.inner.clone();
        let request = Request::builder()
            .uri(url.parse::<Uri>().unwrap())
            .body(client.empty_body())?;
        client.fetch(request).await
    }
}
