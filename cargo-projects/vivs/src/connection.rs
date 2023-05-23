use bytes::{buf, Buf, Bytes, BytesMut};
use std::io::{self, Cursor};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    data_chunk::{self, DataChunk, DataChunkFrame},
    Error, Result,
};

// Gets number of either elements in array or string length
fn number_of(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<u64, Error> {
    use atoi::atoi;

    let current_position = cursored_buffer.position() as usize;
    let length = cursored_buffer.get_ref().len();
    let buffer_slice = &cursored_buffer.get_ref()[current_position..length];

    atoi::<u64>(buffer_slice).ok_or_else(|| "could not parse integer, invalid format".into())
}

pub struct Connection {
    // writer: BufWriter<TcpStream>
    // writer: BufWriter<WriteHalf<'a>>,
    // reader: BufReader<ReadHalf>,
    // reader: BufReader<ReadHalf<'a>>,
    stream: TcpStream,
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
            // writer: BufWriter::new(write),
            stream,
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
    pub async fn write_chunk(mut self, data: &[u8]) -> io::Result<()> {
        self.stream.write_u8(b'+').await?;
        self.stream.write_all(data).await?;
        self.stream.write_all(b"\r\n").await?;
        Ok(())
    }
}
