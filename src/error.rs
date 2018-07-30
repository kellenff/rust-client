use reqwest;
use reqwest::Method;
use std::error::Error;
use std::fmt;

pub type RequestResult<T> = Result<T, RequestError>;

#[derive(Debug)]
pub enum RequestError {
    Reqwest(reqwest::Error),
    UnsupportedMethod(Method),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RequestError::Reqwest(ref err) => err.fmt(f),
            RequestError::UnsupportedMethod(ref m) => write!(f, "Method {} not supported", m),
        }
    }
}

impl Error for RequestError {
    fn description(&self) -> &str {
        match *self {
            RequestError::Reqwest(ref err) => err.description(),
            RequestError::UnsupportedMethod(_) => "Method does not live long enough",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RequestError::Reqwest(ref err) => Some(err),
            RequestError::UnsupportedMethod(_) => None,
        }
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> RequestError {
        RequestError::Reqwest(err)
    }
}
