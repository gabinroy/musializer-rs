#!/usr/bin/env bash
set -e

echo "🍎 Building musializer-core for iOS (Simulator + Physical Devices)..."

# iOS Device target
cargo build --target aarch64-apple-ios --release

# iOS Simulator targets (Apple Silicon + Intel Mac)
cargo build --target aarch64-apple-ios-sim --release
cargo build --target x86_64-apple-ios --release

echo "✅ musializer-core static libraries compiled for iOS!"
