use atoi::atoi;
use bytes::{Buf, Bytes};
use std::{fmt, io::Cursor, num::TryFromIntError, str::Utf8Error};

use crate::{commands::ping::PONG, parser::Parser};

#[derive(Debug, PartialEq)]
pub enum DataChunkError {
    Insufficient,
    Parse(String),
    NoBytesRemaining,
    Utf8(Utf8Error),
    Other(String),
}

impl fmt::Display for DataChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataChunkError::Parse(e) => format!("Protocol error: {:?}", e).fmt(f),
            DataChunkError::Insufficient => "Insufficient data to parse".fmt(f),
            DataChunkError::NoBytesRemaining => "Client has disconnected".fmt(f),
            DataChunkError::Utf8(e) => e.fmt(f),
            DataChunkError::Other(e) => e.fmt(f),
        }
    }
}

// Implement Error trait for our custom error type
impl std::error::Error for DataChunkError {}

// To convert utf8 error in next_as_str() method
impl From<Utf8Error> for DataChunkError {
    fn from(e: Utf8Error) -> Self {
        DataChunkError::Utf8(e)
    }
}

// To convert String error message that the "from" conversion produces
impl From<String> for DataChunkError {
    fn from(value: String) -> Self {
        DataChunkError::Other(value)
    }
}

// To convert the output of number_of to usize to compare with buffer chunk length
impl From<TryFromIntError> for DataChunkError {
    fn from(_src: TryFromIntError) -> DataChunkError {
        "Invalid data chunk".to_owned().into()
    }
}

/// Depending on the context, tries to extract either:
///     1. number of elements in the array
///     2. string character count
fn number_of(cursored_buffer: &mut Cursor<&[u8]>) -> Result<u64, DataChunkError> {
    let slice = line(cursored_buffer)?;

    atoi::<u64>(slice).ok_or(DataChunkError::Parse(
        "Failed to parse an integer from the slice".to_owned(),
    ))
}

