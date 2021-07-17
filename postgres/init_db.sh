#! /bin/bash

source "$(dirname "$0")/.env"
initdb --locale "$LANG" -E UTF8
echo "ALTER USER postgres PASSWORD 'myPassword'" | sudo -u postgres psql
