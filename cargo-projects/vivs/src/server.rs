use std::error::Error;

use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncWriteExt, BufWriter}};
use tokio::io::AsyncBufReadExt;
// use std::io::{Result};

async fn handle_stream(mut stream: TcpStream, addr: std::net::SocketAddr) {
    let (read, write) = stream.split();

    let mut read_buffer = BufReader::new(read);
    // let mut w = BufWriter::new(write);

    // let stream_clone = stream.try_clone().unwrap();
    // let mut buffer = BufWriter::new(r);

    
    loop {
        let mut buffer = String::new();

        tokio::select! {
            // read line until until newline and put it into the buffer
            read_result = read_buffer.read_line(&mut buffer) => {
                match read_result {
                    Ok(val) => {
                        // b
                        // println!("{:?} {}", buffer, val);
                        // break;

                        // let data_type = buffer.get(0..1); // gets data type
                        // let array_len = buffer.get(1..2); // gets data type

                        // if data_type == Some("*") {

                        // }





                        println!("{} {:?}", val, buffer);


                        match write.try_write(b"+PONG\r\n") {
                            Ok(val) => {
                                // println!("GOOD {}", val);
                                // if val == 
                                // break;
                            }
                            Err(e) => {
                                println!("ERROR");
                                // return Err(e.into());
                                // Err("Err");
                            }
                        }



                        // buffer.write(b"PONG");
                        // buffer.s

                    },
                    Err(e) => {
                        println!("{}", e);
                        // return Err(e.into());

                    }
                }
            },
            else => {
                println!("AHA");
                break;
            }
            // println("{:?}", read_result);
        }
        buffer.clear();
        println!("HERE");

        // println!("A {}", b.len());
        // stream.write_all(b"some bytes");
        
        // Ok(())
    }
}

pub async fn start(addr: String, port: String) -> std::io::Result<()> {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await?;

    loop {
        let (mut stream, addr) = listener.accept().await?; // TODO - checks?

        tokio::spawn(async move {
            handle_stream(stream, addr).await;
        });
    }
}
