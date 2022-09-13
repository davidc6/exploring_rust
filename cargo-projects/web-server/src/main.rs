use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}};
use serde::{Serialize, Deserialize};

enum HttpVersion {
    One,
    Two
}

#[derive(Debug)]
struct Request {
    method: String,
    uri: String,
    http_version: String,
    headers: Vec<String>
    // body: 
}

impl Request {
    fn method(&mut self, val: String) {
        self.method = val;
    }

    fn uri(&mut self, val: String) {
        self.uri = val;
    }

    fn http_version(&mut self, val: String) {
        self.http_version = val;
    }
}

#[derive(Serialize, Deserialize, Debug)]

struct Test {
    name: String,
    email: String
}

struct Response {
    // http_version: 
    // status_code: 
    // headers:
    // body:
}

fn process_method(line: String) -> Vec<String> {
    line.split(" ").map(String::from).collect()
}

fn main() {
    // bind (connect) socket to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("STARTED");

    // iterator over streams (open client / server connection)
    for stream in listener.incoming() {
    println!("INSIDE");

        let stream = stream.unwrap();
        // println!("{:?}", stream);        
        // process_connection(stream);
        handle_test(stream);
        println!("Connection established");
    }
}
