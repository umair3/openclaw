#!/bin/bash
# Build .deb package for OpenClaw Linux Desktop App
set -e
DEBDIR="openclaw-linux-deb"

# Build Rust GTK app
cd ../src
cargo build --release
cd ..

# Prepare deb structure
mkdir -p $DEBDIR/usr/bin
cp src/target/release/openclaw-linux $DEBDIR/usr/bin/
# TODO: Add control file, desktop file, icon
# Example:
# cp openclaw-linux.desktop $DEBDIR/usr/share/applications/
# cp openclaw-linux.png $DEBDIR/usr/share/icons/
# Add control file to $DEBDIR/DEBIAN/control
# Copy additional runtime dependencies if needed

# Build .deb
fakeroot dpkg-deb --build $DEBDIR
