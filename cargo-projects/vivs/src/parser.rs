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
            DataChunk::SimpleError(value) => vec![DataChunk::SimpleError(value)],
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

    pub fn peek(&mut self) -> Option<&DataChunk> {
        // let a = self.segments.as_ref();
        // let p = self.segments.peekable();

        // let b = p.next();
        let c = self.segments.peek();
        c
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
