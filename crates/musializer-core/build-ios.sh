#!/usr/bin/env bash
set -e

echo "🍎 Building musializer-core universal XCFramework / static libraries for iOS..."

# Ensure target toolchains are added
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios 2>/dev/null || true

# Determine cargo target directory (handles monorepo root target dir)
TARGET_DIR="../../target"
if [ ! -d "$TARGET_DIR" ]; then
  TARGET_DIR="target"
fi

# 1. Build Physical Device (aarch64-apple-ios)
echo "Building iOS physical device target (aarch64)..."
cargo build --target aarch64-apple-ios --release

# 2. Build Simulators (aarch64 Apple Silicon simulator + x86_64 Intel simulator)
echo "Building iOS simulator targets..."
cargo build --target aarch64-apple-ios-sim --release
cargo build --target x86_64-apple-ios --release

# Locate device and simulator static libraries
DEVICE_LIB=""
SIM_ARM64=""
SIM_X86=""

if [ -f "$TARGET_DIR/aarch64-apple-ios/release/libmusializer_core.a" ]; then
  DEVICE_LIB="$TARGET_DIR/aarch64-apple-ios/release/libmusializer_core.a"
elif [ -f "target/aarch64-apple-ios/release/libmusializer_core.a" ]; then
  DEVICE_LIB="target/aarch64-apple-ios/release/libmusializer_core.a"
fi

if [ -f "$TARGET_DIR/aarch64-apple-ios-sim/release/libmusializer_core.a" ]; then
  SIM_ARM64="$TARGET_DIR/aarch64-apple-ios-sim/release/libmusializer_core.a"
elif [ -f "target/aarch64-apple-ios-sim/release/libmusializer_core.a" ]; then
  SIM_ARM64="target/aarch64-apple-ios-sim/release/libmusializer_core.a"
fi

if [ -f "$TARGET_DIR/x86_64-apple-ios/release/libmusializer_core.a" ]; then
  SIM_X86="$TARGET_DIR/x86_64-apple-ios/release/libmusializer_core.a"
elif [ -f "target/x86_64-apple-ios/release/libmusializer_core.a" ]; then
  SIM_X86="target/x86_64-apple-ios/release/libmusializer_core.a"
fi

# 3. Create universal simulator static library using lipo
mkdir -p "$TARGET_DIR/universal-sim-ios/release"
lipo -create \
  "$SIM_ARM64" \
  "$SIM_X86" \
  -output "$TARGET_DIR/universal-sim-ios/release/libmusializer_core.a"

# 4. Copy libraries to Flutter iOS framework / runner paths
mkdir -p ../../mobile/ios/libs/device
mkdir -p ../../mobile/ios/libs/simulator

cp "$DEVICE_LIB" ../../mobile/ios/libs/device/
cp "$TARGET_DIR/universal-sim-ios/release/libmusializer_core.a" ../../mobile/ios/libs/simulator/

echo "✅ musializer-core static libraries compiled and copied for iOS Device & Simulators!"
