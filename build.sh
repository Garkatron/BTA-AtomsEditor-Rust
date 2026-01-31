#!/bin/bash

# Script para build release y ejecutar
# Uso: ./build.sh [selector|editor] [--run]

set -e

APP=${1:-selector}
RUN_AFTER=${2:-}

echo "🔨 Building $APP in release mode..."

# Build
cargo build --release --features="desktop" --bin $APP

if [ ! -f "target/release/$APP" ]; then
    echo "❌ Build failed: target/release/$APP not found"
    exit 1
fi

echo "✅ Build successful: target/release/$APP"

if [ "$RUN_AFTER" = "--run" ]; then
    echo "🚀 Running $APP..."
    if [ "$APP" = "editor" ]; then
        echo "⚠️  Note: Editor needs a project folder as argument"
        echo "Example: ./target/release/editor /path/to/project"
    else
        ./target/release/$APP
    fi
fi
