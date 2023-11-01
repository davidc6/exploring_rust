#!/bin/bash

# - *1 - data type, number of elements
# - $4 - data type, number of chars in string
# - PING - string which is the actual command

data_type='\r\n\x244\r\n'

# For now, any argument can be passed into the script to trigger this conditional
# -z checks if the string is NULL, zero length
if [ -z $1 ]; then 
  command="*1${data_type}PING\r\n"
else 
  message_length="${#1}"
  command="*2${data_type}PING\r\n\x24${message_length}\r\n${1}\r\n"
fi

# This is for debugging purposes, to see what we are sending to the server
# use -e flag to process \r\n
echo $command

printf $command | nc -C -N -v 127.0.0.1 6379
