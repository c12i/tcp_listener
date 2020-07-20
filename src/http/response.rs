use std::fmt::{Display,Formatter,Result as FmtResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        Response {
            status,
            body,
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status, self.status.reason_phrase(),body)
    }
}