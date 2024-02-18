use crate::GenericResult;
use atoi::atoi;
use bytes::{Buf, Bytes};
use std::{
    fmt::{self},
    io::Cursor,
    num::TryFromIntError,
    str::Utf8Error,
    vec::IntoIter,
};

#[derive(Debug)]
pub enum DataChunkError {
    Insufficient,
    Unknown(String),
    Parse(String),
    NonExistent,
    Other(Utf8Error),
}

impl std::error::Error for DataChunkError {}

impl From<Utf8Error> for DataChunkError {
    fn from(e: Utf8Error) -> Self {
        DataChunkError::Other(e)
    }
}

impl From<String> for DataChunkError {
    fn from(value: String) -> Self {
        DataChunkError::Unknown(value)
    }
}

impl From<&str> for DataChunkError {
    fn from(value: &str) -> DataChunkError {
        value.to_string().into()
    }
}

impl From<TryFromIntError> for DataChunkError {
    fn from(_src: TryFromIntError) -> DataChunkError {
        "invalid data chunk".into()
    }
}

impl fmt::Display for DataChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataChunkError::Parse(e) => format!("protocol error: {:?}", e).fmt(f),
            DataChunkError::Unknown(err) => err.fmt(f),
            DataChunkError::Insufficient => "error".fmt(f),
            DataChunkError::NonExistent => "no next value in the iterator".fmt(f),
            DataChunkError::Other(val) => val.fmt(f),
        }
    }
}

// Gets number of either elements in array or string char count
// TODO - need to stop parsing at the end of the line
fn number_of(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<u64, DataChunkError> {
    let slice = line(cursored_buffer)?;

    atoi::<u64>(slice).ok_or(DataChunkError::Parse(
        "Failed to parse an integer from a slice".to_owned(),
    ))
}

/// Tries to find EOL (\r\n - carriage return(CR) and line feed (LF)).
/// Returns a slice before EOL and advances (increments by 2) the Cursor to the next position
/// which is after EOL. The return value is a slice of bytes if parsed
/// correctly or Err otherwise.
fn line<'a>(cursored_buffer: &'a mut Cursor<&[u8]>) -> Result<&'a [u8], DataChunkError> {
    // get current position and total length
    let current_position = cursored_buffer.position() as usize;
    let length = cursored_buffer.get_ref().len();

    for position in current_position..length + 1 {
        // checks current and next bytes
        if cursored_buffer.get_ref()[position] == b'\r'
            && cursored_buffer.get_ref()[position + 1] == b'\n'
        {
            cursored_buffer.set_position((position + 2) as u64);
            return Ok(&cursored_buffer.get_ref()[current_position..position]);
        }
    }

    Err(DataChunkError::Insufficient)
}

#[derive(Debug, Default)]
pub struct DataChunkFrame {
    /// Iterator of DataChunk type
    segments: IntoIter<DataChunk>,
}

// The iterator should contain all the necessary commands and values e.g. [SET, key, value]
impl DataChunkFrame {
    #[allow(clippy::should_implement_trait)]
    /// Tries to return the next element in the collection.
    /// Returns an error otherwise
    pub fn next(&mut self) -> Option<DataChunk> {
        self.segments.next()
    }

    /// Tries to return next element in the collection/segments.
    /// If the element exists then a String type gets returned.
    /// Other an Error is returned.
    /// The reason the error is returned is because we attempt to convert a
    /// slice of bytes to string slice in the match expression.
    pub fn next_as_str(&mut self) -> Result<Option<String>, DataChunkError> {
        let Some(segment) = self.segments.next() else {
            return Ok(None);
        };

        match segment {
            DataChunk::Bulk(value) => {
                let value = std::str::from_utf8(value.chunk())?;
                Ok(Some(value.to_owned()))
            }
            _ => unimplemented!(),
        }
    }

    pub fn enumerate(self) -> std::iter::Enumerate<IntoIter<DataChunk>> {
        self.segments.enumerate()
    }

    pub fn iter(self) -> IntoIter<DataChunk> {
        self.segments
    }

    pub fn size(&self) -> usize {
        self.segments.len()
    }

    pub fn push_bulk_str(mut self, b: Bytes) -> Self {
        // TODO - this is a hack (for now).
        // Convert iterator to vector in order to push data chunks into it.
        // This functionality is part of the so called "client encoder".
        let mut v: Vec<DataChunk> = self.segments.collect();
        v.push(DataChunk::Bulk(b));
        self.segments = v.into_iter();
        self
    }
}

