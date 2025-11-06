#!/bin/bash
# Restart hegel-pm server with fresh build
#
# Usage:
#   ./restart-server.sh                      # Backend only
#   ./restart-server.sh --frontend           # Backend + frontend (default: Sycamore)
#   FRONTEND=alpine ./restart-server.sh --frontend  # Backend + Alpine.js frontend
#
# Environment variables:
#   FRONTEND   - Frontend to build (default: sycamore)
#                Valid values: sycamore, alpine

set -e  # Exit on error

# Parse arguments
BUILD_FRONTEND=false
if [[ "$1" == "--frontend" ]]; then
    BUILD_FRONTEND=true
fi

FRONTEND=${FRONTEND:-sycamore}

# Create logs directory if it doesn't exist
mkdir -p logs

# Generate timestamped log file
LOG_FILE="logs/server-$(date +%Y%m%d-%H%M%S).log"

echo "📝 Logging to: $LOG_FILE"

echo "🛑 Stopping existing server..."
pkill -f "target/release/hegel-pm" || echo "No server running"
# Wait a moment for process to fully terminate
sleep 0.5

if [ "$BUILD_FRONTEND" = true ]; then
    echo "🎨 Building frontend ($FRONTEND)..."

    case "$FRONTEND" in
        sycamore)
            trunk build --release 2>&1 | tee -a "$LOG_FILE"
            ;;
        alpine)
            if [ ! -d "frontends/alpine" ]; then
                echo "Error: Frontend directory not found: frontends/alpine/"
                echo "See frontends/ADDING_FRONTENDS.md for setup instructions"
                exit 1
            fi
            cp -r frontends/alpine/* static/ 2>&1 | tee -a "$LOG_FILE"
            ;;
        *)
            echo "Error: Unknown frontend '$FRONTEND'"
            echo "Valid frontends: sycamore, alpine"
            exit 1
            ;;
    esac
fi

echo "🔨 Building backend..."
cargo build --release --features server 2>&1 | tee -a "$LOG_FILE"

echo "✅ Build complete"
echo "🚀 Starting server..."
cargo run --bin hegel-pm --features server --release 2>&1 | tee -a "$LOG_FILE"
