#!/bin/bash
# Build Flatpak for OpenClaw Linux Desktop App
set -e

# Build Rust GTK app
cd ../src
cargo build --release
cd ..

# TODO: Create flatpak manifest, desktop file, icon
# Example:
# cp openclaw-linux.desktop .
# cp openclaw-linux.png .
# flatpak-builder --force-clean build-dir openclaw-linux-flatpak.json
# Copy additional runtime dependencies if needed

# Build Flatpak (example, requires flatpak-builder)
# flatpak-builder --force-clean build-dir openclaw-linux-flatpak.json openclaw-linux-flatpak.json
