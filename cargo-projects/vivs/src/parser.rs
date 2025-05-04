use crate::{
    data_chunk::{DataChunk, DataChunkError},
    GenericResult,
};
use bytes::{Buf, Bytes};
use std::{iter::Peekable, vec::IntoIter};

impl Default for Parser {
    fn default() -> Self {
        Parser {
            segments: vec![].into_iter().peekable(),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    segments: Peekable<IntoIter<DataChunk>>,
}

impl Parser {
    /// Constructs Parser by parsing the incoming buffer
    pub fn new(data_chunk: DataChunk) -> GenericResult<Self> {
        let data_chunks_vec = match data_chunk {
            DataChunk::Array(val) => val,
            DataChunk::Bulk(value) => vec![DataChunk::Bulk(value)],
            DataChunk::Null => vec![DataChunk::Null],
            DataChunk::Integer(value) => vec![DataChunk::Integer(value)],
            DataChunk::SimpleError(value) => value,
        };

        let segments = data_chunks_vec.into_iter();

        Ok(Parser {
            segments: segments.peekable(),
        })
    }

    /// Tries to get next element in the collection/segments.
    ///
    /// If the element exists then a String type gets returned,
    /// otherwise an error is returned.
    ///
    /// The reason for the Result return type is because we attempt to convert a
    /// slice of bytes to a string slice in the match expression if it can potentially error.
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

    /// Enables to peek into the DataChunks iterator.
    /// This is useful when needing to get the key,
    /// in order to establish the node that holds the value of that key.
    /// Peeking does not remove values and does not forward the iterator.
    pub fn peek(&mut self) -> Option<&DataChunk> {
        self.segments.peek()
    }

    #[allow(clippy::unwrap_in_result)]
    pub fn peek_as_str(&mut self) -> Option<String> {
        let segment = self.segments.peek();

        match segment {
            Some(DataChunk::Bulk(value)) => {
                Some(std::str::from_utf8(value.chunk()).unwrap().to_owned())
            }
            Some(DataChunk::SimpleError(value)) => {
                let val = value.first().unwrap();

                let bytes = match val {
                    DataChunk::Bulk(bulk_val) => bulk_val,
                    // TODO
                    _ => "hi".as_bytes(),
                };

                Some(std::str::from_utf8(bytes).unwrap().to_owned())
            }
            _ => None,
        }
    }

    pub fn enumerate(self) -> std::iter::Enumerate<Peekable<IntoIter<DataChunk>>> {
        self.segments.enumerate()
    }

    pub fn iter(self) -> Peekable<IntoIter<DataChunk>> {
        self.segments
    }

    pub fn size(&self) -> usize {
        self.segments.len()
    }

    pub fn push_up(mut self, bytes: Bytes) -> Self {
        let mut data_chunks_collection: Vec<DataChunk> = self.segments.collect();
        let mut data_chunks_vec = vec![DataChunk::Bulk(bytes)];
        data_chunks_vec.append(&mut data_chunks_collection);
        self.segments = data_chunks_vec.into_iter().peekable();
        self
    }

    /// Creates a new iterator by first, converting the existing iterator into the vector,
    /// pushing a value to it and then creating a brand new iterator from it.
    pub fn push(mut self, bytes: Bytes) -> Self {
        let mut data_chunks_collection: Vec<DataChunk> = self.segments.collect();
        data_chunks_collection.push(DataChunk::Bulk(bytes));
        self.segments = data_chunks_collection.into_iter().peekable();
        self
    }

    pub fn push_bulk_str(mut self, bytes: Bytes) -> Self {
        // TODO: this is a hack (for now).
        // Convert iterator to vector in order to push data chunks into it.
        // This functionality is part of the so called "client encoder".
        let mut data_chunks_collection: Vec<DataChunk> = self.segments.collect();
        data_chunks_collection.push(DataChunk::Bulk(bytes));
        self.segments = data_chunks_collection.into_iter().peekable();
        self
    }
}

impl Iterator for Parser {
    type Item = DataChunk;

    fn next(&mut self) -> Option<Self::Item> {
        self.segments.next()
    }
}
