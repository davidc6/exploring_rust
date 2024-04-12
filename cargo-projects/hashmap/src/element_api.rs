use crate::hashtable_vec::Bucket;
use std::fmt::Debug;

#[derive(PartialEq)]
pub struct Filled<'a, Key, Value> {
    pub hash: u64,
    pub key: Key,
    pub value: Value,
    pub ht: &'a mut Vec<Bucket<Key, Value>>,
}

impl<'a, Key, Value> Filled<'a, Key, Value> {
    fn key(&self) -> &Key {
        &self.key
    }
}

impl<Key: Debug, Value: Debug> Debug for Filled<'_, Key, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FilledElement").field(self.key()).finish()
    }
}

#[derive(PartialEq)]
pub struct Empty<'a, Key, Value> {
    pub hash: u64,
    pub key: Key,
    pub ht: &'a mut Vec<Bucket<Key, Value>>,
}

impl<'a, Key, Value> Empty<'a, Key, Value> {
    fn key(&self) -> &Key {
        &self.key
    }
}

// Here the compiler infers the lifetime (i.e. the lifetime is elided)
impl<Key: Debug, Value> Debug for Empty<'_, Key, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EmptyElement").field(self.key()).finish()
    }
}

#[derive(PartialEq)]
pub enum Element<'a, Key: 'a, Value: 'a> {
    Empty(Empty<'a, Key, Value>),
    Filled(Filled<'a, Key, Value>),
}
