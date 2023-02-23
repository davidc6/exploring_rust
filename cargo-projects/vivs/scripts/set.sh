#!/bin/bash

# {#1} - number of characters in in the first command line argument (e.g. hello is 5)
# $1 - first command line argument (e.g. hello)
# {#2} - number of characters in in the second command line argument (e.g. hi is 2)
# $2 - second command line argument (e.g. hi)
printf "*3\r\n\x243\r\nSET\r\n\x24${#1}\r\n$1\r\n\x24${#2}\r\n$2\r\n" | nc -C -N 127.0.0.1 6379
