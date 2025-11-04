#!/usr/bin/env bash
# package_cmdcreate.sh - Build and package cmdcreate binary
#
# This script builds cmdcreate in release mode and packages the binary
# with a versioned filename for distribution or backup.
#
# Usage:
#   ./package_cmdcreate.sh <version>
# Example:
#   ./package_cmdcreate.sh 0.7.1

# Check if version argument is provided
if [[ -z "$1" ]]; then
    echo "Provide cmdcreate's version"
    exit 1
fi

VERSION="$1"

# Update Rust crate dependencies
cargo update

# Update Rust toolchain
rustup update

# Build cmdcreate in release mode
cargo build --release

# Copy the compiled binary to Downloads for packaging
cp ../target/release/cmdcreate ~/Downloads

# Rename binary to include version info for easier tracking/distribution
mv ~/Downloads/cmdcreate ~/Downloads/cmdcreate-"$VERSION"-linux-x86_64-bin

# Notify user that packaging is complete
echo -e "\nPackaged cmdcreate to \"cmdcreate-v$VERSION-linux-x86_64-bin\""
