#[macro_use]
extern crate criterion;
extern crate docopt;
extern crate mockito;
extern crate rust_client;

use criterion::Criterion;

use docopt::Docopt;
use rust_client::app::Args;
use rust_client::app::{RunConfig, USAGE};
use rust_client::command::Command;

const URL: &'static str = mockito::SERVER_URL;

fn get_request(config: RunConfig) {
    let command = Command::from(&config);
    let _res = command.send();
}

fn get_benchmark(c: &mut Criterion) {
    let argv = || vec!["rc", "get", URL].into_iter();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv()).deserialize())
        .unwrap();
    let config = RunConfig::from(args);

    c.bench_function("get /", move |b| b.iter(|| get_request(config.clone())));
}

criterion_group!(benches, get_benchmark);
criterion_main!(benches);
