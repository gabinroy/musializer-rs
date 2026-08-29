#!/usr/bin/env bash
set -e

echo "🍎 Building musializer-core universal XCFramework / static libraries for iOS..."

# Ensure target toolchains are added
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios 2>/dev/null || true

# 1. Build Physical Device (aarch64-apple-ios)
echo "Building iOS physical device target (aarch64)..."
cargo build --target aarch64-apple-ios --release

# 2. Build Simulators (aarch64 Apple Silicon simulator + x86_64 Intel simulator)
echo "Building iOS simulator targets..."
cargo build --target aarch64-apple-ios-sim --release
cargo build --target x86_64-apple-ios --release

# 3. Create universal simulator static library using lipo
mkdir -p target/universal-sim-ios/release
lipo -create \
  target/aarch64-apple-ios-sim/release/libmusializer_core.a \
  target/x86_64-apple-ios/release/libmusializer_core.a \
  -output target/universal-sim-ios/release/libmusializer_core.a

# 4. Copy libraries to Flutter iOS framework / runner paths
mkdir -p ../../mobile/ios/libs/device
mkdir -p ../../mobile/ios/libs/simulator

cp target/aarch64-apple-ios/release/libmusializer_core.a ../../mobile/ios/libs/device/
cp target/universal-sim-ios/release/libmusializer_core.a ../../mobile/ios/libs/simulator/

echo "✅ musializer-core static libraries compiled and copied for iOS Device & Simulators!"
