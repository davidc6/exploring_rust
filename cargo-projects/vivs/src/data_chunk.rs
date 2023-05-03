use bytes::Buf;
use std::io::Cursor;

#[derive(Debug)]
pub enum Error {
    Insufficient,
    Uknown(String),
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

#[derive(Debug)]
pub enum DataChunk {
    Array(Vec<DataChunk>),
}

impl DataChunk {
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
                println!("Number");
                // get line and push into array
                let result = line(cursored_buffer);
                println!("{:?}", result);

                Ok(DataChunk::Array(vec![]))
            }
            _ => {
                println!("LINE: {:?}", line(cursored_buffer));
                println!("U8: {:?}", cursored_buffer.get_u8());
                unimplemented!();
            }
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Uknown(value.into())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Error {
        value.to_string().into()
    }
}
