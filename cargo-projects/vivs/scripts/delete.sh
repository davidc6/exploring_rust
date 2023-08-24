
#!/bin/bash

# {#1} - number of characters in in the first command line argument (e.g. hello is 5)
# $1 - first command line argument (e.g. hello)
printf "*2\r\n\x246\r\nDELETE\r\n\x24${#1}\r\n$1\r\n" | nc -C -N 127.0.0.1 6379
