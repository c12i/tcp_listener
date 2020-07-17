use std::net::TcpListener;

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
        let listener = TcpListener::bind(&self.address);
    }
}