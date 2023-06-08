#!/bin/bash

# - *1 - data type, number of elements
# - $4 - data type, number of chars in string
# - PING - string which is the actual command

data_type='\r\n\x244\r\n'

# For now, any argument can be passed into the script to trigger this conditional
if [ -z $1 ]; then 
  command="*1${data_type}PING\r\n"
else 
  command="*2${data_type}PING\r\n\x243\r\nYES\r\n"
fi

# This is for debugging purposes, to see what we are sending to the server
# use -e flag to process \r\n
echo $command

printf $command | nc -C -N 127.0.0.1 6379
