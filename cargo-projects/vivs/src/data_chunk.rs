use crate::Error;
use bytes::Buf;
use std::io::Cursor;

// Gets number of either elements in array or string char count
fn number_of(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<u64, Error> {
    use atoi::atoi;

    // current cursor position
    let current_position = cursored_buffer.position() as usize;
    // number of overall elements in the underlying value
    let length = cursored_buffer.get_ref().len();
    // underlying slice
    let buffer_slice = &cursored_buffer.get_ref()[current_position..length];

    atoi::<u64>(buffer_slice).ok_or_else(|| "could not parse integer from a slice".into())
}

pub enum DataChunk {
    Array(Vec<DataChunk>),
}

impl DataChunk {
    pub fn parse(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<DataChunk, Error> {
        match cursored_buffer.get_u8() {
            b'*' => {
                // use range expression which implements Iterator trait enables to map over each element
                // then collect iterator into a vector
                let commands = (0..number_of(cursored_buffer)?.try_into()?)
                    .map(|_| {
                        DataChunk::parse(cursored_buffer)
                            .unwrap_or_else(|_| panic!("Could not parse"))
                    })
                    .collect::<Vec<DataChunk>>();

                Ok(DataChunk::Array(commands))
            }
            _ => unimplemented!(),
        }
    }
}
