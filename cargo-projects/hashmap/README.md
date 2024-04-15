# HashTable

A super simpler Hash Table (i.e. Hash Map) implementation in Rust. I chose not to use the same method names (or API) as Rust's HashMap.

Please do not use this in the production as it is still under heavy development.

## Overview

```rs
let mut hash_table = HashTable::new();

// Set / insert a key and value
hash_table.set("hello", "world");
// Get a value by key
hash_table.get("hello");
// Delete a value by key
hash_table.delete("hello");

// Using Element API, enables to add extra functionality
hash_table.entry("hello"); // returns FilledElement or EmptyElement enum value
// Set key and value if the key does not already exist
hash_table.entry("hello").or_set("world");
```

### Resources

- Live-coding a linked hash map in Rust - https://www.youtube.com/watch?v=k6xR2kf9hlA
- Entry API - https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
- 
