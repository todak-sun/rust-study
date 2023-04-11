#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = env!("CARGO_MANIFEST_DIR");
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
