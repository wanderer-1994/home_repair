export ROOT_DIR := justfile_directory()

note:
  # Yarn add package root
  echo "yarn -W add <package-name> -D"
  # Yarn add package to workspace member
  echo "yarn workspace <member-package> add <package-name> -D"

setup-install-js:
  yarn

setup-init-android app:
  # Ref <https://v2.tauri.app/start/prerequisites/#android>
  # 1. Install Android Studio at <https://developer.android.com/studio>
  # 2. Persist env variables JAVA_HOME, ANDROID_HOME, NDK_HOME in `~/.zshrc`` file
  # 3. Add rustup targets: rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
  # 4. Open Android Studio app and install additional, non-default tools
  # Discussion "Should I commit gen folder" <https://github.com/tauri-apps/tauri/discussions/8323>
  cd {{app}} && cargo-tauri android init

setup-init-ios app:
  # Ref <https://v2.tauri.app/start/prerequisites/#ios>
  cd {{app}} && cargo-tauri ios init

setup-init-docker-sdk:
  #!/usr/bin/env bash
  set -euo pipefail
  cd {{ROOT_DIR}}/docker/
  docker compose -f docker-compose.yaml up -d

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
