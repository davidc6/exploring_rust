use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}};
use serde::{Serialize, Deserialize};

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
    let  mut buf2 = BufReader::new(stream.try_clone().unwrap());

    // let buf3 = std::io::Cursor::new(stream);
    let mut buf4 = String::new();
    // let mut buf4 = vec![];
    // buf3.read_line(buf4);
    let mut should_exit = false;
    loop {
    let mut buf4 = String::new();

        let len = buf2.read_line(&mut buf4).unwrap();
        // println!("{:?}", std::str::from_utf8(buf2.buffer()));
        println!("{} {:?}", len, buf4);

        if buf4 == "\r\n" {
            let mut buf5 = [0; 42];

            should_exit = true;
            buf2.read_exact(&mut buf5);
            println!("{:?}", std::str::from_utf8(&buf5).unwrap());
            break;
        }

        // if should_exit {
        //     break;
        // }

    }

    // stream.write_all(&[1])?;
    // stream.read_exact(&mut [0; 512])?;
    // Ok(())
    // println!("{:?}", buf.lines());

    // let a: Vec<_> = buf.lines().map(|line| line.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("AAA");
    
    // 8096
    // let mut buf2 = [0; 173];
    // stream.read_exact(&mut buf2)?;



    println!("BBB");

    let mut body_vec: Vec<u8> = vec![];
    let mut counter = 0;

    // for byte in buf2.bytes() {
    //     if *byte.as_ref().unwrap() == 0 {
    //         break;
    //     }

    //     if counter == 4 {
    //         body_vec.push(byte.unwrap());
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


    // let s = match std::str::from_utf8(&buf2) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };

    // buf.lines()

    // let zero_pos = buf2.iter().find(|&&x| x == 0);

    // println!("{:?}", s);
    // println!("{:?}", buf2);

    // println!("{:?}", std::str::from_utf8(&body_vec).unwrap());
    let s = std::str::from_utf8(&body_vec);
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
