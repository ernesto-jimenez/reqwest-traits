# reqwest-traits

## reqwest-traits

This crate provides traits for the [reqwest](https://crates.io/crates/reqwest) crate. It is
intended to be used by libraries that need to make HTTP requests, want to allow users to inject
their own reqwest client, but don't want to force users to use reqwest::Client.

### Example use cases

- Use reqwest::Client in your program, but inject client with
  [rvcr](https://crates.io/crates/rvcr) for testing to avoid making real HTTP requests.
- Let users of your library inject their own reqwest::Client, if they want a caching or tracing
  middleware, a reqwest_middleware::ClientWithMiddleware.

### Example

```rust
use reqwest_traits::Client;

struct MyClient<C: Client> {
    http: C,
}

async fn plain_reqwest() {
    let http = reqwest::Client::new();
    let client = MyClient { http };
    let req = client.http.get("https://example.com");
    let response = req.send().await.unwrap();
    assert_eq!(response.status(), 200);
}

async fn reqwest_middleware() {
    let http = reqwest_middleware::ClientBuilder::new(
        reqwest::Client::new(),
    ).build();
    let client = MyClient { http };
    let req = client.http.get("https://example.com");
    let response = req.send().await.unwrap();
    assert_eq!(response.status(), 200);
}
```

License: MIT
