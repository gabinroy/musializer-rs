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

# 2. Compile all Markdown Guides in docs/ into standalone HTML with premium dark theme
echo "==> Converting Markdown guides to HTML using styled rustdoc template..."
mkdir -p "${GUIDES_DIR}"
TEMPLATES_DIR="${DOCS_DIR}/.templates"
mkdir -p "${TEMPLATES_DIR}"

cat << 'EOF' > "${TEMPLATES_DIR}/header.html"
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js" onload="renderMathInElement(document.body, {delimiters: [{left: '$$', right: '$$', display: true}, {left: '$', right: '$', display: false}]});"></script>
<style>
  :root {
    --bg-color: #0d1117;
    --card-bg: #161b22;
    --card-border: #30363d;
    --text-main: #c9d1d9;
    --text-heading: #f0f6fc;
    --text-muted: #8b949e;
    --accent-cyan: #00f0ff;
    --accent-magenta: #ff007f;
    --accent-blue: #58a6ff;
    --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    --font-mono: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
  }

  * { box-sizing: border-box; }

  body {
    background-color: var(--bg-color) !important;
    color: var(--text-main) !important;
    font-family: var(--font-sans) !important;
    line-height: 1.75 !important;
    margin: 0 !important;
    padding: 0 !important;
    font-size: 16px !important;
    -webkit-font-smoothing: antialiased;
  }

  /* Headings & Anchors */
  .doc-anchor {
    display: none !important;
  }

  h1, h2, h3, h4, h5, h6 {
    color: var(--text-heading) !important;
    font-family: var(--font-sans) !important;
    font-weight: 700 !important;
    margin-top: 2.25rem !important;
    margin-bottom: 0.85rem !important;
    line-height: 1.35 !important;
  }

  h1 {
    font-size: 2.2rem !important;
    border-bottom: 1px solid var(--card-border) !important;
    padding-bottom: 0.75rem !important;
    background: linear-gradient(135deg, var(--accent-cyan) 0%, var(--accent-magenta) 100%) !important;
    -webkit-background-clip: text !important;
    -webkit-text-fill-color: transparent !important;
  }

  h2 {
    font-size: 1.5rem !important;
    border-bottom: 1px solid rgba(48, 54, 61, 0.6) !important;
    padding-bottom: 0.5rem !important;
  }

  h3 { font-size: 1.25rem !important; }

  /* Links */
  a {
    color: var(--accent-blue) !important;
    text-decoration: none !important;
    transition: color 0.15s ease;
  }
  a:hover {
    text-decoration: underline !important;
    color: var(--accent-cyan) !important;
  }

  /* Paragraphs & Lists */
  p, ul, ol {
    margin-bottom: 1.25rem !important;
  }
  ul, ol {
    padding-left: 1.85rem !important;
  }
  li {
    margin-bottom: 0.4rem !important;
  }

  /* Inline Code */
  code {
    font-family: var(--font-mono) !important;
    background: rgba(110, 118, 129, 0.2) !important;
    color: #e6edf3 !important;
    padding: 0.2em 0.45em !important;
    border-radius: 6px !important;
    font-size: 85% !important;
  }

  /* Code Blocks */
  .example-wrap {
    margin: 1.25rem 0 !important;
  }

  pre {
    background-color: var(--card-bg) !important;
    border: 1px solid var(--card-border) !important;
    border-radius: 8px !important;
    padding: 1.1rem 1.35rem !important;
    overflow-x: auto !important;
    margin: 1.25rem 0 !important;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35) !important;
    line-height: 1.55 !important;
  }

  pre code {
    background: transparent !important;
    padding: 0 !important;
    border-radius: 0 !important;
    font-size: 90% !important;
    color: #f0f6fc !important;
  }

  /* Rustdoc Syntax Highlighting Colors */
  pre .kw { color: #ff7b72 !important; font-weight: 600 !important; }
  pre .string { color: #a5d6ff !important; }
  pre .number { color: #79c0ff !important; }
  pre .comment { color: #8b949e !important; font-style: italic !important; }
  pre .macro, pre .fn { color: #d2a8ff !important; }
  pre .prelude-ty, pre .prelude-val { color: #ffa657 !important; }
  pre .question-mark { color: #ff7b72 !important; font-weight: bold !important; }
  pre .attribute { color: #7ee787 !important; }
  pre .self { color: #ff7b72 !important; }

  /* Tables */
  table {
    width: 100% !important;
    border-collapse: separate !important;
    border-spacing: 0 !important;
    margin: 1.75rem 0 !important;
    border: 1px solid var(--card-border) !important;
    border-radius: 8px !important;
    overflow: hidden !important;
    background-color: var(--card-bg) !important;
  }

  th, td {
    padding: 0.85rem 1.15rem !important;
    border-bottom: 1px solid var(--card-border) !important;
    border-right: 1px solid var(--card-border) !important;
    text-align: left !important;
  }

  th:last-child, td:last-child {
    border-right: none !important;
  }

  tr:last-child td {
    border-bottom: none !important;
  }

  th {
    background-color: rgba(255, 255, 255, 0.05) !important;
    color: var(--text-heading) !important;
    font-weight: 600 !important;
  }

  tr:hover td {
    background-color: rgba(255, 255, 255, 0.02) !important;
  }

  /* Blockquotes & Callouts */
  blockquote {
    border-left: 4px solid var(--accent-cyan) !important;
    padding: 0.85rem 1.35rem !important;
    color: var(--text-muted) !important;
    background: rgba(0, 240, 255, 0.05) !important;
    margin: 1.5rem 0 !important;
    border-radius: 0 8px 8px 0 !important;
  }

  blockquote p {
    margin-bottom: 0 !important;
  }

  hr {
    border: 0 !important;
    height: 1px !important;
    background: var(--card-border) !important;
    margin: 2.5rem 0 !important;
  }
</style>
EOF

cat << 'EOF' > "${TEMPLATES_DIR}/nav.html"
<header style="background: rgba(13, 17, 23, 0.9); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border-bottom: 1px solid #30363d; position: sticky; top: 0; z-index: 1000; padding: 0.85rem 1.5rem;">
  <div style="max-width: 900px; margin: 0 auto; display: flex; align-items: center; justify-content: space-between; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
    <a href="../index.html" style="color: #00f0ff; text-decoration: none; font-weight: 600; font-size: 0.95rem; display: flex; align-items: center; gap: 0.4rem; transition: opacity 0.15s ease;">
      &larr; Back to Documentation Hub
    </a>
    <div style="display: flex; align-items: center; gap: 1rem; font-size: 0.85rem;">
      <span style="background: rgba(0, 240, 255, 0.1); color: #00f0ff; border: 1px solid rgba(0, 240, 255, 0.25); padding: 0.2rem 0.6rem; border-radius: 20px; font-weight: 600; font-size: 0.75rem;">Musializer-RS Guide</span>
      <a href="https://github.com/gabinroy/musializer-rs" target="_blank" rel="noopener noreferrer" style="color: #8b949e; text-decoration: none; font-weight: 500;">GitHub &rarr;</a>
    </div>
  </div>
</header>
<main style="max-width: 900px; margin: 0 auto; padding: 2.5rem 1.5rem 5rem 1.5rem;">
EOF

cat << 'EOF' > "${TEMPLATES_DIR}/footer.html"
</main>
<footer style="border-top: 1px solid #30363d; padding: 2.5rem 1.5rem; text-align: center; color: #8b949e; font-size: 0.85rem; background-color: #0d1117; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;">
  <p>Musializer-RS is licensed under the GPL-3.0-or-later License &bull; <a href="../index.html" style="color: #00f0ff; text-decoration: none;">Documentation Hub</a> &bull; <a href="https://github.com/gabinroy/musializer-rs" style="color: #00f0ff; text-decoration: none;">GitHub Repository</a></p>
</footer>
EOF

for md_file in docs/*.md; do
    if [ -f "$md_file" ]; then
        echo "    Compiling $md_file -> HTML with dark theme"
        rustdoc --markdown-no-toc \
                --html-in-header "${TEMPLATES_DIR}/header.html" \
                --html-before-content "${TEMPLATES_DIR}/nav.html" \
                --html-after-content "${TEMPLATES_DIR}/footer.html" \
                --output "${GUIDES_DIR}" "$md_file"
    fi
done

rm -rf "${TEMPLATES_DIR}"

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

      <h2 class="section-title">📖 Architecture & Guides</h2>
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
