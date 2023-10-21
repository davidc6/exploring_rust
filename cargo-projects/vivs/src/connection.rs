use crate::{
    commands::DataType,
    data_chunk::{DataChunk, DataChunkFrame},
    Result,
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
    // writer: BufWriter<WriteHalf<'a>>,
    // reader: BufReader<ReadHalf>,
    // reader: BufReader<ReadHalf<'a>>,
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            // writer: BufWriter::new(write),
            // reader: BufReader::new(read),
            // writer: BufWriter::new(write),
            stream: BufWriter::new(stream),
            // BytesMut is a unique reference into a contiguous slice of memory
            // which acts as a buffer for a tcp stream read functionality
            // 1kb, for now but mostly will need to increase in the future
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn read_and_process_stream(&mut self) -> Result<DataChunkFrame> {
        // Buffer needs to be cleared since the same Connection instance runs for a single tcp connection
        // and unless cleared will be adding to the buffer
        self.buffer.clear();
        // Pull bytes from the source (self.stream: TcpStream) into the provided buffer (self.buffer)
        let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

        // 0 read bytes usually indicates that the connection was closed
        if bytes_read != 0 {
            // Cursor enables to track location in the buffer by providing seek functionality
            // It wraps the underlying buffer (in our case BytesMut)
            // In this case self.buffer refers to the slice (full range) of the buffer (BytesMut)
            let mut cursored_buffer = Cursor::new(&self.buffer[..]);

            // Data frame parsed and structured
            return DataChunk::new(&mut cursored_buffer);
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

    pub async fn write_error(mut self, err_msg_bytes: &[u8]) -> io::Result<()> {
        // "-" - first byte denotes error data type
        // "ERR" - generic error type
        // TODO: as a future improvement we could differentiate between error types
        self.stream.write_all(b"-ERR").await?;
        self.stream.write_all(err_msg_bytes).await?;
        self.stream.flush().await
    }

    pub async fn write_null(&mut self) -> io::Result<()> {
        // "_" - first byte denotes null which represents non-existent values
        self.stream.write_all(b"_\r\n").await?;
        self.stream.flush().await
    }

    pub async fn write_chunk_frame(&mut self, data: DataChunkFrame) -> io::Result<()> {
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

    pub async fn read_chunk_frame(&mut self) -> Result<Bytes> {
        // read response
        let mut data_chunk = self.read_and_process_stream().await?;
        let data_chunk = data_chunk.next();

        match data_chunk.unwrap() {
            DataChunk::Bulk(data) => Ok(data),
            _ => Ok(Bytes::new()),
        }
    }
}
