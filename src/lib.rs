extern crate clap;
extern crate hyper;
extern crate hyper_tls;

use clap::{App, Arg, ArgMatches};
use hyper::client::HttpConnector;
use hyper::rt::{self, Future, Stream};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use std::str::FromStr;

pub fn run_config<'a>() -> RunConfig<'a> {
    let matches = cli_app().get_matches();

    RunConfig::from(matches)
}

pub fn cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("rust-client")
        .version("0.1.0")
        .author("Kellen Frodelius-Fujimoto <kellen@kellenfujimoto.com>")
        .about("A command line http client")
        .arg(
            Arg::with_name("METHOD")
                .help("The method used in the request")
                .possible_values(&["get", "post", "head"])
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("URI")
                .help("The URI to send the request to")
                .required(true)
                .index(2),
        )
}

pub fn request_uri(addr: Uri, method: Method, body: Option<impl Into<Body>>) -> RequestResult<()> {
    let client = build_client();

    let request = build_request(addr, method, body.map(|b| b.into()));

    let sent_request = client.request(request);

    let response = sent_request.and_then(|res| {
        println!("{:?} {}", res.version(), res.status());
        for (key, value) in res.headers() {
            println!("{}: {}", key.as_str(), value.to_str().unwrap());
        }
        res.into_body().concat2()
    });

    let print_body = response
        .and_then(|body| {
            let s = ::std::str::from_utf8(&body)
                .expect("rust-client only supports UTF-8 response bodies");
            println!("---");
            println!("{}", s);
            Ok(())
        })
        .map_err(|err| {
            println!("error: {}", err);
        });

    rt::run(print_body);
    Ok(())
}

fn build_request(addr: Uri, method: Method, body: Option<Body>) -> Request<Body> {
    let mut request = Request::builder();
    request
        .uri(addr)
        .method(method)
        .header("User-Agent", "rust-client/0.1.0");

    let body = body.unwrap_or("".into());
    request.body(body).unwrap()
}

fn build_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    let https_connector = HttpsConnector::new(4).unwrap();
    Client::builder()
        .keep_alive(false)
        .build::<_, hyper::Body>(https_connector)
}

pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug)]
pub enum RequestError {
    Hyper(hyper::Error),
}

#[derive(Debug)]
pub struct RunConfig<'a> {
    raw_matches: ArgMatches<'a>,
    uri: Uri,
    method: Method,
}

// TODO: Figure out lifetime issues around returning reference to properties; I'd rather not clone all over the place
impl<'a> RunConfig<'a> {
    pub fn uri(&self) -> Uri {
        self.uri.clone()
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }
}

impl<'a> From<ArgMatches<'a>> for RunConfig<'a> {
    fn from(matches: ArgMatches<'a>) -> RunConfig<'a> {
        let method = matches
            .value_of("METHOD")
            .map(|method| {
                Method::from_str(&method.to_ascii_uppercase()).expect("Incompatible HTTP method")
            })
            .expect("METHOD is a required argument");

        let uri = matches
            .value_of("URI")
            .map(|addr| uri_with_added_missing_scheme(addr))
            .expect("URI is a required argument");

        RunConfig {
            raw_matches: matches,
            uri,
            method,
        }
    }
}

fn uri_with_added_missing_scheme(addr: &str) -> Uri {
    if addr.starts_with("http") {
        addr.parse::<Uri>().expect("Invalid URI")
    } else {
        let addr = "http://".to_owned() + addr;
        addr.parse::<Uri>().expect("Invalid URI")
    }
}

#[cfg(test)]
mod tests {
    use super::{build_request, cli_app, RunConfig};
    use hyper::{Method, Uri};

    #[test]
    fn run_config_explicit_method() {
        let app = cli_app();
        let get_args = vec!["rc", "get", "http://localhost:8000"];
        let post_args = vec!["rc", "post", "http://localhost:8000"];
        let head_args = vec!["rc", "head", "http://localhost:8000"];

        let get_matches = app.clone().get_matches_from(get_args);
        let get_config = RunConfig::from(get_matches);
        assert_eq!(get_config.method(), Method::GET);

        let post_matches = app.clone().get_matches_from(post_args);
        let post_config = RunConfig::from(post_matches);
        assert_eq!(post_config.method(), Method::POST);

        let head_matches = app.clone().get_matches_from(head_args);
        let head_config = RunConfig::from(head_matches);
        assert_eq!(head_config.method(), Method::HEAD);
    }

    #[test]
    fn build_request_respects_method() {
        let addr = "https://localhost:8000".parse::<Uri>().unwrap();
        let get = Method::GET;
        let post = Method::POST;
        let head = Method::HEAD;

        let get_req = build_request(addr.clone(), get, None);
        assert_eq!(get_req.method(), &Method::GET);

        let post_req = build_request(addr.clone(), post, None);
        assert_eq!(post_req.method(), &Method::POST);

        let head_req = build_request(addr.clone(), head, None);
        assert_eq!(head_req.method(), &Method::HEAD);
    }

    #[test]
    fn config_appends_missing_http() {
        let addr = "localhost:8000";
        let args = vec!["rc", "get", addr];
        let matches = cli_app().get_matches_from(args);
        let config = RunConfig::from(matches);

        assert_eq!(
            config.uri().to_string(),
            "http://localhost:8000/".to_owned()
        );
    }
}
