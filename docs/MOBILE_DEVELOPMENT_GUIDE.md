# Mobile Development Guide: Musializer-RS (Android & iOS)

This guide explains how to build, run, and develop the **Musializer-RS Mobile Application** across Android and iOS.

---

## 🛠️ Prerequisites

1. **Flutter SDK** (`3.7.0+`):
   - Ensure `flutter` is on your `$PATH`.
2. **Rust & Cargo**:
   - Ensure `cargo` is installed.
3. **Cargo Tools**:
   ```bash
   cargo install cargo-ndk flutter_rust_bridge_codegen
   ```

---

## 🤖 Android Setup & Build

### 1. Install Android Rust Targets
```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
```

### 2. Configure Environment Variables
Set your Android SDK and NDK paths in your shell config (`~/.config/fish/config.fish` or `~/.bashrc`):
```bash
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/30.0.16138531"  # Or your NDK version
export PATH="$ANDROID_HOME/platform-tools:$PATH"
```

### 3. Build & Run
- **1-Command Release APK Build**:
  ```bash
  ./build-mobile.sh
  ```
- **Run on connected Android Device or Emulator**:
  ```bash
  # Step A: Compile core libraries
  (cd crates/musializer-core && ./build-android.sh)

  # Step B: Launch Flutter
  cd mobile
  flutter run
  ```

---

## 🍎 iOS Setup & Build (macOS / Xcode)

### 1. Install iOS Rust Targets
```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

### 2. Build & Run
```bash
# Step A: Compile core libraries for iOS
(cd crates/musializer-core && ./build-ios.sh)

# Step B: Launch Flutter on iOS Simulator or Device
cd mobile
flutter run -d ios
```

---

## 🔄 Regenerating Flutter-Rust Bridge (FRB) Bindings

If you add new functions to `crates/musializer-core/src/api.rs`:

```bash
flutter_rust_bridge_codegen generate \
  --rust-root crates/musializer-core \
  --rust-input "crate::api" \
  --dart-root mobile \
  --dart-output mobile/lib/src/rust
```

---

## 🎨 Adding New Visualizer Modes & Themes

1. **New Visualizer Mode**:
   - Add a case in `mobile/lib/models/visualizer_mode.dart`.
   - Create a `CustomPainter` in `mobile/lib/painters/`.
   - Wire it in `_buildPainter()` inside `mobile/lib/main.dart`.
2. **New Color Theme**:
   - Add a preset in `VisualizerTheme.presets` inside `mobile/lib/models/visualizer_theme.dart`.
