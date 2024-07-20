use reqwest::blocking::Client;

use crate::prelude::Io;

pub fn new_client() -> Client {
    Client::builder()
        .cookie_provider(std::sync::Arc::new(reqwest::cookie::Jar::default()))
        .build()
        .unwrap()
}

pub struct HttpRequestIo {
    request: reqwest::blocking::RequestBuilder,
}

impl Io for HttpRequestIo {
    type Output = reqwest::blocking::Response;

    fn run(self) -> Self::Output {
        self.request.send().unwrap()
    }
}

pub fn http_request(request: reqwest::blocking::RequestBuilder) -> HttpRequestIo {
    HttpRequestIo { request }
}
