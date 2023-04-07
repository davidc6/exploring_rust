use core::num;
use std::io::{self, Cursor};

use bytes::Buf;

use crate::{Error, Result};

// Gets number of either elements in array or string length
#[allow(dead_code)]
fn number_of(cursored_buffer: &mut Cursor<&[u8]>) -> std::result::Result<u64, Error> {
    use atoi::atoi;

    let current_position = cursored_buffer.position() as usize;
    let length = cursored_buffer.get_ref().len();
    let buffer_slice = &cursored_buffer.get_ref()[current_position..length];

    atoi::<u64>(buffer_slice).ok_or_else(|| "could not parse integer, invalid format".into())
}

struct DataChunk {}

impl DataChunk {
    fn parse(cursored_buffer: &mut Cursor<&[u8]>) -> Result<u64> {
        match cursored_buffer.get_u8() {
            b'*' => {
                let number = number_of(cursored_buffer)?.try_into()?;

                let mut commands = Vec::with_capacity(number);

                // TODO - functional ?
                // commands
                //     .into_iter()
                //     .map(|a: u8| DataChunk::parse(cursored_buffer));

                for _ in 0..number {
                    // create a vector with parsed command
                    commands.push(DataChunk::parse(cursored_buffer));
                }

                // TODO fix return
                Ok(number as u64)
            }
            _ => unimplemented!(),
        }
    }
}
