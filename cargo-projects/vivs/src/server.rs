use std::{
    collections::HashMap,
    str,
    sync::{Arc, RwLock},
};
use tokio::{
    io::{AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::Command;
use crate::DataStoreWrapper;
use crate::Listener;

#[derive(Debug)]
struct BufSplit(usize, usize);

impl BufSplit {
    fn as_slice<'a>(&self, buffer: &'a Buffer) -> &'a [u8] {
        &buffer[self.0..self.1]
    }
}

#[derive(Debug)]
enum PartBuf {
    String(BufSplit),
    Array(Vec<PartBuf>),
    None,
}

#[derive(Debug)]
enum ParseError {
    Int,
    Unknown,
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
            // byte_position + end + 2 is start + end of word + \r\n -> next positiparse_wordon
            // byte_position - start, byte_position + end - end of actual word
            return Some((
                byte_position + index + 2,
                BufSplit(byte_position, byte_position + index),
            ));
        }
    }
    None
}

/// Returns an integer, for example the number of items in an array and new position cursor
///
/// For example, *2 == returns 2
fn parse_integer(
    buffer: &Buffer,
    byte_position: usize,
) -> Result<Option<(usize, i64)>, ParseError> {
    match parse_word(buffer, byte_position) {
        Some((position, parsed_word)) => {
            let string =
                str::from_utf8(parsed_word.as_slice(buffer)).map_err(|_| ParseError::Int)?;

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
        Some((position, number_of_elements)) => {
            // TODO - very hard-coded value
            let mut v = Vec::with_capacity(number_of_elements as usize);
            let mut pos = position;

            for _ in 0..number_of_elements {
                match parse(buffer, pos)? {
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
        _ => Err(ParseError::Unknown),
    }
}

struct DB {
    store: HashMap<String, String>,
}

impl DB {
    fn new() -> DB {
        DB {
            store: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        let a = self.store.get(key);
        a
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.store.insert(key, value)
    }
}

// previous return type was -> &'a [u8] when
fn get<'a>(buffer: &'a Buffer, key_in_buffer: Option<&PartBuf>) -> std::vec::Vec<u8> {
    // temporary test hash map
    let mut hm = HashMap::new();
    // let mut hm: HashMap<&[u8; 1], &[u8]> = HashMap::new();
    // hm.insert("a", "example\n");

    let some_slice = [104, 101, 108, 108, 111, 50]; // local variable == hello2
    let key_slices: [u8; 3] = [97, 98, 99]; // local variable == abc

    // hm.insert("a", "example\n");
    hm.insert(&key_slices[0..1], &some_slice[0..3]);
    hm.insert(&key_slices[1..2], &some_slice[0..4]);
    hm.insert(&key_slices[2..3], &some_slice[0..5]);

    // extract key value from PartBuf enum
    let key_slice = match key_in_buffer {
        Some(value) => match value {
            PartBuf::String(value) => value.as_slice(buffer),
            _ => panic!("(Error) Key must be a string"),
        },
        None => {
            panic!("(Error) Wrong number of arguments (given 0, expected 1)\n")
        }
    };

    // TODO - Is converstion to utf8 needed here?
    // let key = std::str::from_utf8(key_slice).unwrap_or("");
    // hm.get(key_slice).unwrap_or(&"nil\n").as_bytes()
    // let res = key_slice

    let b = "nil\n".as_bytes();
    let value = hm.get(key_slice).unwrap_or(&b);
    let act = *value;
    act.to_owned()

    // hm.get(key_slice).unwrap_or(&"nil\n".as_bytes())
}

// TODO
// fn set(key, value, buffer: &Buffer) {

// }

fn ping<'a>(buffer: &'a Buffer, message: Option<&PartBuf>) -> &'a [u8] {
    match message {
        Some(val) => match val {
            PartBuf::String(buf_split) => buf_split.as_slice(buffer),
            _ => panic!("Command unrecognised"),
        },
        None => b"PONG\n",
    }
}

fn get_ascii<'a>(key: Option<&PartBuf>, buffer: &'a Vec<u8>) -> &'a [u8] {
    let key_slice = match key {
        Some(value) => match value {
            PartBuf::String(value) => value.as_slice(buffer),
            _ => panic!("(Error) Key must be a string"),
        },
        None => {
            panic!("(Error) Wrong number of arguments (given 0, expected 1)\n")
        }
    };

    let key = std::str::from_utf8(key_slice).unwrap_or("");

    // String::from(key)
    key.as_bytes()
}

fn match_command() {}

