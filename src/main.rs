#![allow(dead_code)]

mod server;
mod http;
mod utils;
mod request_handler;

use macroz::tostr;
use server::Server;
use request_handler::RequestHandler;

fn main() {
    let server = Server::new(tostr!("127.0.0.1:8080"));
    server.run(RequestHandler);
}
