use std::ptr::NonNull;

struct Metadata {
    block_size: usize,
    is_free: bool,
}

struct Footer {}

struct Chunk<T> {
    metadata: Metadata,
    next_chunk: Option<NonNull<T>>,
    prev_chunk: Option<NonNull<T>>,
    footer: Footer,
}
