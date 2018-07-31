extern crate clap;
extern crate mockito;
extern crate reqwest;
extern crate rust_client;

use mockito::mock;
use reqwest::Method;
use reqwest::StatusCode;
use rust_client::app::{cli_app, RunConfig};

const TARGET_URL: &'static str = mockito::SERVER_URL;

const GET_RESP_BODY: &'static str = "Hello, world!";

#[test]
fn get_simple() {
    let _m = mock("GET", "/")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .with_body(GET_RESP_BODY)
        .create();

    let matches = cli_app().get_matches_from(&["rc", "get", TARGET_URL]);
    let config = RunConfig::from(matches);

    assert_eq!(config.method(), Method::Get);

    let mut response = config.execute().unwrap().response;

    assert_eq!(response.status(), StatusCode::Ok);
    assert_eq!(response.text().unwrap(), GET_RESP_BODY);
}

#[test]
fn head_simple() {
    let _m = mock("HEAD", "/")
        .with_status(200)
        .with_header("content-type", "text/plain")
        .create();

    let matches = cli_app().get_matches_from(&["rc", "head", TARGET_URL]);
    let config = RunConfig::from(matches);

    assert_eq!(config.method(), Method::Head);

    let response = config.execute().unwrap().response;

    assert_eq!(response.status(), StatusCode::Ok);
}
