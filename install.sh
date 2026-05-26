#!/bin/bash
# Quick install script for ntc on Linux
echo "Installing ntc..."
sudo dpkg -i ntc_1.7.0-1_amd64.deb
rm ntc_1.7.0-1_amd64.deb
echo "Done! Run 'ntc' to start"