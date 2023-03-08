use std::net::{TcpListener, TcpStream};
use std::convert::TryFrom;
use std::io::{Read};
use crate::http::{Request, Response, StatusCode, ParserError};

pub trait Handler {
        fn handle_request(&mut self, request: &Request) -> Response;
        fn handle_bad_request(&mut self, e: &ParserError) -> Response {
            println!("Failed to handle request: {:?}", e);
            Response::new(StatusCode::BadRequest, None)
        }
}


pub struct Server{
    addr: String
}

impl Server {
    // returns a new instance of this Server struct
    pub fn new(addr: String) -> Self{
        Self { addr }
    }
    
    // starts an infinite loop that accepts server connections 
    pub fn run(self, mut handler: impl Handler){
        println!("Server listening on: {}", &self.addr[10..]);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => Self::handle_client(&mut stream, &mut handler),
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }

    fn handle_client(stream: &mut TcpStream, handler: &mut impl Handler){
        // Only read a buffer of up to 1024 bytes
        let mut buffer = [0; 1024];
        // read from stream using match to get an oK from the result returned from read
        match stream.read(&mut buffer){
            Ok(_)=> {
                // using the Request try_from method to read the buffer -> goto Request.rs
                let response = match Request::try_from(&buffer[..]){
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e)
                };

                if let Err(e) = response.send(stream){
                    println!("Failed to send response: {}", e);
                }
            },
            Err(e) => println!("Failed to read from connection: {}", e)
        }
    }
}