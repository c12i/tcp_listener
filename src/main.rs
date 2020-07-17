mod server {
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
        }
    }
}

mod http {
    mod request {
        pub struct Request {
            path: String,
            query: Option<String>,
            method: super::method::Method,
        }
    }

    mod response {

    }

    mod method {
        #[derive(Debug)]
        pub enum Method {
            GET,
            POST,
            PUT,
            PATCH,
            DELETE,
            OPTIONS,
            TRACE,
            CONNECT,
            HEAD
        }
    }
}

use server::Server;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}
