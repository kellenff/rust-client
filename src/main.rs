extern crate reqwest;
extern crate rust_client;

use reqwest::StatusCode;
use rust_client::app::run_config;
use std::io::{self, Write};
use std::time;

fn main() {
    let config = run_config();

    println!("{} {}", config.method(), config.url());

    let now = time::Instant::now();
    let mut response = config.execute().unwrap().response;
    let elapsed = now.elapsed();
    println!(
        "{:.4}s: {}",
        (elapsed.as_secs() as f64 + f64::from(elapsed.subsec_nanos()) * 1e-9),
        response.status()
    );

    match response.status() {
        StatusCode::Ok => {
            for header in response.headers().iter() {
                print!("{}", header);
            }
            print!("---\n");
            print!("{}", response.text().unwrap());
            io::stdout().flush().ok();
        }
        StatusCode::PayloadTooLarge => {
            println!("Request payload was too large!");
        }
        s => {
            println!("Response status was not Ok: {:?}", s);
        }
    }
}