fn build_response(buffer: Vec<u8>, ds: DB) {
    let commands = match parse(&buffer, 0) {
        Ok(val) => val.unwrap().1,
        Err(..) => PartBuf::None,
    };

    let commands = match commands {
        PartBuf::Array(v) => v,
        _ => vec![],
    };

    if let Some(first_command) = commands.first() {
        match first_command {
            PartBuf::String(v) => {
                let command = v.as_slice(&buffer);

                match command {
                    b"PING" => ping(&buffer, commands.get(2)),
                    b"GET" => get_ascii(commands.get(1), &buffer),
                    b"SET" => {
                        let key = commands.get(1);
                        let value = commands.get(2);

                        let key_ascii = get_ascii(key, &buffer);
                        let value_ascii = get_ascii(value, &buffer);

                        value_ascii

                        // hm.set(key_ascii, value_ascii);
                        // write.try_write("OK".as_bytes())?
                    }
                    _ => {
                        // write.try_write(b"unrecognised command at all\n")?
                        command
                    }
                };
                // extra newline to format for command line better
                // write.try_write(b"\n")?;
            }
            _ => println!("ERR"),
        }
    }
}

async fn handle_stream(
    mut stream: TcpStream,
    _addr: std::net::SocketAddr,
    hm: Arc<RwLock<DB>>,
) -> std::io::Result<()> {
    let (read, write) = stream.split();
    let mut reader = BufReader::new(read);

    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).await?;

    let commands = match parse(&buffer, 0) {
        Ok(val) => val.unwrap().1,
        Err(..) => PartBuf::None,
    };

    let commands = match commands {
        PartBuf::Array(v) => v,
        _ => vec![],
    };

    // thread is blocked with exclusive write access
    let mut rw_lock_guard = hm.write().unwrap();
    // get access to mutable DB reference by dereferencing rw lock write guard
    let hm = &mut rw_lock_guard;

    if let Some(first_command) = commands.first() {
        match first_command {
            PartBuf::String(v) => {
                let command = v.as_slice(&buffer);

                match command {
                    b"PING" => {
                        // let message = commands.get(2);
                        // let a = ping(&buffer, message);
                        // write.try_write(a)?

                        // crate::commands::Ping::response(self, conn)

                        let cmd = Command::parse_cmd().unwrap();
                        // pass db and connection
                        // cmd.run();
                        1
                    }
                    b"GET" => {
                        let key = commands.get(1);
                        let key_ascii = get_ascii(key, &buffer);

                        let s = std::str::from_utf8(key_ascii).unwrap();

                        if let Some(value) = hm.get(s) {
                            write.try_write(value.as_bytes())?
                        } else {
                            write.try_write("nil".as_bytes())?
                        }
                    }
                    b"SET" => {
                        let key = commands.get(1);
                        let value = commands.get(2);

                        let key_ascii = get_ascii(key, &buffer);
                        let s1 = std::str::from_utf8(key_ascii).unwrap().to_string();

                        let value_ascii = get_ascii(value, &buffer);
                        let s2 = std::str::from_utf8(value_ascii).unwrap().to_string();

                        // key to ascii
                        hm.set(s1, s2);
                        write.try_write("OK".as_bytes())?
                    }
                    b"DEL" => {
                        // delete key
                        write.try_write("OK".as_bytes())?
                    }
                    _ => write.try_write(b"unrecognised command at all\n")?,
                };
                // extra newline to format for command line better
                write.try_write(b"\n")?;
            }
            _ => println!("ERR"),
        }
    }

    Ok(())
}

pub async fn run(addr: String, port: String) -> crate::Result<()> {
    let location = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(&location).await?;

    println!("Listening on {:?}", location);

    // Creates a new read write lock that protects the DB (data store) instance
    // wrapped in atomic reference counter (ARC) to enable safe sharing between threads
    let data_store = Arc::new(RwLock::new(DB::new()));

    loop {
        let (stream, addr) = listener.accept().await?;
        // each thread gets a clone (increasing the strong reference counter), a pointer to the same location
        let hm_clone = Arc::clone(&data_store);

        tokio::spawn(async move { handle_stream(stream, addr, hm_clone).await });
    }
}

// New improved way of handling requests
pub async fn start(addr: String, port: String) -> crate::Result<()> {
    let address = format!("{}:{}", addr, port);
    let tcp_listener = TcpListener::bind(&address).await?;

    let listener = Listener {
        tcp_listener,
        db: DataStoreWrapper::new(),
    };

    listener.run().await?;

    Ok(())
}
