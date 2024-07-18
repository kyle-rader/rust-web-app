#!/usr/bin/env sh

# Load environment variables from .env file
source "$(dirname "$0")/.env"

# Conect via DATABASE_URL
psql $DATABASE_URL