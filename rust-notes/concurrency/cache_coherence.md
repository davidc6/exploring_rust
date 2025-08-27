# Cache Coherence

Cache coherence is essential for concurrent systems. Since caching shared data 
introduces a problem of memory view, cache coherence ensures that multiple 
processors see a consistent view of memory.

This can happen when multiple processors have memory view held in their individual 
caches. Without any precautions, different values can be seen for the same memory 
location. This is referred to as *cache coherence problem*.

We have a global state (defined by main memory) and local state (defined by 
individual caches, private to each processor core). In a multicore system, some 
caches levels are shared such as L3 and some are private to each core such as L1 and L2. 
If any read of a data returns the most recently written value then we can informally 
say that the memory system is coherent. 

Cache coherence approaches can be software and hardware based. 

## Software Solution

A software approach relies on complies and operating system. Compiler-based approach 
analyses the code to determine which data may become unsafe to cache. Then the OS 
and hardware prevent noncacheable items from being cached. 

## Hardware Solution

Hardware-based solutions are referred to cache coherence protocols. These can be 
divided into two categories:

1) Snooping-based (or bus-based)

If a data is missing in cache, then that core creates a read request for and places 
it on the shared bus. All other cache on the same bus are able to read and understand 
the request. If no caches is found then it will be provided by main memory.

MESI (protocol) - Modified / Exclusive / Shared / Invalid

2) Directory-based


