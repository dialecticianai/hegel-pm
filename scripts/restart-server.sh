#!/bin/bash
# Restart hegel-pm server with fresh build

set -e  # Exit on error

# Create logs directory if it doesn't exist
mkdir -p logs

# Generate timestamped log file
LOG_FILE="logs/server-$(date +%Y%m%d-%H%M%S).log"

echo "📝 Logging to: $LOG_FILE"
echo "🛑 Stopping existing server..."
pkill -f "target/release/hegel-pm" || echo "No server running"

echo "🔨 Building..."
cargo build --release --features server 2>&1 | tee -a "$LOG_FILE"

echo "🚀 Starting server..."
cargo run --bin hegel-pm --features server --release 2>&1 | tee -a "$LOG_FILE"
