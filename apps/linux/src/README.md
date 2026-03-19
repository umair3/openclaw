# OpenClaw Linux App Source

This directory contains the source code for the OpenClaw Linux desktop application.

## Modules
- `main.rs` — Entry point
- `mod.rs` — Module declarations
- `ui.rs` — UI layer (GTK)
- `gateway_client.rs` — Gateway integration
- `plugin_manager.rs` — Plugin management
- `config_manager.rs` — Config editor/validator
- `logs_module.rs` — Logs viewer
- `ipc_layer.rs` — IPC between UI/backend

## Build
Run `cargo build --release` in this directory.

---

See parent README for full project details.
