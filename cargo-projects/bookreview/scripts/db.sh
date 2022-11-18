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

export PGPASSWORD="${DB_PASSWORD}"

# ping postgres endpoint until it's ready
until psql -h "localhost" -U "${DB_USERNAME}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is unavailable"
    sleep 1
done

# now it's ready
>&2 echo "Postgres is up and running on port ${DB_PORT}!"

# env variable needed to be a valid postgres connection string
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

# create database
sqlx database create
