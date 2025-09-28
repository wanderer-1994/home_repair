#!/bin/bash

set -euf -o pipefail

readonly CORE_SERVICE_PORT=6001
readonly SHARE_SERVICE_DB_ENDPOINT="localhost"
readonly SHARE_SERVICE_DB_PORT=5433
readonly SHARE_SERVICE_DB_NAME="home_repair_share_database"
readonly SHARE_SERVICE_DB_USER="postgres"
readonly SHARE_SERVICE_DB_PASSWORD="postgres"

RUST_LOG=info cargo run --bin core_service_main -- \
  --port $CORE_SERVICE_PORT \
  --db-endpoint $SHARE_SERVICE_DB_ENDPOINT \
  --db-port $SHARE_SERVICE_DB_PORT \
  --db-name $SHARE_SERVICE_DB_NAME \
  --db-user $SHARE_SERVICE_DB_USER \
  --db-password $SHARE_SERVICE_DB_PASSWORD \
  --config-file core_service/main/config/Local.dhall \
