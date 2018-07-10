extern crate hyper;
extern crate rust_client;

use hyper::Method;
use rust_client::request_uri;
use rust_client::run_config;

fn main() {
    let config = run_config();

    let target = config.uri();

    println!("GET {}", target);
    let body: Option<&str> = None;
    request_uri(target, Method::GET, body).expect("Unable to request uri");
}
