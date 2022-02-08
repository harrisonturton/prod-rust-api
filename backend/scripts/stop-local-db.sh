#!/usr/bin/env bash
set -x
set -eo pipefail

# Must match start-local-db.sh
DB_CONTAINER_NAME=${POSTGRES_CONTAINER_NAME:=editordb}

docker stop ${DB_CONTAINER_NAME}
docker rm ${DB_CONTAINER_NAME}
docker image prune