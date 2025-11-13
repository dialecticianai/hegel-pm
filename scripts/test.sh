#!/bin/bash
# Build and test hegel-pm CLI
#
# Usage:
#   ./scripts/test.sh    # Build + test
#
# This script is useful when you want to verify changes quickly.

set -e

echo "🧪 Building and testing hegel-pm..."
echo

echo "🔨 Building CLI (release)..."
cargo build --release
echo

echo "✅ Running tests..."
cargo test
echo

echo "✓ Build and tests complete!"
