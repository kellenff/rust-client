use clap::App;
use clap::Arg;
use clap::ArgMatches;
use error::RequestResult;
use request_url;
use reqwest::Method;
use reqwest::Response;
use reqwest::Url;
use std::str::FromStr;
use uri_with_added_missing_scheme;

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
            Arg::with_name("METHOD")
                .help("The method used in the request")
                .possible_values(&["get", "post", "head"])
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("URL")
                .help("The URL to send the request to")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("BODY")
                .help("An optional literal body for the request")
                .index(3),
        )
}

#[derive(Debug)]
pub struct RunConfig<'a> {
    raw_matches: ArgMatches<'a>,
    url: Url,
    method: Method,
    raw_body: Option<String>,
}

// TODO: Figure out lifetime issues around returning reference to properties; I'd rather not clone all over the place
impl<'a> RunConfig<'a> {
    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn body_str(&'a self) -> Option<&'a str> {
        self.raw_body.as_ref().map(|s| s.as_str())
    }

    pub fn execute(&self) -> RequestResult<ExecutionResponse> {
        let response = request_url(self.url(), self.method(), self.raw_body.clone())?;
        Ok(ExecutionResponse { response })
    }
}

impl<'a> From<ArgMatches<'a>> for RunConfig<'a> {
    fn from(matches: ArgMatches<'a>) -> RunConfig<'a> {
        let method = matches
            .value_of("METHOD")
            .map(|method| {
                Method::from_str(&method.to_ascii_uppercase()).expect("Incompatible HTTP method")
            })
            .expect("METHOD is a required argument");

        let url = matches
            .value_of("URL")
            .map(|addr| uri_with_added_missing_scheme(addr))
            .expect("URL is a required argument");

        let raw_body = matches.value_of("BODY").map(|s| s.to_string());

        RunConfig {
            raw_matches: matches,
            url,
            method,
            raw_body,
        }
    }
}

pub struct ExecutionResponse {
    pub response: Response,
}

#[cfg(test)]
mod tests {
    use super::{cli_app, RunConfig};
    use reqwest::Method;

    #[test]
    fn run_config_explicit_method() {
        let app = cli_app();
        let get_args = vec!["rc", "get", "http://localhost:8000"];
        let post_args = vec!["rc", "post", "http://localhost:8000"];
        let head_args = vec!["rc", "head", "http://localhost:8000"];

        let get_matches = app.clone().get_matches_from(get_args);
        let get_config = RunConfig::from(get_matches);
        assert_eq!(get_config.method(), Method::Get);

        let post_matches = app.clone().get_matches_from(post_args);
        let post_config = RunConfig::from(post_matches);
        assert_eq!(post_config.method(), Method::Post);

        let head_matches = app.clone().get_matches_from(head_args);
        let head_config = RunConfig::from(head_matches);
        assert_eq!(head_config.method(), Method::Head);
    }

    #[test]
    fn config_appends_missing_http() {
        let addr = "localhost:8000";
        let args = vec!["rc", "get", addr];
        let matches = cli_app().get_matches_from(args);
        let config = RunConfig::from(matches);

        assert_eq!(
            config.url().to_string(),
            "http://localhost:8000/".to_owned()
        );
    }

    #[test]
    fn config_includes_body() {
        let args = vec!["rc", "post", "localhost:8000", r#"{"foo": "bar"}"#];
        let matches = cli_app().get_matches_from(args);
        let config = RunConfig::from(matches);

        assert_eq!(config.body_str(), Some(r#"{"foo": "bar"}"#));
    }
}
