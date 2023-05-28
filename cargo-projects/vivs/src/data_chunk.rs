use bytes::{Buf, Bytes};
use std::{fmt, io::Cursor, num::TryFromIntError, vec::IntoIter};

use crate::Result as CustomResult;

#[derive(Debug)]
pub enum Error {
    Insufficient,
    Uknown(String),
    ParseError,
}

// Gets number of either elements in array or string char count
// TODO - need to stop parsing at the end of the line
fn number_of(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<u64, Error> {
    use atoi::atoi;

    // current cursor position
    // let current_position = cursored_buffer.position() as usize;
    // number of overall elements in the underlying value
    // let length = cursored_buffer.get_ref().len();
    // underlying slice
    // let buffer_slice = &cursored_buffer.get_ref()[current_position..length];

    // cursored_buffer.set_position(length as u64 - current_position as u64);

    let slice = line(cursored_buffer)?;

    atoi::<u64>(slice).ok_or_else(|| "could not parse integer from a slice".into())
}

/// Tries to find EOL (\r\n - carriage return(CR) and line feed (LF)),
/// return everything before EOL and advance (by incrementing by 2) Cursor to the next position
/// which is after EOL. The return value is a slice of bytes if parsed
/// correctly or Err otherwise.
fn line<'a>(cursored_buffer: &'a mut Cursor<&[u8]>) -> Result<&'a [u8], Error> {
    // get current position and total length
    let current_position = cursored_buffer.position() as usize;
    let length = cursored_buffer.get_ref().len();

    for position in current_position..length + 1 {
        if cursored_buffer.get_ref()[position] == b'\r'
            && cursored_buffer.get_ref()[position + 1] == b'\n'
        {
            cursored_buffer.set_position((position + 2) as u64);
            return Ok(&cursored_buffer.get_ref()[current_position..position]);
        }
    }

    Err(Error::Insufficient)
}

pub struct DataChunkFrame {
    // data_chunks: DataChunk,
    segments: IntoIter<DataChunk>,
}

impl DataChunkFrame {
    pub fn next(&mut self) -> Result<DataChunk, Error> {
        self.segments.next().ok_or(Error::ParseError)
    }
}

#[derive(Debug)]
pub enum DataChunk {
    Array(Vec<DataChunk>),
    Bulk(Bytes),
}

impl DataChunk {
    pub fn new(cursored_buffer: &mut Cursor<&[u8]>) -> CustomResult<DataChunkFrame> {
        // 1. parse
        // 2. create DataChunk that has iterator also else error
        let commands = DataChunk::parse(cursored_buffer);

        let array = match commands {
            Ok(DataChunk::Array(val)) => val,
            _ => return Err("some error".into()),
        };

        Ok(DataChunkFrame {
            // data_chunks: commands.unwrap(),
            segments: array.into_iter(),
        })
    }

    pub fn parse(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<DataChunk, Error> {
        match cursored_buffer.get_u8() {
            // e.g. *1
            b'*' => {
                // Using range expression ( .. ) which implements Iterator trait enables to map over each element
                // then collect iterator into a vector
                let number = number_of(cursored_buffer)?;
                let commands = (0..number)
                    .map(|_| {
                        DataChunk::parse(cursored_buffer)
                            .unwrap_or_else(|_| panic!("Could not parse"))
                    })
                    .collect::<Vec<DataChunk>>();

                Ok(DataChunk::Array(commands))
            }
            // e.g. $4
            b'$' => {
                // Not parsing the line here, just getting length of string and returning a copy

                // get length of the bulk string + 2 (i.e. \n\r)
                let str_len = number_of(cursored_buffer)?.try_into()?;

                // TODO: do we need to handle the case where we haven't received CR and LF

                let bulk_str_data = Bytes::copy_from_slice(&cursored_buffer.chunk()[..str_len]);

                // advance the interval position (+2 \r and \n) as we've now gotten the needed bulk string
                cursored_buffer.advance(str_len + 2);

                Ok(DataChunk::Bulk(bulk_str_data))
            }
            _ => {
                println!("U8: {:?}", cursored_buffer.get_u8());
                println!("LINE: {:?}", line(cursored_buffer));
                unimplemented!();
            }
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Uknown(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Error {
        value.to_string().into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "invalid data chunk".into()
    }
}

// impl From<std::error::Error> for Error {
//     fn from(_src: Error) -> Error {
//         "Invalid".into()
//     }
// }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError => "protocol error; unexpected end of stream".fmt(f),
            Error::Uknown(err) => err.fmt(f),
            Error::Insufficient => "error".fmt(f),
        }
    }
}

impl std::error::Error for Error {}
