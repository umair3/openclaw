#!/bin/bash
# Automated packaging script for OpenClaw Linux app assets

set -e

# Define paths
BUILD_DIR="$(dirname "$0")/src/target/release"
ASSETS_SRC="$(dirname "$0")/src/assets"
ASSETS_DEST="$BUILD_DIR/assets"

# Create destination directory if it doesn't exist
mkdir -p "$ASSETS_DEST"

# Copy all assets
cp -r "$ASSETS_SRC"/* "$ASSETS_DEST"/

echo "Assets copied to $ASSETS_DEST"
