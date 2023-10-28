#!/bin/bash

# example *3\r\n$3\r\nSET\r\n$4\r\nkey\r\n$5\r\nvalue\r\n
# 
# *3
# An array of 3 items (commands)
#
# $3
# SET
#
# $3
# key
#
# $5
# value
# 
# {#1} - number of characters in in the first command line argument (e.g. key is 3)
# $1 - first command line argument (e.g. hello)
# {#2} - number of characters in in the second command line argument (e.g. value is 5)
# $2 - second command line argument (e.g. hi)

echo ${#1}
echo $1
echo $2
printf "*3\r\n\x243\r\nSET\r\n\x24${#1}\r\n$1\r\n\x24${#2}\r\n$2\r\n" | nc -C -N 127.0.0.1 6379
