#!/usr/bin/env bash
set -euo pipefail

echo "=============================================="
echo " Building Musializer-RS HTML Documentation"
echo "=============================================="

DOCS_DIR="target/doc"
GUIDES_DIR="${DOCS_DIR}/guides"

# 1. Compile Rust Crate Documentation via cargo doc
echo "==> Generating Rustdoc for workspace crates..."
cargo doc --workspace --no-deps

# 2. Compile all Markdown Guides in docs/ into standalone HTML
echo "==> Converting Markdown guides to HTML using rustdoc..."
mkdir -p "${GUIDES_DIR}"

for md_file in docs/*.md; do
    if [ -f "$md_file" ]; then
        echo "    Compiling $md_file -> HTML"
        rustdoc --output "${GUIDES_DIR}" "$md_file"
    fi
done

# 3. Create Root Landing Page (Documentation Hub)
echo "==> Generating root documentation portal (index.html)..."
cat << 'EOF' > "${DOCS_DIR}/index.html"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Musializer-RS Documentation Hub</title>
  <link rel="icon" type="image/svg+xml" href="static.files/favicon-044be391.svg">
  <style>
    :root {
      --bg-color: #0d1117;
      --card-bg: #161b22;
      --card-border: #30363d;
      --card-hover-border: #58a6ff;
      --text-main: #c9d1d9;
      --text-heading: #f0f6fc;
      --text-muted: #8b949e;
      --accent-cyan: #00f0ff;
      --accent-magenta: #ff007f;
      --accent-purple: #7928ca;
      --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    }

    * { box-sizing: border-box; margin: 0; padding: 0; }

    body {
      background-color: var(--bg-color);
      color: var(--text-main);
      font-family: var(--font-sans);
      line-height: 1.6;
      padding: 2.5rem 1.5rem;
      display: flex;
      justify-content: center;
    }

    .container {
      max-width: 1040px;
      width: 100%;
    }

    header {
      text-align: center;
      margin-bottom: 3.5rem;
      padding-bottom: 2rem;
      border-bottom: 1px solid var(--card-border);
    }

    .brand-title {
      font-size: 2.75rem;
      font-weight: 800;
      color: var(--text-heading);
      letter-spacing: -0.02em;
      margin-bottom: 0.5rem;
      background: linear-gradient(135deg, var(--accent-cyan) 0%, var(--accent-magenta) 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }

    .brand-subtitle {
      font-size: 1.2rem;
      color: var(--text-muted);
      max-width: 680px;
      margin: 0 auto;
    }

    .section-title {
      font-size: 1.4rem;
      font-weight: 700;
      color: var(--text-heading);
      margin-bottom: 1.25rem;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }

    .grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 1.5rem;
      margin-bottom: 3rem;
    }

    .card {
      background-color: var(--card-bg);
      border: 1px solid var(--card-border);
      border-radius: 12px;
      padding: 1.75rem;
      text-decoration: none;
      color: inherit;
      transition: transform 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
      display: flex;
      flex-direction: column;
    }

    .card:hover {
      border-color: var(--accent-cyan);
      transform: translateY(-3px);
      box-shadow: 0 8px 24px rgba(0, 240, 255, 0.12);
    }

    .card-badge {
      display: inline-block;
      align-self: flex-start;
      padding: 0.25rem 0.6rem;
      border-radius: 20px;
      font-size: 0.75rem;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      margin-bottom: 1rem;
      background-color: rgba(0, 240, 255, 0.1);
      color: var(--accent-cyan);
      border: 1px solid rgba(0, 240, 255, 0.25);
    }

    .card-badge.guide {
      background-color: rgba(255, 0, 127, 0.1);
      color: var(--accent-magenta);
      border-color: rgba(255, 0, 127, 0.25);
    }

    .card-title {
      font-size: 1.25rem;
      font-weight: 700;
      color: var(--text-heading);
      margin-bottom: 0.5rem;
    }

    .card-desc {
      font-size: 0.95rem;
      color: var(--text-muted);
      flex-grow: 1;
      margin-bottom: 1.25rem;
    }

    .card-link {
      font-size: 0.9rem;
      font-weight: 600;
      color: var(--accent-cyan);
      display: flex;
      align-items: center;
      gap: 0.4rem;
    }

    footer {
      text-align: center;
      padding-top: 2rem;
      border-top: 1px solid var(--card-border);
      color: var(--text-muted);
      font-size: 0.9rem;
    }

    footer a {
      color: var(--accent-cyan);
      text-decoration: none;
    }
  </style>
</head>
<body>
  <div class="container">
    <header>
      <h1 class="brand-title">Musializer-RS Documentation</h1>
      <p class="brand-subtitle">
        High-Performance Real-Time Audio Visualizer & DSP Engine for Desktop and Mobile.
      </p>
    </header>

    <main>
      <h2 class="section-title">📦 Rust Crate API References</h2>
      <div class="grid">
        <a class="card" href="musializer_core/index.html">
          <span class="card-badge">Crate API</span>
          <h3 class="card-title">musializer-core</h3>
          <p class="card-desc">
            Shared pure-Rust audio decoding engine, Symphonia multi-codec stream pipeline, SIMD FFT processor, logarithmic frequency bands, and zero-copy FFI bridge.
          </p>
          <span class="card-link">View Rustdoc &rarr;</span>
        </a>

        <a class="card" href="musializer_desktop/index.html">
          <span class="card-badge">Crate API</span>
          <h3 class="card-title">musializer-desktop</h3>
          <p class="card-desc">
            Native GUI desktop client built with egui and eframe. Hardware-accelerated canvas rendering, offline FFmpeg video exporting, and GitHub release auto-updater.
          </p>
          <span class="card-link">View Rustdoc &rarr;</span>
        </a>
      </div>

      <h2 class="section-title">📖 Architecture & Guides (HTML)</h2>
      <div class="grid">
        <a class="card" href="guides/CORE_API_REFERENCE.html">
          <span class="card-badge guide">Guide</span>
          <h3 class="card-title">Core API & Mathematics Reference</h3>
          <p class="card-desc">
            Deep dive into DSP algorithms, Hann windowing math, logarithmic frequency partitioning, treble compensation curves, and code examples.
          </p>
          <span class="card-link">Read Guide &rarr;</span>
        </a>

        <a class="card" href="guides/ARCHITECTURE.html">
          <span class="card-badge guide">Guide</span>
          <h3 class="card-title">Architecture Documentation</h3>
          <p class="card-desc">
            End-to-end workspace architecture, thread synchronization with atomic pointers, audio buffer data flow, and Flutter-Rust FFI pipeline.
          </p>
          <span class="card-link">Read Guide &rarr;</span>
        </a>

        <a class="card" href="guides/MOBILE_DEVELOPMENT_GUIDE.html">
          <span class="card-badge guide">Guide</span>
          <h3 class="card-title">Mobile Development Guide</h3>
          <p class="card-desc">
            Setup, NDK cross-compilation for Android (aarch64, armv7, x86_64), iOS static frameworks, and Flutter app execution.
          </p>
          <span class="card-link">Read Guide &rarr;</span>
        </a>

        <a class="card" href="guides/APP_EXPORTING_COMMANDS.html">
          <span class="card-badge guide">Guide</span>
          <h3 class="card-title">App Exporting Commands</h3>
          <p class="card-desc">
            FFmpeg CLI parameters, lossless video pipeline specifications, and offline rendering mechanics.
          </p>
          <span class="card-link">Read Guide &rarr;</span>
        </a>

        <a class="card" href="guides/PACKAGE_DISTRIBUTION_GUIDE.html">
          <span class="card-badge guide">Guide</span>
          <h3 class="card-title">Package Distribution Guide</h3>
          <p class="card-desc">
            Packaging instructions for Arch Linux (AUR), Flatpak, Debian (.deb), Fedora (.rpm), AppImage, Windows, and macOS bundles.
          </p>
          <span class="card-link">Read Guide &rarr;</span>
        </a>
      </div>
    </main>

    <footer>
      <p>Musializer-RS is licensed under the GPL-3.0-or-later License &bull; <a href="https://github.com/gabinroy/musializer-rs">GitHub Repository</a></p>
    </footer>
  </div>
</body>
</html>
EOF

echo "==> Documentation build complete!"
echo "    Main portal: ${DOCS_DIR}/index.html"
