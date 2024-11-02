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

// i.e. \r\n
const END_OF_LINE: [u8; 2] = [13, 10];

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
    pub async fn write_chunk(&mut self, data_type: DataType, data: &[u8]) -> io::Result<()> {
        // e.g. hello - $5\r\nhello\r\n
        let data_type = match data_type {
            DataType::SimpleString => b'+',
            DataType::Null => b'_',
            DataType::SimpleError => b'-',
            DataType::Integer => b':',
            DataType::BulkString => b'$',
        };

        // Response is different for these types
        // i.e. +OK\r\n (Simple String) OR :19 (Integer)
        if data_type == b'+' || data_type == b':' {
            self.stream.write_u8(data_type).await?;
            self.stream.write_all(data).await?;
            self.stream.write_all(&END_OF_LINE).await?;
            self.stream.flush().await
        } else {
            // Since data type is u8 and from_utf8 requires a byte slice, we create one
            let data_type_slice = &[data_type];
            let length = data.len();
            let length = length.try_into().unwrap(); // TODO: not very safe

            let bytes_to_write = [
                &data_type_slice[..1],
                &[length],
                &END_OF_LINE,
                &data[0..],
                &END_OF_LINE,
            ]
            .concat();

            self.stream.write_all(&bytes_to_write[..]).await?;
            self.stream.flush().await
        }
    }

    //
    pub async fn write_error(&mut self, err_msg_type_bytes: &[u8]) -> io::Result<()> {
        // "-" - first byte denotes error data type
        // "ERR" - generic error type
        // TODO: as a future improvement we could differentiate between error types
        self.stream.write_all(b"-").await?;
        self.stream.write_all(err_msg_type_bytes).await?;
        self.stream.write_all(&END_OF_LINE).await?;
        self.stream.flush().await
    }

    pub async fn write_error_with_msg(
        &mut self,
        err_msg_type_bytes: &[u8],
        err_msg: &[u8],
    ) -> io::Result<()> {
        // "-" - first byte denotes error data type
        // "ERR" - generic error type
        // TODO: as a future improvement we could differentiate between error types
        self.stream.write_all(b"-").await?;
        self.stream.write_all(err_msg_type_bytes).await?;
        self.stream.write_all(b" ").await?;
        self.stream.write_all(err_msg).await?;
        self.stream.write_all(&END_OF_LINE).await?;
        self.stream.flush().await
    }

    pub async fn write_null(&mut self) -> io::Result<()> {
        // "_" - first byte denotes null which represents non-existent values
        self.stream.write_u8(b'_').await?;
        self.stream.write_all(&END_OF_LINE).await?;
        self.stream.flush().await
    }

    pub async fn write_chunk_frame(&mut self, data: &mut Parser) -> io::Result<()> {
        self.stream.write_all(b"*").await?; // *
        self.stream
            .write_all(data.size().to_string().as_bytes())
            .await?; // 1
        self.stream.write_all(&END_OF_LINE).await?;

        for chunk in data.into_iter() {
            match chunk {
                DataChunk::Bulk(str) => {
                    // string length minus "\r\n"
                    let len_byte = (str.len() - 2).to_string();
                    self.stream.write_all(b"$").await?;
                    self.stream.write_all(len_byte.as_bytes()).await?;
                    self.stream.write_all(&END_OF_LINE).await?;
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
