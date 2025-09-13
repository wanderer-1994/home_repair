export ROOT_DIR := justfile_directory()

setup-install-js:
  yarn

setup-init-android:
  yarn tauri android init

setup-init-ios:
  yarn tauri ios init

dev-desktop:
  yarn tauri dev

dev-android:
  yarn tauri android dev

dev-ios:
  yarn tauri ios dev
