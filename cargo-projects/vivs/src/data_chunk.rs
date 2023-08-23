use crate::Result as CustomResult;
use bytes::{Buf, Bytes};
use std::{fmt, io::Cursor, num::TryFromIntError, vec::IntoIter};

#[derive(Debug)]
pub enum Error {
    Insufficient,
    Uknown(String),
    ParseError,
}

impl std::error::Error for Error {}

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
    pub len: usize,
}

impl DataChunkFrame {
    #![allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<DataChunk, Error> {
        self.segments.next().ok_or(Error::ParseError)
    }

    /// Tries to return next element in the collection
    /// as a String type or Error
    pub fn next_as_str(&mut self) -> Result<String, Error> {
        let Some(segment) = self.segments.next() else {
            return Err(Error::ParseError);
        };

        match segment {
            DataChunk::Bulk(value) => {
                let s = std::str::from_utf8(value.chunk());

                if let Ok(str) = s {
                    Ok(str.to_owned())
                } else {
                    // TODO: fix error handling
                    Ok(String::from("aha"))
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn enumerate(self) -> std::iter::Enumerate<IntoIter<DataChunk>> {
        self.segments.enumerate()
    }
}

#[derive(Debug)]
pub enum DataChunk {
    Array(Vec<DataChunk>),
    Bulk(Bytes),
}

impl DataChunk {
    #![allow(clippy::new_ret_no_self)]
    pub fn new(cursored_buffer: &mut Cursor<&[u8]>) -> CustomResult<DataChunkFrame> {
        // parse commands from byte slice
        let commands = DataChunk::parse(cursored_buffer);

        let data_chunks_vec = match commands {
            Ok(DataChunk::Array(val)) => val,
            _ => return Err("some error".into()),
        };

        let segments = data_chunks_vec.into_iter();
        let segments_length = segments.len();

        Ok(DataChunkFrame {
            // data_chunks: commands.unwrap(),
            segments,
            len: segments_length,
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

                // Compare if speficied and actual lengths are the same
                // The specified length of the buffer elements cannot be more that the length of the buffer itself
                if str_len > cursored_buffer.chunk().len() {
                    return Err(Error::Insufficient);
                }

                // cursored_buffer.chunk().len() - the length of the whole buffer
                let bulk_str_data = Bytes::copy_from_slice(&cursored_buffer.chunk()[..str_len]);

                // advance the interval position (+2 \r and \n) as we've now gotten the needed bulk string
                cursored_buffer.advance(str_len + 2);

                Ok(DataChunk::Bulk(bulk_str_data))
            }
            _ => {
                println!("Usigned 8 bit integer: {:?}", cursored_buffer.get_u8());
                println!("Line: {:?}", line(cursored_buffer));
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParseError => "protocol error; unexpected end of stream".fmt(f),
            Error::Uknown(err) => err.fmt(f),
            Error::Insufficient => "error".fmt(f),
        }
    }
}
