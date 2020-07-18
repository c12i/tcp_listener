use std::net::TcpListener;
use std::io::Read;

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
                    let mut buffer = [0; 1024]; // TODO increase Buffer in production
                    stream.read(&buffer);
                },
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}