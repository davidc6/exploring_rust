use crate::{
    commands::{ping::PONG, DataType},
    data_chunk::DataChunk,
    parser::Parser,
    GenericResult,
};
use bytes::{Bytes, BytesMut};
use std::{
    fmt::Display,
    io::{self, Cursor},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

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

    /// Reads and processes a stream of bytes from the TCP stream.
    pub async fn process_stream(&mut self) -> GenericResult<Parser> {
        // Buffer needs to be cleared since the same Connection instance runs for a single tcp connection
        // and unless cleared, it will be just appending to the buffer
        self.buffer.clear();

        // Pull bytes from the source/tcp stream into the buffer
        let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

        if bytes_read != 0 {
            // Cursor enables to track location in the buffer by providing seek functionality.
            // It wraps the underlying buffer (in our case BytesMut).
            // In this case self.buffer refers to the slice (full range) of the buffer (BytesMut).
            let mut cursored_buffer = Cursor::new(&self.buffer[..]);

            let data_chunk = DataChunk::read_chunk(&mut cursored_buffer)?;
            return Parser::new(data_chunk);
        }

        // 0 read bytes usually indicates end of stream/connection closed status and could be because:
        // 1. the reader reached end of file and most likely won't produce more bytes
        // 2. buffer has remaining capacity of zero
        if self.buffer.is_empty() {
            Ok(Parser::default())
        } else {
            Err(Box::new(ConnectionError::TcpClosed))
        }
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
        self.stream.write_all(b"-(error) ").await?;
        self.stream.write_all(err_msg_bytes).await?;
        self.stream.flush().await
    }

    pub async fn write_null(&mut self) -> io::Result<()> {
        // "_" - first byte denotes null which represents non-existent values
        self.stream.write_all(b"_\r\n").await?;
        self.stream.flush().await
    }

    pub async fn write_chunk_frame(&mut self, data: Parser) -> io::Result<()> {
        self.stream.write_all(b"*").await?; // *
        self.stream
            .write_all(data.size().to_string().as_bytes())
            .await?; // 1
        self.stream.write_all(b"\r\n").await?; // \r\n

        for chunk in data.iter() {
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

    /// TODO: need to rethink this since clients should potentially handle this
    /// The last _ (fall through / catch-all case)
    pub async fn read_chunk_frame(&mut self) -> GenericResult<Bytes> {
        // read response
        let mut data_chunk = self.process_stream().await?;

        match data_chunk.next() {
            Some(DataChunk::Bulk(data_bytes)) => {
                // This is a hack in order to write consistently formatted values to stdout.
                // Since val without quotes can also be written back to stdout without quotes
                // it is not desirable and therefore we want to add extra quotes to the output value.
                // We need to think about allocations here as it will affect performance in the long run.
                // 34 is "
                if data_bytes.first() != Some(&34) && data_bytes != *PONG {
                    let quotes_bytes = Bytes::from("\"");
                    let concat_bytes = [quotes_bytes.clone(), data_bytes, quotes_bytes].concat();
                    Ok(Bytes::from(concat_bytes))
                } else {
                    Ok(data_bytes)
                }
            }
            Some(DataChunk::Null) => Ok(Bytes::from("(nil)")),
            Some(DataChunk::SimpleError(data_bytes)) => Ok(data_bytes),
            Some(DataChunk::Integer(val)) => {
                // convert Bytes to bytes array
                // then determine endianness to create u64 integer value from the bytes array
                // and return integer as string
                let bytes_slice = val.slice(0..8);

                // converts the slice to an array of u8 elements (since u64 is 8 bytes)
                let arr_u8: [u8; 8] = bytes_slice[0..8].try_into().unwrap();
                let integer_as_string = if cfg!(target_endian = "big") {
                    u64::from_be_bytes(arr_u8)
                } else {
                    u64::from_le_bytes(arr_u8)
                }
                .to_string();

                Ok(Bytes::from(format!("(integer) {}", integer_as_string)))
            }
            None => Ok(Bytes::from("Unknown")),
            _ => Ok(Bytes::from("(nil)")), // catch all case
        }
    }
}
