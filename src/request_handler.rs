use super::server::Handler;
use super::http::{Request, Response};
use crate::http::StatusCode;

use macroz::tostr;

pub struct RequestHandler;

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some(tostr!("<h1>TEST</h1>")))
    }
}