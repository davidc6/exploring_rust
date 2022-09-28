use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Read}, f32::consts::E, hash::Hash};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;

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
struct Body {
    #[serde(flatten)]
    fields: HashMap<String, Value>,
}

struct Response {
    http_version: String,
    status_code: String,
    headers: Option<Vec<String>>,
    body: String
}

fn process_method(line: String) -> (String, String, String) {
    let separated: Vec<String> = line.trim_end().split(' ').map(String::from).collect();

    (separated[0].to_owned(), separated[1].to_owned(), separated[2].to_owned())
}

fn process_header(header: String) -> (String, String) {
    let clean_header: Vec<_> = header.split("\r\n").collect();
    let parts: Vec<_> = clean_header[0].split(':').collect();
    (parts[0].to_owned(), parts[1].trim_start().to_owned())
}

fn build_response(request: Request) {
    let res = Response {
        http_version: request.http_version,
        headers: None,
        body: "abc".to_string(),
        status_code: "200".to_string()
    };
}

fn handle_test(mut stream: TcpStream) -> std::io::Result<()> {
    // creates independently owned handle which references the same stream
    let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());

    let mut headers_map: HashMap<String, String> = HashMap::new();
    let mut body_data = Vec::new();
    let mut request = Request {
        method: String::from(""),
        uri: String::from(""),
        http_version: String::from(""),
        headers: vec![]
    };

    loop {
        let mut line = String::new();
        let bytes_read = buffered_stream.read_line(&mut line)?;

        // same as new empty line (\r\n) which is 2 bytes  
        if bytes_read == 2 {
            // big enough buffer size should allow to store body
            const BUFFER_SIZE: usize = 4096;
            let mut body_buffer = [0; BUFFER_SIZE];

            loop {
                let bytes_read = buffered_stream.read(&mut body_buffer)?;

                if bytes_read == 0 {
                    break;
                }

                body_data.extend_from_slice(&body_buffer[..bytes_read]);

                if BUFFER_SIZE > bytes_read {
                    break;
                }
            }

            break;
        }

        if !line.contains("HTTP") {
            let (header_name, header_value) = process_header(line);
            headers_map.insert(header_name, header_value);
            continue;
        }

        let (method, uri, http_ver) = process_method(line);
        
        request.method(method);
        request.uri(uri);
        request.http_version(http_ver);
    }

    let body_string_slice = std::str::from_utf8(&body_data);
    let deserialised_request: Body = serde_json::from_str(body_string_slice.unwrap()).unwrap();

    println!("REQUEST {:?}", deserialised_request.fields);
    let response = "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n{\"hello\":\"one\"}\r\n";

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
