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
        reqwest_middleware::RequestBuilder::header(self, key, value)
    }

    async fn send(self) -> Result<reqwest::Response, reqwest_middleware::Error> {
        reqwest_middleware::RequestBuilder::send(self).await
    }
}

#[async_trait]
impl Client for reqwest_middleware::ClientWithMiddleware {
    type RequestBuilder = reqwest_middleware::RequestBuilder;
    type Error = reqwest_middleware::Error;

    fn get<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::get(self, url)
    }

    fn post<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::post(self, url)
    }

    fn put<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::put(self, url)
    }

    fn patch<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::patch(self, url)
    }

    fn delete<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::delete(self, url)
    }

    fn head<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::head(self, url)
    }

    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::RequestBuilder {
        reqwest_middleware::ClientWithMiddleware::request(self, method, url)
    }

    async fn execute(&self, request: Request) -> Result<Response, Self::Error> {
        reqwest_middleware::ClientWithMiddleware::execute(self, request).await
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
