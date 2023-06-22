use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{IntoUrl, Method, Request, Response};

use crate::{Client, RequestBuilder};

#[async_trait]
impl RequestBuilder for reqwest_middleware::RequestBuilder {
    type Error = reqwest_middleware::Error;

    fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        Self::header(self, key, value)
    }

    fn headers(self, headers: reqwest::header::HeaderMap) -> Self {
        Self::headers(self, headers)
    }

    fn basic_auth<U, P>(self, username: U, password: Option<P>) -> Self
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        Self::basic_auth(self, username, password)
    }

    fn bearer_auth<T>(self, token: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::bearer_auth(self, token)
    }

    fn body<T: Into<reqwest::Body>>(self, body: T) -> Self {
        Self::body(self, body)
    }

    fn json<T: serde::Serialize + ?Sized>(self, json: &T) -> Self {
        Self::json(self, json)
    }

    fn timeout(self, timeout: std::time::Duration) -> Self {
        Self::timeout(self, timeout)
    }

    fn query<T: serde::Serialize + ?Sized>(self, query: &T) -> Self {
        Self::query(self, query)
    }

    fn version(self, version: http::Version) -> Self {
        Self::version(self, version)
    }

    fn build(self) -> Result<Request, reqwest::Error> {
        Self::build(self)
    }

    async fn send(self) -> Result<reqwest::Response, Self::Error> {
        Self::send(self).await
    }
}

#[async_trait]
impl Client for reqwest_middleware::ClientWithMiddleware {
    type RequestBuilder = reqwest_middleware::RequestBuilder;
    type Error = reqwest_middleware::Error;

    fn get<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        Self::get(self, url)
    }

    fn post<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        Self::post(self, url)
    }

    fn put<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        Self::put(self, url)
    }

    fn patch<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        Self::patch(self, url)
    }

    fn delete<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        Self::delete(self, url)
    }

    fn head<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        Self::head(self, url)
    }

    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::RequestBuilder {
        Self::request(self, method, url)
    }

    async fn execute(&self, request: Request) -> Result<Response, Self::Error> {
        Self::execute(self, request).await
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use tokio;

    use super::*;

    struct MyClient<C: Client> {
        http: C,
    }

    #[tokio::test]
    async fn get_with_reqwest() {
        let client = MyClient {
            http: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
        };
        let request = client.http.get("https://www.rust-lang.org");
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
