export ROOT_DIR := justfile_directory()

setup-install-js:
  yarn

setup-init-android app:
  cd {{app}} && cargo-tauri android init

setup-init-ios app:
  cd {{app}} && cargo-tauri ios init

dev-desktop app:
  cd {{app}} && cargo-tauri dev

dev-android app:
  cd {{app}} && cargo-tauri android dev

dev-ios app:
  cd {{app}} && cargo-tauri ios dev
