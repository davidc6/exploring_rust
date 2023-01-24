use tokio::{net::{TcpListener, TcpStream}, io::{BufReader, AsyncReadExt}};
use std::{str};

#[derive(Debug)]
struct BufSplit(usize, usize);

impl BufSplit {
    fn as_slice<'a>(&self, buffer: &'a Buffer) -> &'a[u8] {
        &buffer[self.0..self.1]
    }
}

#[derive(Debug)]
enum PartBuf {
    String(BufSplit),
    Array(Vec<PartBuf>),
    None
}

#[derive(Debug)]
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

    for (index, val) in buffer[byte_position..].iter().enumerate() {
        if val == &b'\r' {
            // byte_position + end + 2 is start + end of word + \r\n -> next position
            // byte_position - start, byte_position + end - end of actual word
            return Some((byte_position + index + 2, BufSplit(byte_position, byte_position + index)));
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

            // error does not propogate
            let integer = string.parse::<i64>().map_err(|_| ParseError::Int)?;

            Ok(Some((position, integer)))
        }
        None => Ok(None),
    }
}

fn parse_array(buffer: &Buffer, byte_position: usize) -> ParseResult {
    match parse_integer(buffer, byte_position)? {
        None => Ok(None),

        Some((position, number_of_arr_elements)) => {            
            // TODO - very hard-coded value 
            let mut v = Vec::with_capacity(number_of_arr_elements as usize);
            let mut pos = position;

            // v.push(BufSplit(byte_position, position));
            // Ok(Some((3, PartBuf::Array(v))))

            for _ in 0..number_of_arr_elements {
                match parse(buffer, position)? {
                    Some((position, val)) => {
                        pos = position;
                        v.push(val);
                    }
                    None => return Ok(None),
                }
            }
            Ok(Some((pos, PartBuf::Array(v))))
        }
    }
}

fn parse_bulk_str(buffer: &Buffer, byte_position: usize) -> ParseResult {
    // number of chars in the string of each element in the array
    match parse_integer(buffer, byte_position)? {
        None => Ok(None),
        Some((position, length)) => {
            let total = position + length as usize;
            let s = PartBuf::String(BufSplit(position, total));
            Ok(Some((total + 2, s)))
        }
    }
}

/// Example stream to parse: *1\r\n$4\r\nPING\r\n
fn parse(buffer: &Buffer, byte_position: usize) -> ParseResult {
    // we need to check the buffer since the buffer could potentially be empty
    if buffer.is_empty() {
        return Ok(None);
    }

    // call relevant parser and process a chunk by matching on a data type
    // byte_position + 1 == skips the data type symbol when parsing
    match buffer[byte_position] {
        b'*' => parse_array(buffer, byte_position + 1),
        b'$' => parse_bulk_str(buffer, byte_position + 1),
        _ => Err(ParseError::Unknown)
    }
}

async fn handle_stream(mut stream: TcpStream, _addr: std::net::SocketAddr) -> std::io::Result<()> {
    let (read, write) = stream.split();
    let mut reader = BufReader::new(read);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).await?;

    let commands = match parse(&buffer, 0) {
        Ok(val) => val.unwrap().1,
        Err(..) => PartBuf::None
    };

    let commands = match commands {
        PartBuf::Array(v) => v,
        _ => vec!()
    };

    // currently this only works for a single command
    for val in commands {
        match val {
            PartBuf::String(v) => {
                let command = v.as_slice(&buffer);

                match command {
                    b"PING" => write.try_write(b"+PONG\n")?,
                    _ => write.try_write(b"unrecognised command")?
                };
            }
            _ => println!("ERR")
        }
    }

    Ok(())
}

pub async fn start(addr: String, port: String) -> std::io::Result<()> {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await?;

    loop {
        let (stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            handle_stream(stream, addr).await
        });
    }
}
