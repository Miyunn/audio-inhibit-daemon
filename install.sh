#!/usr/bin/env bash
set -euo pipefail

SERVICE_NAME="audio-inhibit-daemon"
BIN_NAME="audio-inhibit-daemon"
INSTALL_DIR="$HOME/.local/bin"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"

echo "==> Installing $SERVICE_NAME ..."

# Check for Rust
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: Rust (cargo) is not installed."
    echo "Please install Rust first: https://www.rust-lang.org/tools/install"
    exit 1
fi

# Build release binary
echo "==> Building release binary..."
cargo build --release

# Ensure install dir exists
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "==> Copying binary to $INSTALL_DIR"
cp "target/release/$BIN_NAME" "$INSTALL_DIR/"

# Ensure systemd user dir exists
mkdir -p "$SYSTEMD_USER_DIR"

# Copy service file
if [ -f "$SERVICE_NAME.service" ]; then
    echo "==> Installing systemd service file"
    cp "$SERVICE_NAME.service" "$SYSTEMD_USER_DIR/"
else
    echo "Error: $SERVICE_NAME.service not found in repo."
    exit 1
fi

# Reload systemd and enable service
echo "==> Reloading systemd and enabling service"
systemctl --user daemon-reload
systemctl --user enable --now "$SERVICE_NAME.service"

echo "==> Installation complete!"
echo "The service is now running. Check status with:"
echo "    systemctl --user status $SERVICE_NAME.service"
