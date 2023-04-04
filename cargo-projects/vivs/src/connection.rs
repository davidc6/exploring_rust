use bytes::{buf, Buf, Bytes, BytesMut};
use std::io::{self, Cursor};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{Error, Result};

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

    // TODO: frame reading will happen here
    pub async fn read_and_process_stream(&mut self) -> Result<u64> {
        // Pull bytes from the source (self.stream - TcpStream) into the provided buffer (self.buffer)
        self.stream.read_buf(&mut self.buffer).await?;

        // Cursor enables to track location in the buffer by providing seek functionality
        // It wraps the underlying buffer (in our case BytesMut)
        // In this case self.buffer refers to the slice (full range) of the buffer (BytesMut)
        let mut cursored_buffer = Cursor::new(&self.buffer[..]);

        // cursored_buffer.get_u8();
        // println!("{:?}", &self.buffer[..]);

        // we need to scan the cursored_buffer and find end of line
        // get end of buffer / hard-coded for now
        // last index in the array
        // let end = (cursored_buffer.get_ref().len() - 1) as u64;

        // // end of line
        // cursored_buffer.set_position(end + 2);

        // println!("{:?}", cursored_buffer.remaining());

        // if !cursored_buffer.has_remaining() {
        //     return Err("error".into());
        // }

        // Since buffer is [u8], we use get_u8() to get the first byte from it
        // this also advances the position by one
        // The first byte determines the data type
        // e.g. * is Array
        match cursored_buffer.get_u8() {
            // Array data type
            b'*' => {
                // Get number of elements in the array
                let number = number_of(&mut cursored_buffer);

                println!("Number of elements in array {:?}", number);

                //     let current_position = cursored_buffer.position() as usize; // will be first position
                //     let end_position = &cursored_buffer.get_ref().len() - 1; // second to last byte

                //     let mut command_length: u64 = 0;

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
            }
            _ => unimplemented!(),
        }

        Ok(1)
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
