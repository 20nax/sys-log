#! /bin/bash

source "$(dirname "$0")/.env"
postgres -k "$PGHOST"
