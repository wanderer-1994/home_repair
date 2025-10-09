export ROOT_DIR := justfile_directory()

note:
  # Yarn add package root
  echo "yarn -W add <package-name> -D"
  # Yarn add package to workspace member
  echo "yarn workspace <member-package> add <package-name> -D"

setup-install-js:
  yarn

setup-init-docker-sdk:
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{ROOT_DIR}}/docker/
  docker compose -f docker-compose.yaml up --build -d

dev-customer-app:
  cd $ROOT_DIR/www/packages/customer_app && yarn start

dev-handyman-app:
  cd $ROOT_DIR/www/packages/handyman_app && yarn start

dev-be:
  $ROOT_DIR/.sh/start_backend_local_dev.sh

format:
  cargo fmt && just format-fe

format-fe:
  cd $ROOT_DIR/www && yarn format

lint-fe:
  cd $ROOT_DIR/www && yarn lint

cargo-udeps:
  cargo +nightly udeps --workspace --all-targets

# Generate graphql schema api
gen-graphql-schema:
  cargo run --bin gen_schema
