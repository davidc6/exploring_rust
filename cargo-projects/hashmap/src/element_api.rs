use crate::hashtable_vec::HashTable;
use std::fmt::Debug;

#[derive(PartialEq)]
pub struct FilledElement<'a, Key, Value> {
    pub hash: u64,
    pub key: Key,
    pub value: Value,
    pub ht: &'a mut HashTable<Key, Value>,
}

impl<'a, Key, Value> FilledElement<'a, Key, Value> {
    fn key(&self) -> &Key {
        &self.key
    }
}

// Here the compiler infers the lifetime (i.e. the lifetime is elided)
impl<Key: Debug, Value: Debug> Debug for FilledElement<'_, Key, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FilledElement").field(self.key()).finish()
    }
}

#[derive(PartialEq)]
pub struct EmptyElement<'a, Key, Value> {
    pub hash: u64,
    pub key: Key,
    pub ht: &'a mut HashTable<Key, Value>,
}

impl<'a, Key, Value> EmptyElement<'a, Key, Value> {
    fn key(&self) -> &Key {
        &self.key
    }
}

impl<Key: Debug, Value> Debug for EmptyElement<'_, Key, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EmptyElement").field(self.key()).finish()
    }
}

#[derive(PartialEq)]
pub enum Element<'a, Key: 'a, Value: 'a> {
    Empty(EmptyElement<'a, Key, Value>),
    Filled(FilledElement<'a, Key, Value>),
}

impl<Key: Debug, Value: Debug> Debug for Element<'_, Key, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
                let bucket = filled.ht.buckets.get_mut(hash).unwrap();

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
                let ht_mut = empty.ht;
                let mut bucket_mut = ht_mut.buckets.get_mut(hash);

                // Could also use Some(mut ref bucket_mut),
                // which allows to bind by reference when pattern matching rather than consuming the value
                if let Some(bucket_mut) = &mut bucket_mut {
                    ht_mut.items += 1;
                    bucket_mut.items.push((empty.key, val));
                }

                let (_, value) = bucket_mut.unwrap().items.first_mut().unwrap();
                value
            }
        }
    }

    pub fn key(&self) -> &Key {
        match self {
            Element::Empty(empty) => empty.key(),
            Element::Filled(filled) => filled.key(),
        }
    }
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

    #[test]
    fn key_returns_a_reference_to_element_key() {
        let mut ht = HashTable::new();
        ht.set("hello", "world");
        ht.set("hello2", "world2");

        let element = ht.element("hello");
        assert_eq!(element.key(), &"hello");
    }

    #[test]
    fn key_returns_a_reference_to_element_key_if_key_is_not_set() {
        let mut ht: HashTable<&str, &str> = HashTable::new();

        let element = ht.element("hello");

        assert_eq!(element.key(), &"hello");
        assert_eq!(ht.length(), 0);
    }
}