#[derive(Debug, Default)]
pub enum DataChunk {
    Array(Vec<DataChunk>),
    Bulk(Bytes),
    #[default]
    Null,
    Integer(Bytes),
    SimpleError(Bytes),
}

impl DataChunk {
    #![allow(clippy::new_ret_no_self)]
    /// Constructs DataChunkFrame by parsing the incoming buffer
    pub fn new(cursored_buffer: &mut Cursor<&[u8]>) -> GenericResult<DataChunkFrame> {
        let commands = DataChunk::parse(cursored_buffer);

        let data_chunks_vec = match commands {
            Ok(DataChunk::Array(val)) => val,
            Ok(DataChunk::Bulk(value)) => vec![DataChunk::Bulk(value)],
            Ok(DataChunk::Null) => vec![DataChunk::Null],
            Ok(DataChunk::Integer(value)) => vec![DataChunk::Integer(value)],
            Ok(DataChunk::SimpleError(value)) => vec![DataChunk::SimpleError(value)],
            Err(e) => return Err(e.into()),
        };

        let segments = data_chunks_vec.into_iter();

        Ok(DataChunkFrame { segments })
    }

    /// Splits a string slice by whitespace,
    /// and then builds a String of commands and values.
    ///
    /// This associated function is used by the REPL implementation,
    /// to convert commands to a parsable String which then
    /// gets written as bytes to the tcp stream.
    pub fn from_string(value: &str) -> GenericResult<String> {
        let split = value.trim_end().split(' ');

        let parsed_commands = split.fold(("\r\n".to_owned(), 0), |acc, val| {
            (format!("{}${}\r\n{}\r\n", acc.0, val.len(), val), acc.1 + 1)
        });

        let (commands, commands_count) = parsed_commands;

        Ok(format!("*{commands_count}{commands}"))
    }

    /// Parses the data type (first byte sign like +, :, $ etc)
    /// then get the value that comes after it.
    ///
    /// This can be the number of elements in the array (in the case of *)
    /// or the string size (in the case of $).
    ///
    /// After initially parsing '*', the function calls itself
    /// to parse the rest of the stream eventually returning an array of elements.
    /// For instance command SET "greeting" "hi" which looks like
    /// (*3/r/n$3/r/nSET/r/n$8/r/ngreeting/r/n$2/r/nhi/r/n), will translate to something like
    /// [SET, "greeting", "hi"].
    pub fn parse(
        cursored_buffer: &mut Cursor<&[u8]>,
    ) -> std::result::Result<DataChunk, DataChunkError> {
        // TODO - add cursored_buffer.has_remaining() check
        let n = cursored_buffer.get_u8();

        match n {
            // e.g. *1
            b'*' => {
                // Using range expression ( .. ) which implements Iterator trait,
                // enables to map over each element then collect iterator into a vector.
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
                // Not parsing, just getting length of string.
                let str_len = number_of(cursored_buffer)?.try_into()?;

                // TODO: handle the case where we haven't received CR and LF

                // Compare if indicated ($[4]) and actual lengths are the same,
                // since string length cannot be more that the length of the buffer itself.
                if str_len > cursored_buffer.chunk().len() {
                    return Err(DataChunkError::Insufficient);
                }

                // cursored_buffer.chunk().len() is the length of the whole buffer
                let bulk_str_data = Bytes::copy_from_slice(&cursored_buffer.chunk()[..str_len]);
                // advance the interval position (+2 because of \r and \n) as we've now gotten the needed bulk string
                cursored_buffer.advance(str_len + 2);

                Ok(DataChunk::Bulk(bulk_str_data))
            }
            // e.g. +PING
            b'+' => {
                // up to \r\n
                let str_line = line(cursored_buffer);
                let len = str_line.as_ref().unwrap().len();
                let bulk_str_data = Bytes::copy_from_slice(&str_line.unwrap().chunk()[..len]);
                Ok(DataChunk::Bulk(bulk_str_data))
            }
            // e.g. :1
            b':' => {
                let n = line(cursored_buffer);
                let v = Bytes::copy_from_slice(n.unwrap());
                Ok(DataChunk::Integer(v))
            }
            // null value
            b'_' => Ok(DataChunk::Null),
            // error value
            b'-' => {
                let err = line(cursored_buffer);
                let copied_err = Bytes::copy_from_slice(err.unwrap());
                Ok(DataChunk::SimpleError(copied_err))
            }
            // everything else, catch-all case
            // potentially, when we are trying to parse something that does not exist
            _ => Err(DataChunkError::Parse(format!(
                "Failed to parse unknown data type {:?}",
                n
            ))),
        }
    }
}
