use crate::{commands::DataType, data_chunk::DataChunk, parser::Parser, GenericResult};
use bytes::BytesMut;
use std::{
    fmt::Display,
    io::{self, Cursor},
    net::SocketAddr,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

const END_OF_LINE: &str = "\r\n";

#[derive(Debug)]
pub enum ConnectionError {
    TcpClosed,
}

impl std::error::Error for ConnectionError {}

impl Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConnectionError::TcpClosed => {
                write!(f, "TCP connection closed")
            }
        }
    }
}

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

/// Buffer allocation and frame (network data) parsing occurs here
impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(stream),
            // BytesMut is a unique reference into a contiguous slice of memory
            // which acts as a buffer for a tcp stream read functionality
            // 1kb, for now but mostly will need to increase in the future
            buffer: BytesMut::with_capacity(1024),
        }
    }

    /// Returns a remotely connected peer address. An empty string if no peer_addr is returned
    /// since this method is only used for logging purposes at the moment.
    pub fn connected_peer_addr(&self) -> String {
        let address = self.stream.get_ref().peer_addr();
        if let Ok(addr) = address {
            return addr.to_string();
        }

        "".to_owned()
    }

    pub fn own_addr(&self) -> io::Result<SocketAddr> {
        self.stream.get_ref().local_addr()
    }

    /// Reads and processes a stream of bytes from the TCP stream.
    pub async fn process_stream(&mut self) -> GenericResult<Cursor<&[u8]>> {
        // Buffer needs to be cleared since the same Connection instance runs for a single tcp connection
        // and unless cleared, it will be just appending to the buffer
        self.buffer.clear();

        // Pull bytes from the source/tcp stream into the buffer
        let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

        if bytes_read != 0 || self.buffer.is_empty() {
            // Cursor enables to track location in the buffer by providing seek functionality.
            // It wraps the underlying buffer (in our case BytesMut).
            // In this case self.buffer refers to the slice (full range) of the buffer (BytesMut).
            let cursored_buffer = Cursor::new(&self.buffer[..]);

            return Ok(cursored_buffer);
        }

        Err(Box::new(ConnectionError::TcpClosed))
    }

    // Write chunk of data / frame to the stream
    // Frame is defined as bits of data in this context
    // Since data is buffered in BufWriter no excessive sys calls to write will occur here
    pub async fn write_chunk(
        &mut self,
        data_type: DataType,
        data: Option<&[u8]>,
    ) -> io::Result<()> {
        let data_type = match data_type {
            DataType::SimpleString => b'+',
            DataType::Null => b'_',
            DataType::SimpleError => b'-',
            DataType::Integer => b':',
        };

        self.stream.write_u8(data_type).await?;

        // Write data to the socket
        if data.is_some() {
            self.stream.write_all(data.unwrap()).await?;
        }

        self.stream.write_all(b"\r\n").await?;
        self.stream.flush().await
    }

    pub async fn write_error(&mut self, err_msg_bytes: &[u8]) -> io::Result<()> {
        // "-" - first byte denotes error data type
        // "ERR" - generic error type
        // TODO: as a future improvement we could differentiate between error types
        self.stream.write_all(b"-ERR ").await?;
        self.stream.write_all(err_msg_bytes).await?;
        self.stream.write_all(END_OF_LINE.as_bytes()).await?;
        self.stream.flush().await
    }

    pub async fn write_null(&mut self) -> io::Result<()> {
        // "_" - first byte denotes null which represents non-existent values
        self.stream.write_all(b"_\r\n").await?;
        self.stream.flush().await
    }

    pub async fn write_chunk_frame(&mut self, data: &mut Parser) -> io::Result<()> {
        self.stream.write_all(b"*").await?; // *
        self.stream
            .write_all(data.size().to_string().as_bytes())
            .await?; // 1
        self.stream.write_all(b"\r\n").await?; // \r\n

        for chunk in data.into_iter() {
            match chunk {
                DataChunk::Bulk(str) => {
                    // string length minus "\r\n"
                    let len_byte = (str.len() - 2).to_string();
                    self.stream.write_all(b"$").await?;
                    self.stream.write_all(len_byte.as_bytes()).await?;
                    self.stream.write_all(b"\r\n").await?;
                    self.stream.write_all(&str).await?;
                }
                _ => {
                    // TODO: Handle other data types
                    todo!();
                }
            }
        }

        self.stream.flush().await
    }

    pub async fn write_complete_frame(&mut self, data: &str) -> io::Result<()> {
        self.stream.write_all(data.as_bytes()).await?;
        self.stream.flush().await
    }
}
