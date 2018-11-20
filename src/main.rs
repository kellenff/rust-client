extern crate reqwest;
extern crate rust_client;

use rust_client::app::run_config;
use std::time;
use rust_client::command::Command;
use rust_client::presenter::Presenter;

fn main() {
    let config = run_config();

    println!("{} {}", config.method(), config.url());

    let command = Command::from(&config);

    let now = time::Instant::now();
    let maybe_response = command.send();
    let elapsed = now.elapsed();
    println!(
        "{:.4}s",
        (elapsed.as_secs() as f64 + f64::from(elapsed.subsec_nanos()) * 1e-9),
    );

    let presenter = Presenter::from(maybe_response);

    print!("{}", presenter);
}
