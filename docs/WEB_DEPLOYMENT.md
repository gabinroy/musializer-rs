# Musializer-RS: WebAssembly (WASM) Deployment Guide

Musializer-RS runs directly inside modern web browsers with hardware-accelerated WebGPU/WebGL rendering via WebAssembly and `eframe`.

---

## 🌐 1. Prerequisites

1. **Install Trunk** (WASM web bundler for Rust):
   ```bash
   cargo install trunk
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
- `musializer-rs-<hash>.wasm`
- `musializer-rs-<hash>.js`

Deploy the entire `dist/` folder to any static web host.

---

## 📱 Mobile Web & Touch Features

- **Touch & Responsive**: Scales dynamically to mobile screens, touch scrubbers, and portrait viewports.
- **Audio Uploads**: Users can tap **"Load Audio"** or drag-and-drop audio files directly into the browser canvas to decode and visualize in real-time.
