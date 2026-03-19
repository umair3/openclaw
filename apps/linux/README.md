# OpenClaw Linux Desktop App

This is the Linux desktop application for OpenClaw, built with native GTK.

## Features
For full requirements, see [Linux.md](Linux.md).

---

All work is done in `apps/linux`.

## Working Directory
- Node.js (for gateway/plugin management)
- GTK 4
- Linux (Ubuntu, Arch, Fedora)

## Requirements
- `packaging/` — Scripts for AppImage, .deb, Flatpak
- `src/` — Main application source code

## Directory Structure
See below for build and packaging steps.

## Build Instructions
### Building the Linux App (Rust/Cargo)

The app requires Rust nightly (edition2024) and Cargo with lockfile v4 support.

#### Prerequisites
1. Install rustup (if not already installed):
	sudo apt install rustup
2. Install and activate the nightly toolchain:
	rustup install nightly && rustup default nightly
3. Ensure Cargo is available (comes with rustup):
	cargo --version

#### Build Command
From the repo root:

	cargo build --release --manifest-path apps/linux/src/Cargo.toml -Znext-lockfile-bump

The built binary will be in:
	apps/linux/src/target/release/

---
For packaging (AppImage, .deb, Flatpak), see the packaging/ directory.
