# Musializer-RS: WebAssembly (WASM) Deployment Guide

> [!WARNING]
> **Status: Experimental / Work-In-Progress (WIP)**
> WebAssembly (WASM) support in the browser is currently experimental. While the WebAssembly bundle compiles with `trunk`, web browsers enforce strict security and sandbox limitations (such as Web Audio autoplay restrictions, single-threaded browser runtime, and sandboxed file system access) that may cause playback or initialization inconsistencies.
> 
> **For the full, high-performance, and feature-complete experience (including low-latency audio playback and offline 60 FPS video export), we recommend running the Native Desktop app (`cargo run --release`) or Mobile app.**

---

## ⚠️ Known Web (WASM) Limitations

1. **Browser Audio Autoplay Policies**:
   * Modern browsers require an explicit user gesture (click/tap) before allowing `AudioContext` to start.
2. **Single-Threaded Execution**:
   * Native desktop and mobile platforms use background worker threads for zero-latency audio streaming and DSP Fourier analysis. In the browser, threading requires Web Workers and SharedArrayBuffer with specialized HTTP cross-origin isolation headers.
3. **No Direct Offline Video Export**:
   * Direct FFmpeg piping for offline `.mp4` rendering is only supported on native desktop platforms.
4. **File System Sandboxing**:
   * Native file dialogs (`rfd`) and direct filesystem paths (`PathBuf`) are sandboxed in the browser environment.

## 🌐 1. Prerequisites

1. **Install Trunk** (WASM web bundler for Rust):
   - *Option A (Recommended pre-built binary - instant & avoids build conflicts)*:
     ```bash
     # Linux / macOS:
     curl -L --proto '=https' --tlsv1.2 -sSf https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz -C ~/.cargo/bin
     ```
   - *Option B (Via Cargo with locked dependencies)*:
     ```bash
     cargo install trunk --locked
     ```

2. **Add WASM Target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

---

## 🚀 2. Local Development & Testing

Run the live-reloading dev server:
```bash
trunk serve
```
Open your browser at `http://127.0.0.1:8080`.

---

## 📦 3. Production Release Build

To produce optimized static assets for deployment to GitHub Pages, Netlify, Cloudflare Pages, or Vercel:

```bash
trunk build --release
```

The output files are generated in the `dist/` directory:
- `index.html`
- `musializer_rs_bg.wasm`
- `musializer_rs.js`
- `assets/`

Deploy the entire `dist/` folder to any static web host.

---

## 🔧 4. Troubleshooting Guide

### Issue 1: `Address already in use (os error 98)`
**Cause**: Another process or a previous `trunk serve` instance is already running on port `8080`.

**Solutions**:
- **Option 1**: Specify a custom port when serving:
  ```bash
  trunk serve --port 8081
  ```
- **Option 2**: Free port `8080` by terminating the occupying process:
  ```bash
  # Find and kill the process using port 8080:
  fuser -k 8080/tcp
  # Or:
  kill $(lsof -t -i:8080)
  ```

---

### Issue 2: `trunk: command not found`
**Cause**: Cargo's binary directory (`~/.cargo/bin`) is not in your current shell's `$PATH`.

**Solutions**:
- **Bash / Zsh**:
  ```bash
  export PATH="$HOME/.cargo/bin:$PATH"
  ```
- **Fish Shell**:
  ```fish
  fish_add_path ~/.cargo/bin
  ```

---

### Issue 3: Compilation Errors When Installing Trunk via Cargo
**Cause**: Compiling Trunk from source can sometimes encounter transitive dependency version mismatches.

**Solution**:
Always install the official pre-built binary (Option A above) or use the `--locked` flag:
```bash
cargo install trunk --locked
```

---

## 📱 5. Mobile Web & Touch Features

- **Touch & Responsive**: Scales dynamically to mobile screens, touch scrubbers, and portrait viewports.
- **Audio Uploads**: Users can tap **"Load Audio"** or drag-and-drop audio files directly into the browser canvas to decode and visualize in real-time.
