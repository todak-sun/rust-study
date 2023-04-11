use super::HttpStatusCode;
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    status_code: HttpStatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: HttpStatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(content) => content,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
