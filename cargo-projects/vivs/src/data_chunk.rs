use std::io::Cursor;

use bytes::Buf;

use crate::{Error, Result};

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
                let number = number_of(cursored_buffer)?.try_into()?;
                let mut commands = Vec::with_capacity(number);
                commands.resize_with(3, || DataChunk::Array(vec![]));

                let commands = commands
                    .iter_mut()
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
