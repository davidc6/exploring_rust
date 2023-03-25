use bytes::{Bytes, BytesMut};
use std::io::{self};
use tokio::{io::AsyncWriteExt, net::TcpStream};

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
            // writer: BufWriter::new(write)
            // writer: BufWriter::new(write),
            stream,
            buffer: BytesMut::with_capacity(1024 * 4),
        }
    }

    // TODO: frame reading will happen here
    pub async fn read_chunk() {
        todo!()
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
