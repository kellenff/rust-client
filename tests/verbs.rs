extern crate docopt;
extern crate mockito;
extern crate reqwest;
extern crate rust_client;

use docopt::Docopt;
use mockito::mock;
use reqwest::Method;
use reqwest::StatusCode;
use rust_client::app::{Args, RunConfig, USAGE};
use rust_client::command::Command;

const TARGET_URL: &'static str = mockito::SERVER_URL;

const GET_RESP_BODY: &'static str = "Hello, world!";

#[test]
fn get_simple() {
    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(GET_RESP_BODY)
        .create();

    let args = || vec!["rc", "get", TARGET_URL].into_iter();

    let matches: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(args()).deserialize())
        .unwrap();
    let config = RunConfig::from(matches);

    assert_eq!(config.method(), Method::GET);

    let command = Command::from(&config);

    let response = command.send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.text(), GET_RESP_BODY);
}
