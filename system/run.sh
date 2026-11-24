#!/usr/bin/env bash
# Run script untuk Learning Notes server

set -e
export PATH="$HOME/.cargo/bin:$PATH"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building..."
cargo build --release 2>&1

echo ""
echo "Server berjalan di http://localhost:3000"
echo "Tekan Ctrl+C untuk berhenti."
echo ""

./target/release/learning-notes
