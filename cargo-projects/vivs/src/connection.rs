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

        let data_chunk_parsed = DataChunk::new(&mut cursored_buffer);

        //     // iterate over the buffer and identify a end of a line
        //     // then return the line
        //     for position in current_position..end_position {
        //         // we get the reference to the underlying value in the cursor
        //         // if we find that at some point there's \r followed by \n then we have a line
        //         // we then update position in the buffer to the start of the next line
        //         // *1\r\n\x244\r\nPING\r\n\r\n - [*1\r\n] line 1, [x244\r\n] line 2, [PING\r\n\r\n] line 3
        //         if cursored_buffer.get_ref()[position] == b'\r' && cursored_buffer.get_ref()[position + 1] == b'\n' {
        //             cursored_buffer.set_position((position + 2) as u64);
        //             // get usize by first converting slice in array
        //             // then converting from native endian int value to usize
        //             command_length = u64::from_be_bytes(
        //                 cursored_buffer.get_ref()[current_position..position]
        //                     .try_into()
        //                     .unwrap(),
        //             );
        //             // return Ok(command_length);
        //             break;
        //             // cursored_buffer.get_ref()[current_position..position];
        //         }
        //     }

        //     Ok(command_length)
        //     }
        //     _ => unimplemented!(),
        // }

        // Ok(data_chunk)
        Ok(data_chunk_parsed.unwrap())
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
