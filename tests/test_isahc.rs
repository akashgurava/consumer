mod common;

use common::*;

// #[cfg_attr(not(feature = "isahc-client"), ignore)]
#[cfg(feature = "isahc-client")]
mod isahc_tests {
    use isahc::AsyncReadResponseExt;

    use consumer::{Consumer, IsahcClient};

    use super::*;

    #[tokio::test]
    pub async fn test_isahc_single_request() {
        let server = start_server().await;
        let url = format!("{}/hello", server.uri());

        let consumer = Consumer::<IsahcClient>::default();
        let response = consumer.get(&url).await;
        assert!(response.is_ok());

        let mut response = response.unwrap();
        assert_eq!(response.status(), 200);

        let data = response.text().await;
        assert!(data.is_ok());
        assert_eq!(data.unwrap(), r#"{"msg": "world"}"#);
    }
}
