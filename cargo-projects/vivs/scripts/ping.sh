#!/bin/bash

printf '*1\r\n\x244\r\nPING\r\n\r\n' | nc -C -N 127.0.0.1 6379
