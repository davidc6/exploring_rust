use std::{net::{TcpListener, TcpStream}};
use std::io::{BufReader, BufRead, Write, Read};
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use serde_json::Value;

mod request;
mod response;

use crate::request::{Request};
use crate::response::{Response};

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    #[serde(flatten)]
    fields: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseOld {
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

impl fmt::Display for ResponseOld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.http_version, self.status_code)
    }
}

// method, uri, version
fn process_request_line(line: String) -> (String, String, String) {
    let separated: Vec<String> = line.trim_end().split(' ').map(String::from).collect();

    (separated[0].to_owned(), separated[1].to_owned(), separated[2].to_owned())
}

// header name, header value
fn process_header(header: String) -> (String, String) {
    let clean_header: Vec<_> = header.split("\r\n").collect();
    let parts: Vec<_> = clean_header[0].split(':').collect();
    (parts[0].to_owned(), parts[1].trim_start().to_owned())
}

fn build_response2(request: Request) -> String {
    let body = BodyResponse {
        name: "New user",
        message: "This is my message"
    };
    let parsed = serde_json::to_string(&body).unwrap();

    let response = Response::builder()
        .header(("content-type".into(), "application/json".into()));

    // response.header());

    let r = response
        .method(request.method())
        .status(&"200".to_string())
        .version(request.http_version())
        .body(parsed);

    // {version} {status} OK \r\n {headers} \r\n\r\n {body} \r\n
    format!("{} 200 OK\r\n{}\r\n{}\r\n", r.version(), r.headers(), r.body())
}

fn build_response(request: request::Request) -> String {
    let mut headers_map: HashMap<String, String> = HashMap::new();
    headers_map.insert("content-type".to_owned(), "application/json".to_owned());

    let res = ResponseOld {
        http_version: request.http_version().to_owned(),
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

    let body = BodyResponse {
        name: "New user",
        message: "This is my message"
    };

    let parsed = serde_json::to_string(&body).unwrap();

    format!("{http_version} 200 OK\r\n{headers}\r\n\r\n{parsed}\r\n")
}

fn send_response(res: String) {
    let r = String::from(res);
}

fn process_stream(mut stream: TcpStream) -> std::io::Result<()> {
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

        // lines following the first line, i.e. headers
        if !line.contains("HTTP") {
            let (name, value) = process_header(line);
            request.header((name, value));
            continue;
        }

        // first / request line of the HTTP message - e.g. POST / HTTP/1.1
        let (method, uri, http_version) = process_request_line(line);
        request
            .method(method)
            .uri(uri)
            .version(http_version);
    }

    let body_string_slice = std::str::from_utf8(&body_data);
    let deserialised_request: Body = serde_json::from_str(body_string_slice.unwrap()).unwrap();

    // let response = build_response(request.body());
    let response = build_response2(request.body());
    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}

fn main() {
    // bind (connect) socket to the address and port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // iterator over streams (open client / server connection)
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        process_stream(stream);
    }
}
