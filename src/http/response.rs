use std::fmt::{Display,Formatter,Result as FmtResult};
use std::io::{Write, Result as IoResult};
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

    pub fn send<T:Write>(&self, stream: &mut T) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        // write to stream w/o allocating memory to the heap
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status, self.status.reason_phrase(),body)
    }
}

impl Display for Response {
    fn fmt(&self, _f: &mut Formatter) -> FmtResult {
        unimplemented!()
    }
}