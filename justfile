export ROOT_DIR := justfile_directory()

note:
  # Yarn add package root
  echo "yarn -W add <package-name> -D"
  # Yarn add package to workspace member
  echo "yarn workspace <member-package> add <package-name> -D"

setup-install-js:
  yarn

setup-init-android app:
  cd {{app}} && cargo-tauri android init

setup-init-ios app:
  cd {{app}} && cargo-tauri ios init

setup-init-docker-sdk:
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{ROOT_DIR}}/docker/
  docker compose -f docker-compose.yaml up -d

dev-desktop app:
  cd {{app}} && cargo-tauri dev

dev-android app:
  cd {{app}} && cargo-tauri android dev

dev-ios app:
  cd {{app}} && cargo-tauri ios dev

build-desktop app:
  cd {{app}} && cargo-tauri build

dev-fe app:
  cd $ROOT_DIR/www/packages/{{app}} && yarn dev

build-fe app:
  cd $ROOT_DIR/www/packages/{{app}} && yarn build

format:
  cargo fmt && just format-fe

format-fe:
  cd $ROOT_DIR/www && yarn format

lint-fe:
  cd $ROOT_DIR/www && yarn lint

cargo-udeps:
  cargo +nightly udeps --workspace --all-targets
