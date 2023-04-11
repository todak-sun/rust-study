use crate::http::{HttpStatusCode, Request, Response};
use std::convert::TryFrom;

use std::io::{Read, Write};
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("server running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => {
                                    dbg!(req);
                                    Response::new(
                                        HttpStatusCode::Ok,
                                        Some("<h1>Hello World</h1>".to_owned()),
                                    )
                                }
                                Err(e) => Response::new(HttpStatusCode::BadRequest, None),
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("{:?}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("error occurred. {:?}", e);
                }
            }
        }
    }
}
