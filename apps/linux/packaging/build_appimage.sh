#!/bin/bash
# Build AppImage for OpenClaw Linux Desktop App
set -e APPDIR="openclaw-linux.AppDir"

# Build Rust GTK app
cd ../src
cargo build --release
cd ..

# Prepare AppDir structure
mkdir -p $APPDIR/usr/bin
cp src/target/release/openclaw-linux $APPDIR/usr/bin/
# TODO: Add desktop file, icon, dependencies
# Example:
# cp openclaw-linux.desktop $APPDIR/
# cp openclaw-linux.png $APPDIR/
# linuxdeploy --appdir $APPDIR --output appimage
# Copy additional runtime dependencies if needed

# Build AppImage
appimagetool $APPDIR
