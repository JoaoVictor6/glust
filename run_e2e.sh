#!/bin/bash
set -e

# Cleanup function to kill background processes on exit
cleanup() {
    echo "Stopping processes..."
    kill $(jobs -p) 2>/dev/null || true
    echo "Stopping Docker services..."
    docker compose down
}
trap cleanup EXIT

echo "Starting Docker services..."
docker compose up -d
echo "Waiting for DB..."
sleep 5

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

# Wait for services to be up (retry/health check)
echo "Waiting for Glust Server to be ready..."
for i in {1..30}; do
    if curl -s http://localhost:3000/health > /dev/null; then
        echo "Server is up!"
        break
    fi
    echo "Waiting for server... ($i/30)"
    sleep 1
done

# Check if server came up
if ! curl -s http://localhost:3000/health > /dev/null; then
    echo "Server failed to start. Logs:"
    cat glust.log
    exit 1
fi

echo "Waiting for Test Client..."
sleep 2

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
