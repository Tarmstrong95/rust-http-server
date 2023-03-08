
#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
    // create default path incase public path isnt provided
    let defualt_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // if public path is provided we unwrap it or default to default path
    let public_path = env::var("PUBLIC").unwrap_or(defualt_path);

    let address = String::from("localhost:8080");
    let server = Server::new(address);
    // instantiate new website handler instance with "public path" vairable as parameter
    server.run(WebsiteHandler::new(public_path));
}
    
/*
GET /user?id=10 HTTP/1.1\r\n
 */