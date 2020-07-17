struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Server {
            address: address.to_string()
        }
    }

    fn run(self) {
        println!("Server running on http://{}", self.address);
    }
}

fn main() {
    let server = Server::new(tostr!("127.0.0.1:8080"));
    server.run();
}

#[macro_export]
macro_rules! tostr {
    ($input:expr) => {
        String::from($input)
    };
}