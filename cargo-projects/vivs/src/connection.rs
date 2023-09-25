use crate::{
    commands::DataType,
    data_chunk::{DataChunk, DataChunkFrame},
    Result,
};
use bytes::BytesMut;
use std::io::{self, Cursor};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

pub struct Connection {
    // writer: BufWriter<WriteHalf<'a>>,
    // reader: BufReader<ReadHalf>,
    // reader: BufReader<ReadHalf<'a>>,
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        // pub fn new(read: ReadHalf<'a>, write: WriteHalf<'a>) -> Connection<'a> {
        // by passing read and write, we need to di
        // let mut s = Box::new(socket);
        // let (read, write) = stream.split();

        Connection {
            // writer: BufWriter::new(write),
            // reader: BufReader::new(read),
            // writer: BufWriter::new(write),
            stream: BufWriter::new(stream),
            // BytesMut is a unique reference into a continuguos slice of memory
            // 1kb, for now but mostly will need to increase in the future
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn read_and_process_stream(&mut self) -> Result<DataChunkFrame> {
        // Pull bytes from the source (self.stream: TcpStream) into the provided buffer (self.buffer)
        self.stream.read_buf(&mut self.buffer).await?;

        // Cursor enables to track location in the buffer by providing seek functionality
        // It wraps the underlying buffer (in our case BytesMut)
        // In this case self.buffer refers to the slice (full range) of the buffer (BytesMut)
        let mut cursored_buffer = Cursor::new(&self.buffer[..]);

        // Data frame parsed and structured
        DataChunk::new(&mut cursored_buffer)
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
}
