# README

Vivs is an in-memory data store.

### Examples

`PING` command:

```sh
# Option 1
# 
# -C    sends CRLF as line-ending
# -N    shutsdown the network socket after EOF on the input (required by some servers to finish work) 
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

### Notes

- `netcat` has to be installed in the container, in order to install login as root (`docker exec -it bash`)
- EOF for TCP connection means the connection has been closed, not just the sender temporarily stopped sending more data.
- A bytestream is a sequence of bytes that is used to perform input and output operations which essentially is 8 bits composed of 0s and 1s.
- Parsing combinators - combining more parsers in a single parser.
- `frame` (networking) is a chunk of bits that a host/client can send