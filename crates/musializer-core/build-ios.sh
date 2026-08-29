#!/usr/bin/env bash
set -e

echo "🍎 Building musializer-core universal XCFramework / static libraries for iOS..."

# Ensure target toolchains are added
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios 2>/dev/null || true

# Determine absolute workspace root directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# 1. Build Physical Device (aarch64-apple-ios)
echo "Building iOS physical device target (aarch64)..."
cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --target aarch64-apple-ios --release

# 2. Build Simulators (aarch64 Apple Silicon simulator + x86_64 Intel simulator)
echo "Building iOS simulator targets..."
cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --target aarch64-apple-ios-sim --release
cargo build --manifest-path "$SCRIPT_DIR/Cargo.toml" --target x86_64-apple-ios --release

# Find the built static libraries
find_lib() {
  local target="$1"
  if [ -f "$WORKSPACE_ROOT/target/$target/release/libmusializer_core.a" ]; then
    echo "$WORKSPACE_ROOT/target/$target/release/libmusializer_core.a"
  elif [ -f "$SCRIPT_DIR/target/$target/release/libmusializer_core.a" ]; then
    echo "$SCRIPT_DIR/target/$target/release/libmusializer_core.a"
  else
    find "$WORKSPACE_ROOT/target" -name "libmusializer_core.a" -path "*/$target/*" | head -n 1
  fi
}

DEVICE_LIB="$(find_lib "aarch64-apple-ios")"
SIM_ARM64="$(find_lib "aarch64-apple-ios-sim")"
SIM_X86="$(find_lib "x86_64-apple-ios")"

echo "Discovered libraries:"
echo "  Device (aarch64): $DEVICE_LIB"
echo "  Sim (arm64):      $SIM_ARM64"
echo "  Sim (x86_64):     $SIM_X86"

if [ -z "$DEVICE_LIB" ] || [ ! -f "$DEVICE_LIB" ]; then
  echo "❌ Error: Could not locate aarch64-apple-ios library"
  exit 1
fi

if [ -z "$SIM_ARM64" ] || [ ! -f "$SIM_ARM64" ]; then
  echo "❌ Error: Could not locate aarch64-apple-ios-sim library"
  exit 1
fi

if [ -z "$SIM_X86" ] || [ ! -f "$SIM_X86" ]; then
  echo "❌ Error: Could not locate x86_64-apple-ios library"
  exit 1
fi

# 3. Create universal simulator static library using lipo
OUTPUT_SIM_DIR="$WORKSPACE_ROOT/target/universal-sim-ios/release"
mkdir -p "$OUTPUT_SIM_DIR"
lipo -create "$SIM_ARM64" "$SIM_X86" -output "$OUTPUT_SIM_DIR/libmusializer_core.a"

# 4. Copy libraries to Flutter iOS framework / runner paths
mkdir -p "$WORKSPACE_ROOT/mobile/ios/libs/device"
mkdir -p "$WORKSPACE_ROOT/mobile/ios/libs/simulator"

cp "$DEVICE_LIB" "$WORKSPACE_ROOT/mobile/ios/libs/device/"
cp "$OUTPUT_SIM_DIR/libmusializer_core.a" "$WORKSPACE_ROOT/mobile/ios/libs/simulator/"

echo "✅ musializer-core static libraries compiled and copied for iOS Device & Simulators!"
