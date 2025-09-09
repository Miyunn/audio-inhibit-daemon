#!/usr/bin/env bash
set -euo pipefail

SERVICE_NAME="audio-inhibit-daemon"
BIN_NAME="audio-inhibit-daemon"
INSTALL_DIR="$HOME/.local/bin"
SYSTEMD_USER_DIR="$HOME/.config/systemd/user"

echo "==> Uninstalling $SERVICE_NAME ..."

# Stop and disable service
if systemctl --user is-enabled --quiet "$SERVICE_NAME.service"; then
    echo "==> Stopping and disabling systemd service"
    systemctl --user disable --now "$SERVICE_NAME.service"
fi

# Remove systemd unit
if [ -f "$SYSTEMD_USER_DIR/$SERVICE_NAME.service" ]; then
    echo "==> Removing systemd unit file"
    rm -f "$SYSTEMD_USER_DIR/$SERVICE_NAME.service"
    systemctl --user daemon-reload
fi

# Remove binary
if [ -f "$INSTALL_DIR/$BIN_NAME" ]; then
    echo "==> Removing binary from $INSTALL_DIR"
    rm -f "$INSTALL_DIR/$BIN_NAME"
fi

echo "==> Uninstall complete!"
