#!/bin/bash

set -euf -o pipefail

readonly CORE_SERVICE_PORT=6001
readonly CORE_SERVICE_DB_ENDPOINT="localhost"
readonly CORE_SERVICE_DB_PORT=5433
readonly CORE_SERVICE_DB_NAME="core_service"
readonly CORE_SERVICE_DB_USER="postgres"
readonly CORE_SERVICE_DB_PASSWORD="postgres"

readonly ACCOUNT_SERVICE_DB_ENDPOINT="localhost"
readonly ACCOUNT_SERVICE_DB_PORT=5433
readonly ACCOUNT_SERVICE_DB_NAME="account_service"
readonly ACCOUNT_SERVICE_DB_USER="postgres"
readonly ACCOUNT_SERVICE_DB_PASSWORD="postgres"

readonly SEARCH_SERVICE_DB_ENDPOINT="localhost"
readonly SEARCH_SERVICE_DB_PORT=5433
readonly SEARCH_SERVICE_DB_NAME="search_service"
readonly SEARCH_SERVICE_DB_USER="postgres"
readonly SEARCH_SERVICE_DB_PASSWORD="postgres"

RUST_LOG=info cargo run --bin core_service_main -- \
  --port $CORE_SERVICE_PORT \
  --db-endpoint $CORE_SERVICE_DB_ENDPOINT \
  --db-port $CORE_SERVICE_DB_PORT \
  --db-name $CORE_SERVICE_DB_NAME \
  --db-user $CORE_SERVICE_DB_USER \
  --db-password $CORE_SERVICE_DB_PASSWORD \
  --acc-db-endpoint $ACCOUNT_SERVICE_DB_ENDPOINT \
  --acc-db-port $ACCOUNT_SERVICE_DB_PORT \
  --acc-db-name $ACCOUNT_SERVICE_DB_NAME \
  --acc-db-user $ACCOUNT_SERVICE_DB_USER \
  --acc-db-password $ACCOUNT_SERVICE_DB_PASSWORD \
  --sea-db-endpoint $SEARCH_SERVICE_DB_ENDPOINT \
  --sea-db-port $SEARCH_SERVICE_DB_PORT \
  --sea-db-name $SEARCH_SERVICE_DB_NAME \
  --sea-db-user $SEARCH_SERVICE_DB_USER \
  --sea-db-password $SEARCH_SERVICE_DB_PASSWORD \
  --config-file core_service/main/config/Local.dhall \
