#!/bin/sh
# init-test-db.sh — provision the Postgres `test` role and `test_db` database.
# Runs once inside the postgres container at first startup (docker-entrypoint-initdb.d).
# This script is only used by docker-compose.test.yml (the test overlay).
set -e
psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
    CREATE ROLE test WITH LOGIN PASSWORD 'test';
    CREATE DATABASE test_db OWNER test;
    GRANT ALL PRIVILEGES ON DATABASE test_db TO test;
EOSQL
