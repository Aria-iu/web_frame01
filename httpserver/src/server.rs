use std::{io::Read, net::TcpListener};

use http::httprequest::HttpRequest;
use super::router::Router;

pub struct Server<'a>{
    socket_addr: &'a str,
}

impl<'a> Server<'a>{
    pub fn new(addr: &'a str) -> Server<'a>{
        Server { socket_addr: addr, }
    }

    pub fn run(&self){
        let conn = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running On {}",self.socket_addr);
        for stream in conn.incoming(){
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buf = [0;90];
            // parse incoming message
            stream.read(&mut read_buf).unwrap();
            let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}