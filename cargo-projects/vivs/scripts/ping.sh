#!/bin/bash

# - *1 - data type, number of elements
# - $4 - data type, number of chars in string
# - PING - string which is the actual command

printf '*1\r\n\x244\r\nPING\r\n' | nc -C -N 127.0.0.1 6379
