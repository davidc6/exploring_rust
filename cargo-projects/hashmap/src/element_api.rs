use crate::hashtable_vec::Bucket;
use std::fmt::Debug;

impl<Key: Debug, Value: Debug> Debug for Element<'_, Key, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Element::Filled(ref filled) => f.debug_tuple("Element").field(filled).finish(),
            Element::Empty(ref empty) => f.debug_tuple("Element").field(empty).finish(),
        }
    }
}

impl<'a, Key: Debug + Eq, Value: Debug + Eq> Element<'a, Key, Value> {
    pub fn or_set(self, val: Value) -> &'a mut Value {
        match self {
            Element::Filled(filled) => {
                let hash = filled.hash as usize;
                let bucket = filled.ht.get_mut(hash).unwrap();
                let element = bucket
                    .items
                    .iter_mut()
                    .find(|current_val| filled.key == current_val.0)
                    .unwrap(); // Ok to unwrap since we know that the value exists somewhere in the Vector based on the previous check
                let (_, value) = element;
                value
            }
            Element::Empty(empty) => {
                let hash = empty.hash as usize;
                let mut bucket_mut = empty.ht.get_mut(hash);

                // could also use ref allows to bind by reference when pattern matching rather than consuming the value
                if let Some(bucket_mut) = &mut bucket_mut {
                    bucket_mut.items.push((empty.key, val));
                }

                let (_, value) = bucket_mut.unwrap().items.first_mut().unwrap();
                value
            }
        }
    }
}

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

#[cfg(test)]
mod element_api_tests {
    use crate::hashtable_vec::HashTable;

    #[test]
    fn or_set_does_not_insert_value_if_value_exists_in_table() {
        let mut ht = HashTable::new();
        ht.set("hello", "world");

        let item = ht.element("hello").or_set("world2");

        assert_eq!(item, &mut "world");
    }

    #[test]
    fn or_set_inserts_value_if_value_does_not_exist_in_table() {
        let mut ht = HashTable::new();
        let item = ht.element("hello").or_set("world2");

        assert_eq!(item, &mut "world2");
        assert_eq!(ht.get("hello"), Some(&"world2"));
    }
}
