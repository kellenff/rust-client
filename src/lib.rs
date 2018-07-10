extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;

use clap::{App, Arg, ArgMatches};
use hyper::client::HttpConnector;
use hyper::rt::{self, Future, Stream};
use hyper::{Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

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
            Arg::with_name("URI")
                .help("The URI to send the request to")
                .required(true)
                .index(1),
        )
}

pub fn request_uri(addr: Uri, method: Method, body: Option<impl Into<Body>>) -> RequestResult<()> {
    let client = build_client();

    let request = build_request(addr, method, body.map(|b| b.into()));

    let response = client.request(request);

    let work = response
        .and_then(|res| {
            for (key, value) in res.headers() {
                println!("{}: {}", key.as_str(), value.to_str().unwrap());
            }
            res.into_body().concat2()
        })
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

    rt::run(work);
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
}

impl<'a> RunConfig<'a> {
    pub fn uri(&self) -> Uri {
        // TODO: Figure out lifetime issues around returning reference to self.uri; I'd rather not clone all over the place
        self.uri.clone()
    }
}

impl<'a> From<ArgMatches<'a>> for RunConfig<'a> {
    fn from(matches: ArgMatches<'a>) -> RunConfig<'a> {
        let uri = match matches.value_of("URI") {
            Some(s) => s
                .parse::<Uri>()
                .expect("URI argument must conform to the `hyper` URI specification"),
            None => {
                // URI is a required argument; `clap` should catch this case for us
                unreachable!();
            }
        };
        RunConfig {
            raw_matches: matches,
            uri,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{build_request, cli_app, RunConfig};
    use hyper::{Method, Uri};

    #[test]
    fn run_config_uri_method() {
        let mut app = cli_app();
        let cli_args = vec!["rc", "http://localhost:8000"];
        let matches = app.get_matches_from(cli_args);
        let uri = matches
            .value_of("URI")
            .unwrap()
            .clone()
            .parse::<Uri>()
            .unwrap();
        let config = RunConfig::from(matches);

        assert_eq!(uri, config.uri());
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
}
