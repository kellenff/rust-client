use command::CommandError;
use response::CompletedResponse;
use std::fmt;

pub struct Presenter {
    colorize: bool,
    response: Result<CompletedResponse, CommandError>,
}

impl Presenter {
    pub fn new(response: Result<CompletedResponse, CommandError>) -> Presenter {
        Presenter {
            colorize: false,
            response,
        }
    }

    pub fn colorize(mut self, colorize: bool) -> Presenter {
        self.colorize = colorize;
        self
    }
}

impl fmt::Display for Presenter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Presenter {
                response: Ok(response),
                ..
            } => {
                // We're ignoring colorizing for now
                // write the headers
                for (name, value) in response.headers().iter() {
                    writeln!(f, "{}: {}", name, value.to_str().unwrap_or(""))?;
                }
                writeln!(f, "---")?;
                write!(f, "{}", response.text())
            }
            Presenter {
                response: Err(err), ..
            } => write!(f, "{}", err),
        }
    }
}

impl From<Result<CompletedResponse, CommandError>> for Presenter {
    fn from(res: Result<CompletedResponse, CommandError>) -> Presenter {
        Presenter {
            colorize: false,
            response: res,
        }
    }
}
