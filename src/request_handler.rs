use super::server::Handler;
use super::http::{Request, Response,StatusCode,Method};
use macroz::tostr;

pub struct RequestHandler {
    public_path: String,
}

impl RequestHandler {
    pub fn new(public_path: String) -> Self {
        Self {
            public_path
        }
    }
}

impl Handler for RequestHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some(tostr!("<h1>Home page</h1>"))),
                "/hello" => Response::new(StatusCode::Ok, Some(tostr!("<h1>Hello</h1>"))),
                _ => Response::new(StatusCode::NotFound, None)
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}