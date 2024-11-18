#!/usr/bin/env bash

set -e

echo "Ensure database is running..."
docker-compose up db -d

export DATABASE_URL="postgres://pgdev:pgdev@localhost:5432/dev?sslmode=disable"

echo "Start server in live-run mode..."
cargo run live-run