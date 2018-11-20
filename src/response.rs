use command::CommandError;
use hyper::header::HeaderMap;
use hyper::header::CONTENT_TYPE;
use mime::Mime;
use reqwest::Response;
use reqwest::StatusCode;

pub struct CompletedResponse {
    headers: HeaderMap,
    mime: Mime,
    response_bytes: Vec<u8>,
    status_code: StatusCode,
}

impl CompletedResponse {
    pub fn consume_response(mut response: Response) -> Result<CompletedResponse, CommandError> {
        let headers = response.headers().to_owned();
        let mime_type = {
            headers
                .get(CONTENT_TYPE)
                .map(|content_type| {
                    content_type
                        .to_str()
                        .unwrap_or("")
                        .parse::<Mime>()
                        .unwrap_or(::mime::TEXT_PLAIN)
                }).unwrap_or(::mime::TEXT_PLAIN)
        };
        let mut response_bytes = vec![];
        response.copy_to(&mut response_bytes)?;
        let status_code = response.status();

        Ok(CompletedResponse {
            headers,
            mime: mime_type,
            response_bytes,
            status_code,
        })
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.response_bytes).to_string()
    }

    pub fn status(&self) -> StatusCode {
        self.status_code
    }

    pub fn content_type(&self) -> Mime {
        self.mime.clone()
    }
}
