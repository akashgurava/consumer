mod common;

use common::*;

// #[cfg_attr(not(feature = "hyper-client"), ignore)]
#[cfg(feature = "hyper-client")]
mod hyper_tests {
    use hyper::body::to_bytes;

    use consumer::{Consumer, HyperClient};

    use super::*;

    #[tokio::test]
    async fn test_hyper_single_request() {
        let server = start_server().await;
        let url = format!("{}/hello", server.uri());

        let consumer = Consumer::<HyperClient>::default();
        let response = consumer.get(&url).await;
        assert!(response.is_ok());

        let mut response = response.unwrap();
        assert_eq!(response.status(), 200);

        let data = to_bytes(response.body_mut()).await;
        assert!(data.is_ok());

        let data = String::from_utf8(data.unwrap().to_vec()).unwrap();
        assert_eq!(data, r#"{"msg": "world"}"#);
    }
}
