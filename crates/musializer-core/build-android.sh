#!/usr/bin/env bash
set -e

NDK_PLATFORM=26

mkdir -p ../../target/android-libs/arm64-v8a
mkdir -p ../../target/android-libs/armeabi-v7a
mkdir -p ../../target/android-libs/x86_64

mkdir -p ../../mobile/android/app/src/main/jniLibs/arm64-v8a
mkdir -p ../../mobile/android/app/src/main/jniLibs/armeabi-v7a
mkdir -p ../../mobile/android/app/src/main/jniLibs/x86_64

echo "🔨 Cross-compiling musializer-core for Android (API Level $NDK_PLATFORM)..."

# 1. ARM64
cargo ndk -t aarch64-linux-android --platform $NDK_PLATFORM build --release
cp ../../target/aarch64-linux-android/release/libmusializer_core.so ../../target/android-libs/arm64-v8a/
cp ../../target/aarch64-linux-android/release/libmusializer_core.so ../../mobile/android/app/src/main/jniLibs/arm64-v8a/

# 2. ARMv7
cargo ndk -t armv7-linux-androideabi --platform $NDK_PLATFORM build --release
cp ../../target/armv7-linux-androideabi/release/libmusializer_core.so ../../target/android-libs/armeabi-v7a/
cp ../../target/armv7-linux-androideabi/release/libmusializer_core.so ../../mobile/android/app/src/main/jniLibs/armeabi-v7a/

# 3. x86_64
cargo ndk -t x86_64-linux-android --platform $NDK_PLATFORM build --release
cp ../../target/x86_64-linux-android/release/libmusializer_core.so ../../target/android-libs/x86_64/
cp ../../target/x86_64-linux-android/release/libmusializer_core.so ../../mobile/android/app/src/main/jniLibs/x86_64/

echo "✅ Android .so libraries generated in target/android-libs and mobile jniLibs!"
