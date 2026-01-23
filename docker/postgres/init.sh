#!/bin/bash
set -e

if [ "$POSTGRES_DB" != "testdb" ]; then
    echo "Creating database: testdb"
    psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
        SELECT 'CREATE DATABASE testdb'
        WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'testdb')\gexec
EOSQL
else
    echo "Database testdb matches defaults, skipping creation."
fi
