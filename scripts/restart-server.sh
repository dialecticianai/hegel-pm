#!/bin/bash
# Restart hegel-pm server with fresh build
#
# Usage:
#   ./restart-server.sh           # Backend only
#   ./restart-server.sh --frontend # Backend + frontend (WASM)

set -e  # Exit on error

# Parse arguments
BUILD_FRONTEND=false
if [[ "$1" == "--frontend" ]]; then
    BUILD_FRONTEND=true
fi

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
    echo "🎨 Building frontend (WASM)..."
    trunk build --release 2>&1 | tee -a "$LOG_FILE"
fi

echo "🔨 Building backend..."
cargo build --release --features server 2>&1 | tee -a "$LOG_FILE"

echo "✅ Build complete"
echo "🚀 Starting server..."
cargo run --bin hegel-pm --features server --release 2>&1 | tee -a "$LOG_FILE"
