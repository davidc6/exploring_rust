use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}, f32::consts::E, hash::Hash};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    prod: u8,
    thisIs: String
    // email: String
}

struct Response {
    // http_version: 
    // status_code: 
    // headers:
    // body:
}

fn process_method(line: String) -> Vec<String> {
    line.split(' ').map(String::from).collect()
}

fn process_header<'a>(header: String) -> (String, String) {
    // header name, value, end of line
    let clean_header: Vec<_> = header.split("\r\n").collect();

    // if clean_header.len() > 0 {
    let parts: Vec<_> = clean_header[0].split(":").collect();
    println!("Header {} {}", parts[0], parts[1]);

    // }

    // if parts[0].contains("Content-Length") {
    //     return parts[1].trim_start();
    // }

    (parts[0].to_string(), parts[1].trim_start().to_string())
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

    println!("{:?}", buf);

    let (req): Vec<_> = buf
        .lines() // streams of data are split here on newline byte
        .enumerate()
        .map(|(index, line)| {
            println!("{:?}", line);

            let end = match line {
                Ok(line) => {
                    match index {
                        0 => {
                            let res = process_method(line);

                            request.method(res.get(0).unwrap().to_string());
                            request.uri(res.get(1).unwrap().to_string());
                            request.http_version(res.get(2).unwrap().to_string());

                            String::from("a")
                        },
                        _ => {
                            match request.method {
                                _ if request.method == *"GET" && !line.is_empty() => {
                                    request.headers.push(line);
                                    String::from("a")
                                },
                                _ if request.method == *"POST" && !line.is_empty() => {
                                    request.headers.push(line);
                                    String::from("a")
                                },
                                _ => {
                                    String::from("")
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::InvalidData => String::from("Check your requets for non-UTF8 data"),
                        _ => String::from("There has been an error, check again later")
                    }
                }
            };

            end
        })
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_test(mut stream: TcpStream) -> std::io::Result<()> {
    // creates independently owned handle which references the same stream
    let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());

    // body buffer - TODO
    let mut map: HashMap<String, String> = HashMap::new();
    let mut buf5 = [0; 12];
    let mut data = Vec::new();

    loop {
        let mut line = String::new();
        let num_bytes_read = buffered_stream.read_line(&mut line).unwrap();

        if num_bytes_read == 2 {
            let mut buf10 = [0; 4096];

            loop {
                let n = buffered_stream.read(&mut buf10)?;

                println!("SIZE {:?}", n);

                if n == 0 {
                    println!("BREAK");
                    break;
                }

                data.extend_from_slice(&buf10[..n]);
                if 4096 > n {
                    break;
                }
            }
            break;
        }

        // parsing takes place
        if !line.contains("HTTP") {
            let (name, value) = process_header(line);
            map.insert(name, value);
        }



    }
    println!("{:?}", data);
    // end of request
    // if num_bytes_read == 2 {
    //     let length = map.get("Content-Length").unwrap().parse().unwrap();

    //     let mut counter = 0;

    //     loop {
    //         let mut buf7 = [0; 4];

    //         // counter.push()

    //         body.push_str(std::str::from_utf8(&buf7).unwrap());
    //         counter += 4;

    //     }




                // buffered_stream.read_exact(&mut buf5);

                // println!("{:?}", std::str::from_utf8(&buf5).unwrap());

                // println!("{}", length);

                // break;
            // }



            // println!("{:?}", std::str::from_utf8(buffered_stream.buffer()));

            // if line == "\r\n" {    
            //     buffered_stream.read_exact(&mut buf5);

            //     println!("{:?}", std::str::from_utf8(&buf5).unwrap());

            //     break;
            // }

            // println!("{} {:?}", num_bytes_read, line);

        // }

    // stream.write_all(&[1])?;
    // stream.read_exact(&mut [0; 512])?;
    // Ok(())
    // println!("{:?}", buf.lines());

    // let a: Vec<_> = buf.lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect();
    
    // 8096
    // let mut buffered_stream = [0; 150];
    // stream.read_exact(&mut buffered_stream)?;

    let mut body_vec: Vec<u8> = vec![];
    let mut counter = 0;

    // for byte in buffered_stream.bytes() {
    //     if *byte.as_ref().unwrap() == 0 {
    //         break;
    //     }

    //     if counter == 4 {
    //         body_vec.push(9);
    //         continue;
    //     }

    //     if *byte.as_ref().unwrap() == 13 {
    //         counter += 1;
    //         continue;
    //     }

    //     if *byte.as_ref().unwrap() == 10 {
    //         counter += 1;
    //         continue;
    //     }

    //     counter = 0;
    // }


    // let s = match std::str::from_utf8(&buffered_stream) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };

    // buf.lines()

    // let zero_pos = buffered_stream.iter().find(|&&x| x == 0);

    // println!("{:?}", s);
    // println!("{:?}", buffered_stream);
    // println!("{:?}", std::str::from_utf8(&body_vec).unwrap());

    let s = std::str::from_utf8(&data);

    // let deserialized: Test = serde_json::from_str(std::str::from_utf8(&body_vec).unwrap()).unwrap();
    let des: Test = serde_json::from_str(s.unwrap()).unwrap();

    println!("{:?}", des);
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}

fn main() {
    // bind (connect) socket to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterator over streams (open client / server connection)
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("{:?}", stream);        
        // process_connection(stream);
        handle_test(stream);
        println!("Connection established");
    }
}
