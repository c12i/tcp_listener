use std::net::TcpListener;
use std::io::{Read,Write};
use std::convert::TryFrom;
use super::http::{Request,Response,StatusCode};

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
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(StatusCode::Ok, Some(String::from("<h1>Hello World</h1>")))
                                },
                                Err(err) => {
                                    println!("Failed to parse a request: {}", err);
                                    Response::new(StatusCode::BadRequest, None).send(&mut stream)
                                },
                            };

                            if let Err(err) = response.send(&mut stream) {
                                println!("Failed to send response, {}", err);
                            }
                        },
                        Err(err) => println!("Failed to read from connection: {}", err),
                    }
                },
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}