use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt, AsyncBufReadExt}};
use std::{str};

#[derive(Debug)]
enum DataTypes {
    Array
}

// carriage return 
fn parse_until_crlf(stream: &[u8]) -> Result<(&[u8], &[u8]), String> {
    for (index, val) in stream.iter().enumerate() {
        if val == &b'\r' && stream[index + 1] == b'\n' {
            // result and rest
            return Ok((&stream[..index], &stream[index + 2..stream.len()]));
        }
    }

    // error
    return Err("Err".to_owned());
}

// fn parse_bulk_strings(buffer: &[u8]) -> Result<(Vec<&[u8]>, &[u8]), ()> {
    // 4\r\nPING\r\n

//     let array = parse_until_crlf(buffer);

//     let str_size = array.as_ref().unwrap().0;
//     let left = array.as_ref().unwrap().1;

//     let mut vec: Vec<_> = Vec::with_capacity(std::str::from_utf8(str_size).unwrap().parse::<u64>().unwrap() as usize);

//     let pos = str_size[0] as char;
//     let res = parse(&left[pos.to_digit(10).unwrap() as usize..]);

//     Ok((vec, left))
// }

// fn parse_array(buffer: &[u8]) -> Result<(Vec<&[u8]>, &[u8]), ()> {
//     let array = parse_until_crlf(buffer);

//     let arr_size = array.as_ref().unwrap().0;
//     let left = array.as_ref().unwrap().1;
    
//     let mut vec: Vec<_> = Vec::with_capacity(std::str::from_utf8(arr_size).unwrap().parse::<u64>().unwrap() as usize);

//     let res = parse(left);

//     Ok((vec, left))
// }


struct BufSplit(usize, usize);

impl BufSplit {
    fn as_slice<'a>(&self, buffer: &'a Buffer) -> &'a[u8] {
        &buffer[self.0..self.1]
    }
}

enum PartBuf {
    String(BufSplit),
    Array(Vec<BufSplit>)
}

enum ParseError {
    Int,
    Unknown
}

// represents parse result
type ParseResult = Result<Option<(usize, PartBuf)>, ParseError>;
type Buffer = [u8];

/// Returns everything before the newline
/// 
/// For example, the function takes in a buffer which contains 'hello\r\n' and return Option<(8, (1, 5))>
fn parse_word(buffer: &Buffer, byte_position: usize) -> Option<(usize, BufSplit)> {
    // edge cases?

    // tracks end of the word
    let mut end_position = byte_position;

    for val in &buffer[byte_position..] {
        end_position += 1;

        if val == &b'\r' {
            // byte_position + end + 2 is start + end of word + \r\n -> next position
            // byte_position - start, byte_position + end - end of actual word
            return Some((byte_position + end_position + 2, BufSplit(byte_position, byte_position + end_position)));
        }
    }

    None
}

/// Returns an integer, for example the number of items in an array and new position cursor
/// 
/// For example, *2 == returns 2
fn parse_integer(buffer: &Buffer, byte_position: usize) -> Result<Option<(usize, i64)>, ParseError> {
    match parse_word(buffer, byte_position) {
        Some((position, parsed_word)) => {
            let string = str::from_utf8(parsed_word.as_slice(buffer)).map_err(|_| ParseError::Int)?;
            let integer = string.parse::<i64>().map_err(|_| ParseError::Int)?;
            Ok(Some((position, integer)))
        }
        None => Ok(None),
    }
}

/// 
fn parse_array(buffer: &Buffer, byte_position: usize) -> ParseResult {
    match parse_integer(buffer, byte_position)? {
        None => Ok(None),
        Some((position, number_of_arr_elements)) => {
            // hard-coded
            let mut v = Vec::with_capacity(1);
            v.push(BufSplit(1, 2));
            Ok(Some((3, PartBuf::Array(v))))
        }
    }

    // return Ok(None);
}

fn parse(buffer: &Buffer, byte_position: usize) -> ParseResult {
    // we need to check the buffer since the buffer could potentially be empty
    if buffer.is_empty() {
        return Ok(None);
    }

    // call relevant parser and process a chunk by matching on a data type
    // byte_position + 1 == skips the data type symbol when parsing
    match buffer[byte_position] {
        b'*' => parse_array(buffer, byte_position + 1),
        _ => Err(ParseError::Unknown)
    }
}

// fn parse(buffer: &[u8]) -> Result<(), ()> {
//     println!("Buffer {:?}\n", buffer);

//     if let Some(data_type) = buffer.first() {
//         let data_type = match data_type {
//             b'*' => parse_array(&buffer[1..buffer.len()]),
//             b'$' => parse_bulk_strings(&buffer[1..buffer.len()]),
//             b'\r' => Ok((Vec::with_capacity(1), buffer)),
//             _ => Ok((Vec::with_capacity(1), buffer))
//         };

//         return Ok(());
//     } else {
//         // run the command
//         // panic!("Unexpected");
//         return Ok(());
//     }

//     return Ok(());
// }

async fn handle_stream(mut stream: TcpStream, addr: std::net::SocketAddr) -> std::io::Result<()> {
    let (read, write) = stream.split();
    let reader = BufReader::new(read);
    let buffer = reader.buffer();
    let commands = parse(buffer, 0);

    // result?

    Ok(())
}

pub async fn start(addr: String, port: String) -> std::io::Result<()> {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await?;

    loop {
        let (mut stream, addr) = listener.accept().await?; // TODO - checks?

        tokio::spawn(async move {
            handle_stream(stream, addr).await;
        });
    }
}
