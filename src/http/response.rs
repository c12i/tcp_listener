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