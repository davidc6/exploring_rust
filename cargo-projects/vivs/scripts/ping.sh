#!/bin/bash

# - *1 - data type, number of elements
# - $4 - data type, number of chars in string
# - PING - string which is the actual command

DATA_TYPE='\r\n\x244\r\n'

# For now, any argument can be passed into the script to trigger this conditional
if [ -z $1 ]; then 
  COMMAND="*1${DATA_TYPE}PING\r\n"
else 
  COMMAND="*2${DATA_TYPE}PING\r\n\x243\r\nYES\r\n"
fi

# This is for debugging purposes, to see what we are sending to the server
# use -e flag to process \r\n
echo $COMMAND

printf $COMMAND | nc -C -N 127.0.0.1 6379

