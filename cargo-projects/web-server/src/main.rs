use std::{net::{TcpListener, TcpStream}};
use std::io::{BufReader, BufRead, Write, Read};
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json::Value;

mod request;

use crate::request::{Request};

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    #[serde(flatten)]
    fields: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    http_version: String,
    status_code: String,
    headers: Option<HashMap<String, String>>,
    body: String
}

#[derive(Serialize, Deserialize, Debug)]
struct BodyResponse<'a> {
    name: &'a str,
    message: &'a str
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.http_version, self.status_code)
    }
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

// fn build_res() {
//     response::new()
//         .
// }

fn build_response(request: request::Request) -> String {
    let mut headers_map: HashMap<String, String> = HashMap::new();
    headers_map.insert("content-type".to_owned(), "application/json".to_owned());

    let body = BodyResponse {
        name: "New user",
        message: "This is my message"
    };

    let parsed = serde_json::to_string(&body).unwrap();

    let res = Response {
        http_version: request.http_version(),
        headers: Some(headers_map),
        status_code: "200".to_string(),
        body: "abc".to_string(),
    };

    let http_version = res.http_version;
    let mut headers = "".to_owned();

    for (k, v) in res.headers.unwrap().into_iter() {
        headers.push_str(&k);
        headers.push_str(": ");
        headers.push_str(&v);
    }

    format!("{http_version} 200 OK\r\n{headers}\r\n\r\n{parsed}\r\n")
}

fn handle_test(mut stream: TcpStream) -> std::io::Result<()> {
    // creates independently owned handle which references the same stream
    let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());

    let mut body_data = Vec::new();
    let mut request = Request::builder();

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
            request.header((header_name, header_value));
            continue;
        }

        // first / request line of the HTTP message 
        let (method, uri, http_version) = process_method(line);
        request
            .method(method)
            .uri(uri)
            .version(http_version);
    }

    let body_string_slice = std::str::from_utf8(&body_data);
    let deserialised_request: Body = serde_json::from_str(body_string_slice.unwrap()).unwrap();

    let response = build_response(request.body());
    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}

fn main() {
    // bind (connect) socket to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterator over streams (open client / server connection)
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_test(stream);
    }
}
