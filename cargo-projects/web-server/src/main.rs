use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};

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

struct Response {
    // http_version: 
    // status_code: 
    // headers:
    // body:
}

fn process_method(line: String) -> Vec<String> {
    line.split(" ").map(String::from).collect()
}

fn process_connection(mut stream: TcpStream) {
    let mut request = Request {
        method: String::from(""),
        uri: String::from(""),
        http_version: String::from(""),
        headers: vec![]
    };
    
    // create new buffer to read the stream that wraps mutatable a mutable red to TcpStream
    let buf = BufReader::new(&mut stream);
    let (req): Vec<_> = buf
        .lines() // streams of data are split here on newline byte
        .enumerate()
        .map(|(index, line)| {
            let end = match line {
                Ok(line) => {
                    let a = match index {
                        0 => {
                            let res = process_method(line);

                            request.method(res.get(0).unwrap().to_string());
                            request.uri(res.get(1).unwrap().to_string());
                            request.http_version(res.get(2).unwrap().to_string());
                            String::from("a")
                        },
                        _ => {
                            let b = match request.method {
                                _ if request.method == String::from("GET") && !line.is_empty() => {
                                    request.headers.push(line);
                                    String::from("a")
                                },
                                _ => {
                                    String::from("")
                                }
                            };

                            b
                        }
                    };
                    a
                },
                Err(e) => {
                    let val = match e.kind() {
                        std::io::ErrorKind::InvalidData => String::from("Check your requets for non-UTF8 data"),
                        _ => String::from("There has been an error, check again later")
                    };
                    val
                }
            };
            println!("{:?}", end);
            end
        })
        .take_while(|line| !line.is_empty())
        .collect();

        println!("Hello {:?}", request);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(response.as_bytes()).unwrap();

    // println!("{:?}", http_req);
}

fn main() {
    // bind (connect) socket to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterator over streams (open client / server connection)
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        process_connection(stream);
        println!("Connection established");
    }
}
