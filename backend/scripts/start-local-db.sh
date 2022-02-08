#!/usr/bin/env bash
set -x
set -eo pipefail

ROOT=$(git rev-parse --show-toplevel)

# Must match stop-local-db.sh
DB_CONTAINER_NAME=${POSTGRES_CONTAINER_NAME:=editordb}

# Must match `config.toml` in order for database to connect
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=editor}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Creates instance of standard postgres container. Can connect with:
#   $ psql postgresql://postgres:password@localhost:5432/postgres
# [See the docs](https://hub.docker.com/_/postgres) for more info.
docker run \
    --name ${POSTGRES_CONTAINER_NAME} \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -v "${ROOT}/backend/scripts/sql:/docker-entrypoint-initdb.d" \
    -p "${DB_PORT}":5432 \
    -d postgres