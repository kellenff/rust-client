extern crate rust_client;
extern crate clap;
extern crate hyper;

use clap::{Arg, App};

use rust_client::request_uri;
use hyper::Uri;
use hyper::Method;

fn main() {
    let matches = App::new("rust-client")
        .version("0.1.0")
        .author("Kellen Frodelius-Fujimoto <kellen@kellenfujimoto.com>")
        .about("A command line http client")
        .arg(Arg::with_name("URI")
            .help("The URI to send the request to")
            .required(true)
            .index(1))
        .get_matches();

    let target = matches.value_of("URI").expect("URI is required!");

    println!("GET {}", target);
    request_uri(target.parse::<Uri>().unwrap(), Method::HEAD);
}
