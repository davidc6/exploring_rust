# README

Vivs is a simple, (currently) experimental in-memory data store.

**This project is still in its early stages and work in progress**

### Guide

To run:

```sh
# server
cargo run --bin vivs

# server with logs
RUST_LOG=vivs --bin vivs

# client / repl, to execute commands
cargo run --bin vivs-repl
```

To run integration tests:

```sh
cargo test --test commands
```

### Commands

- `PING [value]` - pings the server, tests whether it's alive and can be also used to test latency
- `GET key` - gets the value by key from the server
- `SET key value [XS seconds]` - sets key to hold the value, optionally setting expire time
    - `XS` option (stands for [X]Expire [S]Seconds)
- `DELETE key` - deletes key from the store
- `TTL key` - checks whether a key has time to live (expiry time)

### Examples (without using the repl/client)

`PING` command:

```sh
# Option 1
# 
# -C    sends CRLF as line-ending
# -N    shuts down the network socket after EOF on the input (required by some servers to finish work) 
printf '*1\r\n\x244\r\nPING\r\n\r\n' | nc -C -N 127.0.0.1 6379

# Option 2
#
# -e    this flag enables interpretation of backslash escapes
# -e    echo -e '*1\r\n\x244\r\nPING\r\n\r\n' | nc -C -N 127.0.0.1 6379
echo -e '*1\r\n\x244\r\nPING\r\n\r\n' | nc -C -N 127.0.0.1 6379
```

`SET` command:

```sh
printf '*3\r\n\x243\r\nSET\r\n\x241\r\na\r\n\x243\r\n123\r\n' | nc -C -N 127.0.0.1 6379
```

`GET` command:

```sh
printf '*2\r\n\x243\r\n\GET\r\n\x241\r\na\r\n' | nc -C -N 127.0.0.1 6379
```

`POST` command

```sh
# TODO
```

### Client implementation notes

- `0xA` - newline char
- `\x24` - hex for $ (dollar sign)

## General architecture

- Client sends a frame which server parses
- Server parses the payload by splitting it into "chunks" (Example `*1$4PING` get split into `*1`, `$4`, `PING`)
- Server then writes back to the stream which is read by the client

## TODOs

- [x] Logging (.log) for all commands
- [x] PING
- [x] SET
- [x] GET
- [x] DELETE
- [x] Save strings that contain spaces i.e. "Hello world"
- [x] Build a REPL to test commands
- [ ] HELLO (a command that returns instance information)
- [X] TTL command, implement using a simple algorithm that checks if key is still valid when getting or ttling it
- [ ] TTL (semi-active i.e. check ttl when key is being accessed AND/OR active i.e. sort keys by expiration in radix tree)
- [ ] Build a client (connect to kv store, call get, set, delete commands)
- [ ] GET command should only have one option for now
- [ ] Repl EXIT command
- [X] On DELETE remove expiration key
- [ ] Flag any commands options that are incorrect to the user, also would be nice to have some sort of a command completion

### Notes

- `netcat` has to be installed in the container, in order to install login as root (`docker exec -it bash`)
- EOF for TCP connection means the connection has been closed, not just the sender temporarily stopped sending more data.
- A bytestream is a sequence of bytes that is used to perform input and output operations which essentially is 8 bits composed of 0s and 1s.
- Parsing combinators - combining more parsers in a single parser.
- `frame` (networking) is a chunk of bits that a host/client can send

### References

- Redis: under the hood - https://www.pauladamsmith.com/articles/redis-under-the-hood.html
- Intro to Redis DS - https://scalegrid.io/blog/introduction-to-redis-data-structures-hashes/
- Memory optimization - https://redis.io/docs/management/optimization/memory-optimization/
- Redis Data Structures - https://redis.com/redis-enterprise/data-structures/
- A collection of Redis internals links - https://abgoswam.wordpress.com/2016/11/22/redis-internals/
