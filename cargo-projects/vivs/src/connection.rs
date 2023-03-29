use bytes::{buf, Buf, Bytes, BytesMut};
use std::io::{self, Cursor};
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
            // BytesMut is a unique reference into a continuguos slice of memory
            // 1kb, for now but mostly will need to increase in the future
            buffer: BytesMut::with_capacity(1024),
        }
    }

    // TODO: frame reading will happen here
    pub async fn read_chunk(&self) {
        // Enables to track location in the buffer by using Cursor which provides seek functionality
        // by wrapping an underlying buffer (in our case BytesMut)
        // self.buffer is referring to the slice (full range) of the buffer (BytesMut)
        let mut buff = Cursor::new(&self.buffer[..]);

        match buff.get_u8() {
            b'*' => {
                let current_position = buff.position() as usize; // will be first position
                let end_position = buff.get_ref().len() - 1; // second to last byte

                for position in current_position..end_position {
                    // we get the reference to the underlying value in the cursor
                    // if we find that at some point there's \r followed by \n then we have a line
                    // we then update position in the buffer to the start of the next line
                    // *1\r\n\x244\r\nPING\r\n\r\n - [*1\r\n] line 1, [x244\r\n] line 2, [PING\r\n\r\n] line 3
                    if buff.get_ref()[position] == b'\r' && buff.get_ref()[position + 1] == b'\n' {
                        buff.set_position((position + 2) as u64);
                    }
                }
            }
            _ => unimplemented!(),
        }
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
