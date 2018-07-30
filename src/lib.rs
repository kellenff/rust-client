extern crate clap;
extern crate reqwest;

use reqwest::Client;
use reqwest::{Body, Method, Response, Url};
use std::time::Duration;

use error::RequestError;
use error::RequestResult;

pub mod app;
pub mod error;

pub fn request_url(
    addr: Url,
    method: Method,
    body: Option<impl Into<Body>>,
) -> RequestResult<Response> {
    let client = build_client();

    Ok(match method {
        Method::Get => client.get(addr).send()?,
        Method::Post => {
            if let Some(body) = body {
                client.post(addr).body(body.into()).send()?
            } else {
                client.post(addr).send()?
            }
        }
        Method::Head => client.head(addr).send()?,
        _ => {
            return Err(RequestError::UnsupportedMethod(method));
        }
    })
}

fn build_client() -> Client {
    Client::builder()
        .gzip(true)
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

fn uri_with_added_missing_scheme(addr: &str) -> Url {
    if addr.starts_with("http") {
        addr.parse::<Url>().expect("Invalid URL")
    } else {
        let addr = "http://".to_owned() + addr;
        addr.parse::<Url>().expect("Invalid URL")
    }
}
