use app::RunConfig;
use reqwest::ClientBuilder;
use reqwest::Error as ReqwestError;
use reqwest::Method;
use reqwest::StatusCode;
use reqwest::Url;
use reqwest::UrlError;
use response::CompletedResponse;
use std::fmt;

/// A Request Command
pub struct Command {
    client: ClientBuilder,
    addr: Url,
    method: Method,
}

impl Command {
    pub fn new(addr: &str) -> Result<Command, CommandError> {
        let addr = addr.parse::<Url>()?;
        let client = ClientBuilder::new();
        let method = Method::GET;

        Ok(Command {
            addr,
            client,
            method,
        })
    }

    pub fn method(mut self, method: Method) -> Command {
        self.method = method;
        self
    }

    pub fn send(self) -> Result<CompletedResponse, CommandError> {
        let response = self
            .client
            .build()?
            .request(self.method, self.addr)
            .send()?;

        CompletedResponse::consume_response(response)
    }
}

impl<'a> From<&'a RunConfig> for Command {
    fn from(config: &'a RunConfig) -> Command {
        Command {
            method: config.method(),
            addr: config.url(),
            client: ClientBuilder::new(),
        }
    }
}

#[derive(Debug)]
pub enum CommandError {
    AddrParse(UrlError),
    Http(Option<StatusCode>),
    Reqwest(ReqwestError),
}

impl From<reqwest::UrlError> for CommandError {
    fn from(err: reqwest::UrlError) -> CommandError {
        CommandError::AddrParse(err)
    }
}

impl From<ReqwestError> for CommandError {
    fn from(err: ReqwestError) -> CommandError {
        if err.is_http() {
            CommandError::Http(err.status())
        } else {
            CommandError::Reqwest(err)
        }
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::AddrParse(err) => write!(f, "Unexpected url: {}", err),
            CommandError::Http(Some(status_code)) => {
                write!(f, "Error response from the server: {}", status_code)
            }
            CommandError::Http(None) => write!(f, "Http error"),
            CommandError::Reqwest(err) => write!(f, "Client error: {}", err),
        }
    }
}
