#!/bin/bash

# install.sh - Install system_update binary

set -e # Exit on error

echo "System Update Installer"
echo "======================="
echo

# Check if Rust is installed
if ! command -v rustc &>/dev/null; then
  echo "Error: Rust is not installed."
  echo "Install Rust from https://rustup.rs/ with:"
  echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
  exit 1
fi

# Check if running as root
if [ "$EUID" -ne 0 ]; then
  echo "Error: This script must be run as root (use sudo)"
  exit 1
fi

# Compile the binary
echo "Compiling system_update..."
rustc -O system_update.rs -o system_update

if [ $? -ne 0 ]; then
  echo "Error: Compilation failed"
  exit 1
fi

echo "✓ Compilation successful"

# Install to /usr/local/bin
echo "Installing to /usr/local/bin..."
cp system_update /usr/local/bin/system_update
chmod 755 /usr/local/bin/system_update

echo "✓ Installation complete!"
echo
echo "You can now run 'sudo system_update' from anywhere"
echo

# Clean up
rm -f system_update
echo "Cleaned up temporary files"
