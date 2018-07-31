#[macro_use]
extern crate criterion;
extern crate mockito;
extern crate rust_client;

use criterion::Criterion;

use rust_client::app::cli_app;
use rust_client::app::RunConfig;

const URL: &'static str = mockito::SERVER_URL;

fn get_request(config: RunConfig) {
    let _resp = config.execute();
}

fn get_benchmark(c: &mut Criterion) {
    let args = ["rc", "get", URL];
    let app = cli_app();
    let matches = app.get_matches_from(&args);
    let config = RunConfig::from(matches);

    c.bench_function("get /", move |b| b.iter(|| get_request(config.clone())));
}

criterion_group!(benches, get_benchmark);
criterion_main!(benches);