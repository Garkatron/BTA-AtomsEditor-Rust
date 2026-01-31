#!/bin/bash


set -e

echo "🔨 Building all binaries in release mode..."

cargo build --release --features="desktop" --bin selector
cargo build --release --features="desktop" --bin editor

echo ""
echo "✅ Build complete!"
echo "   Selector: ./target/release/selector"
echo "   Editor:   ./target/release/editor"
echo ""
echo "Run selector: ./target/release/selector"
echo "Run editor:   ./target/release/editor /path/to/project"
