#!/bin/bash

set -e

echo "[1/4] Building for WebAssembly..."
cargo build --target wasm32-unknown-unknown --release

echo "[2/4] Preparing docs/ folder..."
mkdir -p docs/

echo "[3/4] Copying build output..."
# Copy the generated WASM file (make sure the name matches your crate)
cp target/wasm32-unknown-unknown/release/website.wasm docs/

# Copy static frontend files
cp static/index.html docs/
cp static/game.js docs/
cp static/style.css docs/

# Optional: Copy assets if needed by your loader (not required by macroquad unless you custom-load them)
# cp -r assets docs/

echo "[4/4] Build complete. You can now deploy the 'docs/' folder to GitHub Pages."

