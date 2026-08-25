# Musializer-RS: Mobile (Android & iOS) Deployment Guide

This guide covers building, packaging, signing, and running **Musializer-RS** natively on Android and iOS devices.

---

## 📱 1. Android Build & APK Packaging

Musializer-RS uses `cargo-apk` and `android-activity` (`NativeActivity`) with direct hardware-accelerated WGPU rendering.

### 1.1 Prerequisites
1. **Android SDK & NDK**:
   - Install Android Studio or Android Command-line Tools (`cmdline-tools`).
   - Install NDK (version `r25c` or newer) and CMake via SDK Manager.
   - Set environment variables:
     ```bash
     export ANDROID_HOME="$HOME/Android/Sdk"
     export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/25.2.9519653" # your NDK version path
     export PATH="$PATH:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools"
     ```

2. **Cargo APK Tools**:
   ```bash
   cargo install cargo-apk cargo-ndk
   ```

3. **Android Rust Targets**:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
   ```

### 1.2 Building the APK
```bash
# Debug APK
cargo apk build

# Release APK (Optimized)
cargo apk build --release
```
The output APK is generated at:
`target/release/apk/Musializer-RS.apk`

### 1.3 Installing & Running on Android Device
```bash
# Connect device with USB Debugging enabled (or start emulator)
adb devices

# Install APK directly
adb install target/release/apk/Musializer-RS.apk

# Launch App
adb shell am start -n com.musializer.app/android.app.NativeActivity
```

---

## 🍏 2. iOS Build & Xcode Project Integration

On iOS, Musializer-RS is compiled as a static C-compatible library (`libmusializer_rs.a`) and linked into an Xcode iOS App container.

### 2.1 Prerequisites
- macOS host with **Xcode 15+** installed.
- iOS Rust targets:
  ```bash
  rustup target add aarch64-apple-ios x86_64-apple-ios
  cargo install cargo-lipo
  ```

### 2.2 Building Universal iOS Library
```bash
# Compile universal iOS binary (Device ARM64 + Simulator x86_64)
cargo lipo --release
```
The compiled universal library is located at:
`target/universal/release/libmusializer_rs.a`

### 2.3 Linking in Xcode
1. Open Xcode and create a new **iOS App** (Swift or Objective-C lifecycle).
2. In Project Settings > **General > Frameworks, Libraries, and Embedded Content**:
   - Add `libmusializer_rs.a`.
   - Add Apple system frameworks:
     - `AudioToolbox.framework`
     - `AVFoundation.framework`
     - `CoreAudio.framework`
     - `Metal.framework`
     - `QuartzCore.framework`
3. Build and Run on your iOS Simulator or physical iPhone/iPad.

---

## 🎧 Mobile Permissions & Audio Behavior

- **Audio Permissions**: Configured in `Cargo.toml` (`android.permission.READ_MEDIA_AUDIO` / `READ_EXTERNAL_STORAGE`).
- **Touch Navigation**:
  - Minimum 44x44 point touch hitboxes for finger accuracy.
  - Responsive layout dynamically switches to portrait stacking when screen width is under 600 points.
- **Audio Routing**:
  - Android: `cpal` automatically routes through native AAudio / OpenSL ES.
  - iOS: `cpal` routes through CoreAudio with low latency.
- **Video Export**:
  - Mobile sandboxes cannot spawn headless FFmpeg subprocesses. On mobile, tapping **"Record Info"** provides instructions on recording high-definition 60 FPS video using iOS Control Center or Android Quick Settings Screen Recorder.
