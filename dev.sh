#!/bin/bash

# Uso: ./dev.sh [selector|editor]

set -e

APP=${1:-selector}  # Default: selector

echo "🚀 Running $APP in development mode..."

if [ "$APP" = "selector" ]; then
    RUST_LOG=debug cargo run --features="desktop" --bin selector
elif [ "$APP" = "editor" ]; then
    if [ -z "$2" ]; then
        echo "❌ Error: Editor needs a project folder"
        echo "Usage: ./dev.sh editor /path/to/project"
        exit 1
    fi
    RUST_LOG=debug cargo run --features="desktop" --bin editor -- "$2"
else
    echo "❌ Unknown app: $APP"
    echo "Usage: ./dev.sh [selector|editor] [project_path]"
    exit 1
fi
