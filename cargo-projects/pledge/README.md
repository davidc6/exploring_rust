# Simple memory allocator

## Memory Layout & Alignment

### Alignment

This is to describe where the bytes for a type will be stored. It is the arrangement of data in memory. Alignment comes into play when the objects that need to be aligned are larger than the smallest addressable unit. Certain types can only live at certain memory locations for the compiler to generate good, efficient code. It is the way data is arranged in memory.

We cannot use an arbitrary memory location since hardware has some constraints around where a type can be placed. All values must start at a byte boundary (values cannot be put at a bit in memory) i.e. they have to be byte-aligned (and placed at an address that is a multiple of 8 i.e. bits).

CPU often accesses memory in blocks (typically larger than 1 byte). This is referred to as the CPU's **word size**. For example, on 64 bite machines most values are accessed in chunks of 8 bytes and operations start at an 8-byte aligned address. 

If a pointer is not 8-byte aligned, it's not as efficient for the CPU. It has to do two reads, one from the second half of the first 8 byte chunk and then the other from first half of the second chunk and then splice the two together. Additionally, since there are two reads, any of these parts can be written to by another thread concurrently and unexpected results may arise. This is referred to as  **misaligned access** which could lead to bad performance and concurrency issues. It is preferred to have **naturally aligned** CPU arguments. 

### Memory Layout

This is how a compiler decides to organise and store a type in a program's memory. 

### Rust Traits

Allocator (experimental) - can allocate, grow, shrink, and deallocate blocks of data
GlobalAlloc - a memory allocator that can be registered as the std default 

### Data Structures

Free List - in order to track which parts of free memory are not in use and to allocate required memory by the processes a number of different data structures can be used. One of the most common and simplest ones is a free list. This is a list of memory that are not in use. 