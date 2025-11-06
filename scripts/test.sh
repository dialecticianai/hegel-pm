#!/bin/bash
# Build and test hegel-pm without starting the server
#
# Usage:
#   ./scripts/test.sh                      # Build + test everything (default: Sycamore)
#   FRONTEND=alpine ./scripts/test.sh      # Build with Alpine.js frontend
#   ./scripts/test.sh --exclude frontend   # Backend only (skip frontend build)
#   ./scripts/test.sh --exclude backend    # Frontend only (skip cargo)
#
# Environment variables:
#   FRONTEND   - Frontend to build (default: sycamore)
#                Valid values: sycamore, alpine
#
# This script is useful when you want to verify changes without restarting the server.

set -e

SKIP_FRONTEND=false
SKIP_BACKEND=false
FRONTEND=${FRONTEND:-sycamore}

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
    echo "🎨 Building frontend ($FRONTEND)..."

    case "$FRONTEND" in
        sycamore)
            trunk build --release
            ;;
        alpine)
            if [ ! -d "frontends/alpine" ]; then
                echo "Error: Frontend directory not found: frontends/alpine/"
                echo "See frontends/ADDING_FRONTENDS.md for setup instructions"
                exit 1
            fi
            cp -r frontends/alpine/* static/
            ;;
        *)
            echo "Error: Unknown frontend '$FRONTEND'"
            echo "Valid frontends: sycamore, alpine"
            exit 1
            ;;
    esac

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
