use docopt::Docopt;
use reqwest::Method;
use reqwest::Url;

pub const USAGE: &str = r#"
Usage: rc [(get|post)] [options] <address> [<body>]

Options:
    --nocolor   Do not colorize output
"#;

pub fn run_config() -> RunConfig {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    RunConfig::from(args)
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Args {
    cmd_get: bool,
    cmd_post: bool,
    flag_nocolor: bool,
    arg_address: String,
    arg_body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RunConfig {
    url: Url,
    method: Method,
    raw_body: Option<String>,
    color: bool,
}

// TODO: Figure out lifetime issues around returning reference to properties; I'd rather not clone all over the place
impl RunConfig {
    pub fn color(&self) -> bool {
        self.color
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn body_str(&self) -> Option<&str> {
        self.raw_body.as_ref().map(|s| s.as_str())
    }
}

impl From<Args> for RunConfig {
    fn from(args: Args) -> RunConfig {
        let method = if args.cmd_post {
            Method::POST
        } else {
            Method::GET
        };
        let url = uri_with_added_missing_scheme(&args.arg_address);
        let raw_body = args.arg_body;
        let color = !args.flag_nocolor;

        RunConfig {
            method,
            url,
            raw_body,
            color,
        }
    }
}

fn uri_with_added_missing_scheme(addr: &str) -> Url {
    if addr.starts_with("http") {
        addr.parse::<Url>().expect("Invalid URL")
    } else {
        let addr = "http://".to_owned() + addr;
        addr.parse::<Url>().expect("Invalid URL")
    }
}

#[cfg(test)]
mod tests {
    use super::{RunConfig, USAGE};
    use app::Args;
    use docopt::Docopt;
    use reqwest::Method;

    #[test]
    fn run_config_explicit_method() {
        let get_args = || vec!["rc", "get", "http://localhost:8000"].into_iter();
        let post_args = || vec!["rc", "post", "http://localhost:8000"].into_iter();

        let get_matches: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(get_args()).deserialize())
            .unwrap();
        let get_config = RunConfig::from(get_matches);
        assert_eq!(get_config.method(), Method::GET);

        let post_matches: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(post_args()).deserialize())
            .unwrap();
        let post_config = RunConfig::from(post_matches);
        assert_eq!(post_config.method(), Method::POST);
    }

    #[test]
    fn config_appends_missing_http() {
        let addr = "localhost:8000";
        let args = || vec!["rc", "get", addr].into_iter();
        let matches: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(args()).deserialize())
            .unwrap();
        let config = RunConfig::from(matches);

        assert_eq!(
            config.url().to_string(),
            "http://localhost:8000/".to_owned()
        );
    }

    #[test]
    fn config_includes_body() {
        let args = || vec!["rc", "post", "localhost:8000", r#"{"foo": "bar"}"#].into_iter();
        let matches: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(args()).deserialize())
            .unwrap();
        let config = RunConfig::from(matches);

        assert_eq!(config.body_str(), Some(r#"{"foo": "bar"}"#));
    }

    #[test]
    fn nocolor_flag_respected() {
        let args = || vec!["rc", "get", "--nocolor", "localhost:8000"].into_iter();
        let matches: Args = Docopt::new(USAGE)
            .and_then(|d| d.argv(args()).deserialize())
            .unwrap();
        let config = RunConfig::from(matches);

        assert!(!config.color());
    }
}
