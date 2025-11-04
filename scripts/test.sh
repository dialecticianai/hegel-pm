#!/bin/bash
# Build and test hegel-pm without starting the server
#
# Usage:
#   ./scripts/test.sh                      # Build + test everything (default)
#   ./scripts/test.sh --exclude frontend   # Backend only (skip WASM)
#   ./scripts/test.sh --exclude backend    # Frontend only (skip cargo)
#
# This script is useful when you want to verify changes without restarting the server.

set -e

SKIP_FRONTEND=false
SKIP_BACKEND=false

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --exclude)
            if [[ "$2" == "frontend" ]]; then
                SKIP_FRONTEND=true
                shift
            elif [[ "$2" == "backend" ]]; then
                SKIP_BACKEND=true
                shift
            else
                echo "Error: --exclude requires 'frontend' or 'backend'"
                exit 1
            fi
            ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

echo "🧪 Building and testing hegel-pm..."
echo

if [ "$SKIP_FRONTEND" = false ]; then
    echo "🎨 Building frontend (WASM)..."
    trunk build --release
    echo
fi

if [ "$SKIP_BACKEND" = false ]; then
    echo "🔨 Building backend (release)..."
    cargo build --release --features server
    echo

    echo "✅ Running tests..."
    cargo test --features server
    echo
fi

echo "✓ Build and tests complete!"
