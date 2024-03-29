use std::{net::{TcpListener, TcpStream}, io::Error, time::Duration};
use std::io::{BufReader, BufRead, Write, Read};
use serde::{Serialize, Deserialize};

mod request;
mod response;
mod pool;

use crate::request::{Request};
use crate::response::{Response};
use crate::pool::{ThreadPool};

#[derive(Serialize, Deserialize, Debug)]
struct BodyResponse<'a> {
    name: &'a str,
    message: &'a str
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

fn build_response(request: Request) -> Response {
    // Assuming it's a POST request here
    let body = if request.uri() == &String::from("/") {
        // artificially delay the response
        std::thread::sleep(Duration::from_secs(10));

        BodyResponse {
            name: "Some name",
            message: "Some message"
        }
    } else {
        BodyResponse {
            name: "Some name 2",
            message: "Some message 2"
        }
    };
    let parsed = serde_json::to_string(&body).unwrap();
    Response::builder()
        .header(("Content-Type".into(), "application/json".into())) // 
        .header(("Content-Length".to_string(), (parsed.len() + 2).to_string()))
        .header(("Connection".to_string(), "close".to_string()))
        .method(request.method())
        .status(&"200".to_string())
        .version(request.http_version())
        .body(parsed)
}

fn send_response(mut stream: TcpStream, response: Response) -> Result<(), Error> {
    let s = format!(
        "{ver} {status} OK\r\n{head}\r\n{body}\r\n",
        ver = response.version(),
        status = response.status(),
        head = response.headers(),
        body = response.body()
    );
    stream.write_all(s.as_bytes())?;
    stream.flush().unwrap();
    Ok(())
}

fn process_stream(stream: TcpStream) -> std::io::Result<()> {
    // creates independently owned handle which references the same stream
    let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());

    let mut body_data = Vec::new();
    let mut request = Request::builder();

    loop {
        let mut line = String::new();
        // read message line
        let bytes_read = buffered_stream.read_line(&mut line)?;

        // same as new empty line (\r\n) which is 2 bytes, empty line means that the line contains the body
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

        // first / request line of the HTTP message - e.g. "POST / HTTP/1.1"
        let (method, uri, http_version) = process_request_line(line);
        request
            .method(method)
            .uri(uri)
            .version(http_version);
    }

    let response = build_response(request.body());
    send_response(stream, response)?;

    Ok(())
}

fn main() {
    // TODO - to implement
    let pool = ThreadPool::new(4);

    // bind (connect) socket to address and port
    // it is now listening to the incoming streams
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // a single stream represents an open connection in which a client 
    // connects to a server and the server responds and closes the connection.
    // first a stream is read and then a response is writtenback to the stream
    // iterator over TcpStream type streams (open client / server connection)
    // these are actually connection attempts as a connection attempt might fail
    // due to some OS specific limitations such as for instance a limit of open connections
    // add take(2) to incoming() to gracefully shut the service down
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // a thread per connection
        pool.execute(|| {
            process_stream(stream);
        })
    }

    println!("Shutting down");
}
