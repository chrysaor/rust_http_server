use crate::http::{Request};
use std::convert::TryFrom;
use std::io::{Read};
use std::net::TcpListener;
use crate::handler::handler::Handler;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // TODO: Single thread
            match listener.accept() {
                Ok((mut stream, _)) => {

                    // TODO: Buffer size
                    let mut buffer = [0; 2048];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let received_msg = String::from_utf8_lossy(&buffer);
                            println!("Received: {}", received_msg);

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => { handler.handle_request(&request) }
                                Err(e) => { handler.handle_bad_request(&e) }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to parse request: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
