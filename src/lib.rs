extern crate hyper;
extern crate futures;
extern crate hyper_tls;

use hyper::Response;
use hyper::Method;
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper::HeaderMap;
use hyper::Uri;
use std::io::{self, Write};
use hyper_tls::HttpsConnector;

pub fn request_uri(addr: Uri, method: Method) -> RequestResult<()> {
    let https_connector = HttpsConnector::new(4).unwrap();
    let client = Client::builder()
        .keep_alive(false)
        .build::<_, hyper::Body>(https_connector);

    let fut = client
        .get(addr)
        .and_then(|res| {
            for (key, value) in res.headers() {
                println!("{}: {}", key.as_str(), value.to_str().unwrap());
            }
            res.into_body().concat2()
        })
        .and_then(|body| {
            let s = ::std::str::from_utf8(&body)
                .expect("Reponse must by valid utf-8");
            println!("---");
            println!("{}", s);
            Ok(())
        })
        .map_err(|err| {
            println!("error: {}", err);
        });

    rt::run(fut);
    Ok(())
}

pub struct ResponseComponents {
    headers: HeaderMap,
}

pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug)]
pub enum RequestError {
    Hyper(hyper::Error)
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert!(false);
    }
}