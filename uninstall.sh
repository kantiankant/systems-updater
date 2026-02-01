#!/bin/bash

# uninstall.sh - Uninstall system_update binary

set -e

echo "System Update Uninstaller"
echo "========================="
echo

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo "Error: This script must be run as root (use sudo)"
  exit 1
fi

# Check if binary exists
if [ ! -f /usr/local/bin/system_update ]; then
  echo "Error: system_update is not installed"
  exit 1
fi

# Remove the binary
echo "Removing /usr/local/bin/system_update..."
rm -f /usr/local/bin/system_update

echo "✓ Uninstallation complete!"
