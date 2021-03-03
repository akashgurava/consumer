use std::net::TcpListener;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

pub async fn start_server() -> MockServer {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener.local_addr().expect("unable to bind on port");

    // Start a background HTTP server on a random local port
    let mock_server = MockServer::builder().listener(listener).start().await;

    Mock::given(method("GET"))
        .and(path("/hello"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(r#"{"msg": "world"}"#, "application/json"),
        )
        // Mounting the mock on the mock server - it's now effective!
        .mount(&mock_server)
        .await;
    mock_server
}
