#!/bin/bash
set -e

# Cleanup function to kill background processes on exit
cleanup() {
    echo "Stopping processes..."
    kill $(jobs -p) 2>/dev/null || true
}
trap cleanup EXIT

echo "Building project..."
cargo build --bin glust
cargo build -p test-client

echo "Starting Glust Server (Ingestor)..."
cargo run --bin glust > glust.log 2>&1 &
SERVER_PID=$!
echo "Server PID: $SERVER_PID"

echo "Starting Test Client..."
cargo run -p test-client > client.log 2>&1 &
CLIENT_PID=$!
echo "Client PID: $CLIENT_PID"

# Wait for services to be up (simple sleep for MVP)
echo "Waiting for services to initialize..."
sleep 5

echo "Triggering log generation..."
curl -v -X POST -H "Content-Type: application/json" \
     -d '{"count": 5, "message": "hello world from script"}' \
     http://localhost:8080/generate

echo ""
echo "Logs generated. Checking server output..."
echo "--- Glust Server Output (tail) ---"
tail -n 10 glust.log

echo ""
echo "--- Test Client Output (tail) ---"
tail -n 10 client.log

echo ""
echo "Test finished. Press Ctrl+C to stop or wait 5 seconds before auto-exit."
sleep 5
