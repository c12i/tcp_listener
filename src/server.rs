use std::net::TcpListener;
use std::io::{Read,Write};
use std::convert::TryFrom;
use super::http::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server {
            address: address.to_string()
        }
    }

    pub fn run(self) {
        println!("Server running on http://{}", self.address);
        let listener = TcpListener::bind(&self.address).expect("Error binding TcpListener");

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // TODO increase Buffer capacity in production
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                },
                                Err(err) => println!("Failed to parse a request: {}", err),
                            };
                        },
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                },
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}