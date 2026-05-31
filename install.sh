#!/bin/bash
# Quick install script for ntc on Linux (Debian/Ubuntu)
# Version: 1.8.0

set -e  # Exit on error

echo "Installing ntc v1.8.0..."

# Check if running with sudo
if [ "$EUID" -ne 0 ]; then 
    echo "Please run with sudo: sudo ./install.sh"
    exit 1
fi

# Check if .deb file exists
if [ ! -f "ntc_1.8.0-1_amd64.deb" ]; then
    echo "Error: ntc_1.8.0-1_amd64.deb not found!"
    echo "Please download it from GitHub Releases first."
    exit 1
fi

# Install the package
dpkg -i ntc_1.8.0-1_amd64.deb

# Clean up
rm -f ntc_1.8.0-1_amd64.deb

echo ""
echo "✅ ntc 1.8.0 installed successfully!"
echo "Run 'ntc' to start the interactive shell"
echo "Run 'ntc --help' for usage information"