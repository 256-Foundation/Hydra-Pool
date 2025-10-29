#!/bin/bash
set -e

echo "Building Hydrapool Debian package..."
echo

# Check if cargo-deb is installed
if ! command -v cargo-deb &> /dev/null; then
    echo "cargo-deb not found. Installing..."
    cargo install cargo-deb
fi

# Build the release binaries first
echo "Building release binaries..."
cargo build --release

# Build the Debian package
echo "Creating Debian package..."
cargo deb

echo
echo "Build complete!"
echo "Package location: target/debian/"
ls -lh ../target/debian/*.deb
echo
echo "To install the package:"
echo "  sudo dpkg -i target/debian/hydrapool_*.deb"
echo
echo "After installation:"
echo "  1. Edit /etc/hydrapool/config.toml with your settings"
echo "  2. Start the service: sudo systemctl start hydrapool"
echo "  3. Enable at boot: sudo systemctl enable hydrapool"
echo "  4. Check status: sudo systemctl status hydrapool"
