use http_test_server::http::{Method, Status};
use http_test_server::TestServer;

use reqwest_traits::{Client, RequestBuilder};

async fn test<C: Client>(client: C) {
    let server = TestServer::new().unwrap();
    let resource = server.create_resource("/example");

    resource
        .status(Status::OK)
        .method(Method::GET)
        .header("Content-Type", "application/text")
        .header("Cache-Control", "no-cache")
        .body("Hello");

    let res = client
        .get(&format!("http://localhost:{}/example", server.port()))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(res.status(), 200);
    let bytes = res.bytes().await.expect("res.bytes()");
    assert_eq!("Hello", bytes);
}

#[tokio::test]
async fn inject_reqwest_client() {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    test(client).await;
}

#[tokio::test]
async fn inject_reqwest_middleware_client() {
    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
    test(client).await;
}
