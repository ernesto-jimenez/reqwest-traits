//! # reqwest-traits
//!
//! This crate provides traits for the [reqwest](https://crates.io/crates/reqwest) crate. It is
//! intended to be used by libraries that need to make HTTP requests, want to allow users to inject
//! their own reqwest client, but don't want to force users to use reqwest::Client.
//!
//! ## Example use cases
//!
//! - Use reqwest::Client in your program, but inject client with
//!   [rvcr](https://crates.io/crates/rvcr) for testing to avoid making real HTTP requests.
//! - Let users of your library inject their own reqwest::Client, if they want a caching or tracing
//!   middleware, a reqwest_middleware::ClientWithMiddleware.
//!
//! ## Example
//!
//! ```rust
//! use reqwest_traits::Client;
//!
//! struct MyClient<C: Client> {
//!     http: C,
//! }
//!
//! async fn plain_reqwest() {
//!     let http = reqwest::Client::new();
//!     let client = MyClient { http };
//!     let req = client.http.get("https://example.com");
//!     let response = req.send().await.unwrap();
//!     assert_eq!(response.status(), 200);
//! }
//!
//! async fn reqwest_middleware() {
//!     let http = reqwest_middleware::ClientBuilder::new(
//!         reqwest::Client::new(),
//!     ).build();
//!     let client = MyClient { http };
//!     let req = client.http.get("https://example.com");
//!     let response = req.send().await.unwrap();
//!     assert_eq!(response.status(), 200);
//! }
//! ```

use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{IntoUrl, Method, Request, Response};

/// The `RequestBuilder` trait represents `reqwest::RequestBuilder`.
#[async_trait]
pub trait RequestBuilder {
    type Error: std::error::Error;

    fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>;

    async fn send(self) -> Result<reqwest::Response, Self::Error>;
}

/// The `Client` trait represents `reqwest::Client`.
#[async_trait]
pub trait Client {
    type RequestBuilder: RequestBuilder;
    type Error: std::error::Error;

    /// Makes a `GET` request to a URL.
    fn get<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder;

    /// Makes a `POST` request to a URL.
    fn post<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder;

    /// Makes a `PUT` request to a URL.
    fn put<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder;

    /// Makes a `PATCH` request to a URL.
    fn patch<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder;

    /// Makes a `DELETE` request to a URL.
    fn delete<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder;

    /// Makes a `HEAD` request to a URL.
    fn head<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder;

    /// Builds a `Request` with the `Method` and `Url`.
    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::RequestBuilder;

    /// Executes a `Request`.
    async fn execute(&self, request: Request) -> Result<Response, Self::Error>;
}

#[cfg(feature = "reqwest")]
mod impl_reqwest;
pub use impl_reqwest::*;

#[cfg(feature = "reqwest-middleware")]
mod impl_reqwest_middleware;
pub use impl_reqwest_middleware::*;
