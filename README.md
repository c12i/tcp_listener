# helge
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.0.0-blue.svg?cacheSeconds=2592000" />
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

> A single threaded HTTP/1.1  server. Named after Helge Doppler from Netflix series Dark

## How it works
```rs
use std::env;
use server::Server;
use request_handler::RequestHandler;

fn main() {
  // set path to static files
  let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
  
  // public path will either be path set as env var or the initial default path
  let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

  // create server instance and set address as localhost:8080
  let server = Server::new(String::from("127.0.0.1:8080"));

  // server listens to connections
  server.run(RequestHandler::new(public_path));
}
```

## Usage

```sh
cargo run
```

## Run tests

```sh
cargo test
```