/// Tries to find EOL i.e. end of line (\r\n - carriage return(CR) and line feed (LF)).
///
/// Returns a slice before EOL and advances (increments by 2) the Cursor to the next position
/// which is after EOL. The return value is a slice of bytes if parsed
/// correctly or Err otherwise.
fn line<'a>(cursored_buffer: &'a mut Cursor<&[u8]>) -> Result<&'a [u8], DataChunkError> {
    // get current position and total length
    let current_position = cursored_buffer.position() as usize;
    // get the number of elements in the underlying slice
    let length = cursored_buffer.get_ref().len();

    if length == 0 {
        return Err(DataChunkError::Insufficient);
    }

    for position in current_position..length {
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

#[derive(Debug, Default, PartialEq)]
pub enum DataChunk {
    Array(Vec<DataChunk>),
    Bulk(Bytes),
    #[default]
    Null,
    Integer(Bytes),
    SimpleError(Bytes),
}

impl DataChunk {
    /// Splits a string slice by whitespace,
    /// and builds a String of commands and values.
    /// Additionally, this method takes into account strings with spaces
    /// that are surrounded by " (double quotes) or ' (single quotes).
    ///
    /// This associated function is used by the REPL implementation,
    /// to convert commands to a parsable String which then
    /// gets converted to bytes and written to the tcp stream.
    ///
    /// # Example
    ///
    /// The following string received from the REPL:
    /// SET some-key some-value
    ///
    /// will get transformed into this string:
    /// *3\r\n$3\r\nSET\r\$8\r\nsome-key\r\n$10\r\nsome-value\r\n
    pub fn from_string(value: &str) -> String {
        let mut elements: Vec<String> = vec![];

        if !value.contains(" ") {
            // trim() here to remove \n at the end of the value
            // e.g. ping\n becomes ping
            let command = value.trim();
            return format!("*1\r\n${}\r\n{command}\r\n", command.len());
        }

        let mut start_position = 0;
        let mut end_position = 0;

        let chars_iter = value.chars();
        let chars_count = chars_iter.clone().count();
        let mut in_range = false;

        for char in chars_iter {
            // value inside of " or ' e.g. "hello world" or 'hello world'
            if (char == '"' || char == '\'') && in_range {
                let value = value[start_position..end_position].to_owned();
                elements.push(value);

                end_position += 1;
                start_position = end_position;

                in_range = false;
                continue;
            }

            // a space or newline but not when seeking inside of " or '
            if (char == ' ' || char == '\n') && !in_range {
                let slice = value[start_position..end_position].to_owned();
                if !slice.is_empty() {
                    elements.push(slice);
                }

                end_position += 1;
                start_position = end_position;
                continue;
            }

            // start of value inside " or '
            if (char == '"' || char == '\'') && !in_range {
                in_range = true;
                start_position = end_position + 1;
                end_position += 1;
                continue;
            }

            if end_position + 1 == chars_count && !elements.is_empty() {
                let slice = value[start_position..=end_position].to_owned();

                elements.push(slice);
                break;
            }

            // any other values outside of " or '
            end_position += 1;
        }

        let (commands, commands_count) =
            elements.iter().fold(("\r\n".to_owned(), 0), |acc, val| {
                (format!("{}${}\r\n{}\r\n", acc.0, val.len(), val), acc.1 + 1)
            });

        format!("*{commands_count}{commands}")
    }

    fn parse_array(cursored_buffer: &mut Cursor<&[u8]>) -> Result<DataChunk, DataChunkError> {
        let number = number_of(cursored_buffer)?;

        // Using range expression ([start position]..[end position]) which implements Iterator trait,
        // enables to map over each element then collect iterator into a vector.
        let commands = (0..number)
            .map(|_| DataChunk::read_chunk(cursored_buffer))
            .collect::<Result<Vec<_>, DataChunkError>>();

        Ok(DataChunk::Array(commands?))
    }

    fn parse_bulk_strings(
        cursored_buffer: &mut Cursor<&[u8]>,
    ) -> Result<DataChunk, DataChunkError> {
        // Not parsing, just getting length of string and converting to usize
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

    fn parse_simple_string(
        cursored_buffer: &mut Cursor<&[u8]>,
    ) -> Result<DataChunk, DataChunkError> {
        // up to \r\n
        let str_line = line(cursored_buffer);
        let default = [];
        let len = str_line.as_ref().unwrap_or(&&default[0..]).len();
        let bulk_str_data = Bytes::copy_from_slice(&str_line.unwrap_or_default().chunk()[..len]);
        Ok(DataChunk::Bulk(bulk_str_data))
    }

    fn parse_integer(cursored_buffer: &mut Cursor<&[u8]>) -> Result<DataChunk, DataChunkError> {
        let n = line(cursored_buffer);
        let integer = Bytes::copy_from_slice(n.unwrap_or_default());
        Ok(DataChunk::Integer(integer))
    }

    fn parse_simple_errors(
        cursored_buffer: &mut Cursor<&[u8]>,
    ) -> Result<DataChunk, DataChunkError> {
        let err = line(cursored_buffer);
        let copied_err = Bytes::copy_from_slice(err.unwrap_or_default());

        Ok(DataChunk::SimpleError(copied_err))
    }

    /// Parses the data type (first byte sign like +, :, $ etc)
    /// then gets the value that comes after it.
    ///
    /// This can be the number of elements in the array (in the case of *)
    /// or the string size (in the case of $).
    ///
    /// After initially parsing '*', the function calls itself
    /// to parse the rest of the stream eventually returning an array of elements.
    /// For instance command SET "greeting" "hi" which looks like
    /// (*3/r/n$3/r/nSET/r/n$8/r/ngreeting/r/n$2/r/nhi/r/n), will translate to something like
    /// [SET, "greeting", "hi"].
    pub fn read_chunk(
        cursored_buffer: &mut Cursor<&[u8]>,
    ) -> std::result::Result<DataChunk, DataChunkError> {
        // Client disconnects
        if !cursored_buffer.has_remaining() {
            return Err(DataChunkError::NoBytesRemaining);
        }

        let first_byte = cursored_buffer.get_u8();

        match first_byte {
            // e.g. *1 (denotes the number of elements in the commands / values array: 1 element)
            b'*' => Self::parse_array(cursored_buffer),
            // e.g. $4 (denotes the length of the next element in the array: 4 bytes)
            b'$' => Self::parse_bulk_strings(cursored_buffer),
            // e.g. +PING (generally used as a response to a command,
            // for example is the incoming command is PING, the response would be +PONG)
            b'+' => Self::parse_simple_string(cursored_buffer),
            // e.g. :1 (denotes integer response type,
            // for example DELETE <key> will return :1 if one record was deleted)
            b':' => Self::parse_integer(cursored_buffer),
            // null value
            b'_' => Ok(DataChunk::Null),
            // error value
            b'-' => Self::parse_simple_errors(cursored_buffer),
            // everything else, catch-all case
            // potentially, when we are trying to parse something that does not exist
            _ => Err(DataChunkError::Parse(format!(
                "Failed to parse unknown data type {:?}",
                first_byte
            ))),
        }
    }

    pub async fn read_chunk_frame(data_chunk: &mut Parser) -> Result<Bytes, DataChunkError> {
        match data_chunk.next() {
            Some(DataChunk::Bulk(data_bytes)) => {
                // This is a hack in order to write consistently formatted values to stdout.
                // Since val without quotes can also be written back to stdout without quotes
                // it is not desirable and therefore we want to add extra quotes to the output value.
                // We need to think about allocations here as it will affect performance in the long run.
                // 34 is "
                if data_bytes.first() != Some(&34) && data_bytes != *PONG {
                    let quotes_bytes = Bytes::from("\"");
                    let concat_bytes = [quotes_bytes.clone(), data_bytes, quotes_bytes].concat();
                    Ok(Bytes::from(concat_bytes))
                } else {
                    Ok(data_bytes)
                }
            }
            Some(DataChunk::Null) => Ok(Bytes::from("(nil)")),
            Some(DataChunk::SimpleError(data_bytes)) => Ok(data_bytes),
            Some(DataChunk::Integer(val)) => {
                // convert Bytes to bytes array
                // then determine endianness to create u64 integer value from the bytes array
                // and return integer as string
                let bytes_slice = val.slice(0..8);

                // converts the slice to an array of u8 elements (since u64 is 8 bytes)
                let arr_u8: [u8; 8] = bytes_slice[0..8].try_into().unwrap();
                let integer_as_string = if cfg!(target_endian = "big") {
                    u64::from_be_bytes(arr_u8)
                } else {
                    u64::from_le_bytes(arr_u8)
                }
                .to_string();

                Ok(Bytes::from(format!("(integer) {}", integer_as_string)))
            }
            None => Ok(Bytes::from("Unknown")),
            _ => Ok(Bytes::from("(nil)")), // catch all case
        }
    }
}

#[cfg(test)]
mod data_chunk_tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn number_of_works() {
        let cursor_inner = [49, b'\r', b'\n'];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = number_of(&mut cursored_buffer);

        assert_eq!(actual, Ok(1));
    }

    #[test]
    fn number_of_works_with_digit_and_chars() {
        let cursor_inner = [50, 111, 112, b'\r', b'\n'];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = number_of(&mut cursored_buffer);
        assert_eq!(actual, Ok(2));
    }

    #[test]
    fn number_of_picks_the_first_number() {
        let cursor_inner = [49, 111, 112, 57, b'\r', b'\n'];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = number_of(&mut cursored_buffer);
        assert_eq!(actual, Ok(1));
    }

    #[test]
    fn number_of_returns_err_when_no_digits_in_value() {
        let cursor_inner = [111, 112, b'\r', b'\n'];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = number_of(&mut cursored_buffer);
        assert_eq!(
            actual,
            Err(DataChunkError::Parse(
                "Failed to parse an integer from the slice".to_owned()
            ))
        );
    }

    #[test]
    fn line_returns_data_before_eol() {
        let cursor_inner = [50, 111, 112, b'\r', b'\n'];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = line(&mut cursored_buffer);
        let expected = [50, 111, 112];
        assert_eq!(actual, Ok(&expected[0..]));
        assert_eq!(cursored_buffer.position(), 5);
    }

    #[test]
    fn line_returns_err_if_no_eol() {
        let cursor_inner = [50, 111, 112];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = line(&mut cursored_buffer);
        assert_eq!(actual, Err(DataChunkError::Insufficient));
        assert_eq!(cursored_buffer.position(), 0);
    }

    #[test]
    fn line_returns_empty_slice_if_no_values_before_eol() {
        let cursor_inner = [b'\r', b'\n'];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = line(&mut cursored_buffer);
        let expected = [];
        assert_eq!(actual, Ok(&expected[..]));
        assert_eq!(cursored_buffer.position(), 2);
    }

    #[test]
    fn line_returns_err_if_no_values_in_buffer() {
        let cursor_inner = [];
        let mut cursored_buffer = Cursor::new(&cursor_inner[..]);

        let actual = line(&mut cursored_buffer);
        assert_eq!(actual, Err(DataChunkError::Insufficient));
    }

    #[test]
    fn parse_array_works() {
        let command_as_data_chunk_string = DataChunk::from_string("SET a b");
        let command_as_bytes = command_as_data_chunk_string.as_bytes();
        let mut cursored_buffer = Cursor::new(command_as_bytes);

        let actual = DataChunk::read_chunk(&mut cursored_buffer);

        let vec_data_chunk = vec![
            DataChunk::Bulk(Bytes::from("SET".to_owned())),
            DataChunk::Bulk(Bytes::from("a".to_owned())),
            DataChunk::Bulk(Bytes::from("b")),
        ];
        let expected = Ok(DataChunk::Array(vec_data_chunk));

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_array_no_bytes_remaining() {
        let command_as_bytes = [0; 0];
        let mut cursored_buffer = Cursor::new(&command_as_bytes[..]);

        let actual = DataChunk::read_chunk(&mut cursored_buffer);

        assert_eq!(actual, Err(DataChunkError::NoBytesRemaining));
    }

    #[test]
    fn parse_array_parse_error() {
        // correct way   "*1\r\n$4\r\nPING\r\n"
        // incorrect way "*\r\n1\r\n$4\r\nPING" <--- using this variant to test
        let command_as_data_chunk_string = String::from("*\r\n1\r\n$4\r\nPING\r\n");
        let command_as_bytes = command_as_data_chunk_string.as_bytes();
        let mut cursored_buffer = Cursor::new(command_as_bytes);

        let actual = DataChunk::read_chunk(&mut cursored_buffer);

        assert_eq!(
            actual,
            Err(DataChunkError::Parse(
                "Failed to parse an integer from the slice".to_owned()
            ))
        );
    }

    #[test]
    fn parse_array_parse_command_length_error() {
        // correct way   "*1\r\n$4\r\nPING\r\n"
        // incorrect way "*1\r\n$\r\n4\r\nPING\r\n" <--- using this variant to test
        let command_as_data_chunk_string = String::from("*1\r\n$\r\n4\r\nPING\r\n");
        let command_as_bytes = command_as_data_chunk_string.as_bytes();
        let mut cursored_buffer = Cursor::new(command_as_bytes);

        let actual = DataChunk::read_chunk(&mut cursored_buffer);

        assert_eq!(
            actual,
            Err(DataChunkError::Parse(
                "Failed to parse an integer from the slice".to_owned()
            ))
        );
    }
}
