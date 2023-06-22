use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{IntoUrl, Method, Response, Request};

use super::{Client, RequestBuilder};

#[async_trait]
impl RequestBuilder for reqwest::RequestBuilder {
    type Error = reqwest::Error;

    fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        reqwest::RequestBuilder::header(self, key, value)
    }

    async fn send(self) -> Result<reqwest::Response, reqwest::Error> {
        reqwest::RequestBuilder::send(self).await
    }
}

#[async_trait]
impl Client for reqwest::Client {
    type RequestBuilder = reqwest::RequestBuilder;
    type Error = reqwest::Error;

    fn get<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest::Client::get(self, url)
    }

    fn post<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest::Client::post(self, url)
    }

    fn put<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest::Client::put(self, url)
    }

    fn patch<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest::Client::patch(self, url)
    }

    fn delete<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest::Client::delete(self, url)
    }

    fn head<U: IntoUrl>(&self, url: U) -> Self::RequestBuilder {
        reqwest::Client::head(self, url)
    }

    fn request<U: IntoUrl>(&self, method: Method, url: U) -> Self::RequestBuilder {
        reqwest::Client::request(self, method, url)
    }

    async fn execute(&self, request: Request) -> Result<Response, Self::Error> {
        reqwest::Client::execute(self, request).await
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
            http: reqwest::ClientBuilder::new().build().unwrap(),
        };
        let request = client.http.get("https://www.rust-lang.org");
        let response = request.send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
