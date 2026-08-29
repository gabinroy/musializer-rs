#!/usr/bin/env bash
set -e

echo "🚀 Musializer-RS Mobile Build Automation"
echo "======================================="

# Check environment variables
if [ -z "$ANDROID_HOME" ]; then
    echo "⚠️  ANDROID_HOME not set. Setting default: $HOME/Android/Sdk"
    export ANDROID_HOME="$HOME/Android/Sdk"
fi

if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "⚠️  ANDROID_NDK_HOME not set. Setting default: $ANDROID_HOME/ndk/30.0.16138531"
    export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/30.0.16138531"
fi

export PATH="$HOME/development/flutter/bin:$HOME/.cargo/bin:$ANDROID_HOME/platform-tools:$PATH"

# 1. Build Rust core libraries for Android
echo ""
echo "📦 Step 1: Compiling Rust Core with cargo-ndk..."
(cd crates/musializer-core && ./build-android.sh)

# 2. Build Release APKs
echo ""
echo "📱 Step 2: Building Flutter Release APKs..."
cd mobile
flutter build apk --release --split-per-abi

echo ""
echo "🎉 Build Complete!"
echo "APKs generated at:"
ls -lh build/app/outputs/flutter-apk/app-*.apk
