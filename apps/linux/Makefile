# Makefile for OpenClaw Linux app build and asset packaging

.PHONY: all build package

all: build package

build:
	cargo build --release --manifest-path src/Cargo.toml

package:
	bash package-assets.sh

# Usage:
# make         # builds and packages assets
# make build   # only builds
# make package # only packages assets
