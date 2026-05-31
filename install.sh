#!/bin/bash
# install.sh - Build ntc from source after git clone
# Requires: Rust (cargo) installed

set -e

echo "🔨 Building ntc from source..."

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build release binary
cargo build --release

# Detect OS for installation path
OS=$(uname -s)
case "$OS" in
    Linux)
        if command -v termux-info &> /dev/null; then
            # Termux (Android)
            INSTALL_DIR="$PREFIX/bin"
            cp target/release/ntc "$INSTALL_DIR/"
            chmod +x "$INSTALL_DIR/ntc"
            echo "✅ Installed to $INSTALL_DIR/ntc"
        else
            # Regular Linux
            echo "📁 Installing to /usr/local/bin/ (requires sudo)..."
            sudo cp target/release/ntc /usr/local/bin/
            echo "✅ Installed to /usr/local/bin/ntc"
        fi
        ;;
    Darwin)
        # macOS
        echo "📁 Installing to /usr/local/bin/ (requires sudo)..."
        sudo cp target/release/ntc /usr/local/bin/
        echo "✅ Installed to /usr/local/bin/ntc"
        ;;
    *)
        echo "⚠️ Unknown OS: $OS"
        echo "Binary built at: target/release/ntc"
        echo "Copy it manually to your PATH."
        exit 0
        ;;
esac

echo ""
ntc --version
echo ""
echo "🎉 ntc installed successfully! Run 'ntc' to start."