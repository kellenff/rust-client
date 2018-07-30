extern crate clap;
extern crate futures;
extern crate hyper;
extern crate reqwest;
extern crate rust_client;

use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use reqwest::Method;
use rust_client::app::{cli_app, RunConfig};
use std::thread;
use std::thread::JoinHandle;

const TEST_SERVER_ADDR: &'static str = "127.0.0.1:7890";

const GET_RESP_BODY: &'static str = "Hello, world!";

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn dummy_service(req: Request<Body>) -> BoxFut {
    use hyper::Method;
    let mut response = Response::new(Body::empty());

    match req.method() {
        &Method::GET => {
            *response.body_mut() = Body::from(GET_RESP_BODY);
        }
        _ => {}
    };

    Box::new(future::ok(response))
}

fn run_test_server() -> JoinHandle<()> {
    thread::spawn(|| {
        let addr = TEST_SERVER_ADDR.parse().unwrap();
        let server = Server::bind(&addr)
            .serve(|| service_fn(dummy_service))
            .map_err(|e| eprintln!("server error: {}", e));
        hyper::rt::run(server);
    })
}

#[test]
fn get_simple() {
    run_test_server();

    let matches = cli_app().get_matches_from(&["rc", "get", TEST_SERVER_ADDR]);
    let config = RunConfig::from(matches);

    assert_eq!(config.method(), Method::Get);

    config.execute().unwrap();
}

#[test]
fn head_simple() {
    run_test_server();

    let matches = cli_app().get_matches_from(&["rc", "head", TEST_SERVER_ADDR]);
    let config = RunConfig::from(matches);

    assert_eq!(config.method(), Method::Head);

    config.execute().unwrap();
}
