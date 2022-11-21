#!/usr/bin/env bash

# print out command args during execution
set -x
# -e  - exit if script errors during execution
# -o pipefail - the return value of a pipeline is the status of the last command that had a non-zero status upon exit
set -eo pipefail

DB_USERNAME="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=bookreview}"
DB_PORT="${POSTGRES_PORT:=5423}"

docker run \
    -e POSTGRES_USER=${DB_USERNAME} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000 # max connections
