ui-prepare:
  cd folderlock-ui && pnpm install

ui:
  cd folderlock-ui && pnpm tauri dev

lib:
  cd folderlock-lib && cargo test
