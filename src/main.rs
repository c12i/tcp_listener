#![allow(dead_code)]

mod server;
mod http;
mod utils;

use macroz::tostr;
use server::Server;

fn main() {
    let server = Server::new(tostr!("127.0.0.1:8080"));
    server.run();
}
