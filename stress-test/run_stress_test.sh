#!/bin/bash
set -e

# =============================================================================
# Glust OTLP Stress Test Runner
# =============================================================================
# This script runs stress tests against the Glust OTLP ingestion endpoint
# and collects throughput and latency metrics.
#
# Prerequisites:
#   - wrk (https://github.com/wg/wrk)
#   - Rust toolchain (for building payload generator)
#   - Running Glust server on specified host:port
#
# Usage:
#   ./run_stress_test.sh [OPTIONS]
#
# Options:
#   -h, --host       Target host (default: localhost)
#   -p, --port       Target port (default: 3000)
#   -t, --threads    Number of threads (default: 4)
#   -c, --connections Number of connections (default: 100)
#   -d, --duration   Test duration (default: 30s)
#   -o, --output     Output file for results (default: results.txt)
# =============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default configuration
HOST="localhost"
PORT="3000"
THREADS=4
CONNECTIONS=100
DURATION="30s"
OUTPUT_FILE="results.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--host)
            HOST="$2"
            shift 2
            ;;
        -p|--port)
            PORT="$2"
            shift 2
            ;;
        -t|--threads)
            THREADS="$2"
            shift 2
            ;;
        -c|--connections)
            CONNECTIONS="$2"
            shift 2
            ;;
        -d|--duration)
            DURATION="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -h, --host        Target host (default: localhost)"
            echo "  -p, --port        Target port (default: 3000)"
            echo "  -t, --threads     Number of threads (default: 4)"
            echo "  -c, --connections Number of connections (default: 100)"
            echo "  -d, --duration    Test duration (default: 30s)"
            echo "  -o, --output      Output file for results (default: results.txt)"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

TARGET_URL="http://${HOST}:${PORT}"

print_header "Glust OTLP Stress Test"
echo ""
echo "Configuration:"
echo "  Target:      ${TARGET_URL}/v1/logs"
echo "  Threads:     ${THREADS}"
echo "  Connections: ${CONNECTIONS}"
echo "  Duration:    ${DURATION}"
echo "  Output:      ${OUTPUT_FILE}"
echo ""

# Check dependencies
print_header "Checking Dependencies"

if ! command -v wrk &> /dev/null; then
    print_error "wrk is not installed"
    echo "Install with:"
    echo "  Ubuntu/Debian: sudo apt install wrk"
    echo "  macOS: brew install wrk"
    exit 1
fi
print_success "wrk found"

if ! command -v cargo &> /dev/null; then
    print_error "Rust/Cargo is not installed"
    exit 1
fi
print_success "cargo found"

# Check if server is running
echo ""
print_header "Checking Server Availability"

if curl -s --connect-timeout 5 "${TARGET_URL}" > /dev/null 2>&1 || curl -s --connect-timeout 5 "${TARGET_URL}/v1/logs" > /dev/null 2>&1; then
    print_success "Server is reachable at ${TARGET_URL}"
else
    print_warning "Server may not be running at ${TARGET_URL}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Generate OTLP payload
echo ""
print_header "Generating OTLP Payload"

if [ ! -f "payload.bin" ]; then
    echo "Building payload generator..."
    cargo build --release -p stress-test --bin generate-payload

    echo "Generating payload..."
    ../target/release/generate-payload
else
    print_success "Using existing payload.bin"
fi

if [ ! -f "payload.bin" ]; then
    print_error "Failed to generate payload.bin"
    exit 1
fi

PAYLOAD_SIZE=$(stat -c%s "payload.bin" 2>/dev/null || stat -f%z "payload.bin" 2>/dev/null)
print_success "Payload ready: ${PAYLOAD_SIZE} bytes"

# Run stress test
echo ""
print_header "Running Stress Test"
echo "Starting wrk with ${THREADS} threads, ${CONNECTIONS} connections for ${DURATION}..."
echo ""

# Capture system info
TIMESTAMP=$(date -Iseconds)
KERNEL=$(uname -r)
CPU_INFO=$(grep "model name" /proc/cpuinfo 2>/dev/null | head -1 | cut -d: -f2 | xargs || sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "Unknown")

# Run wrk and capture output
WRK_OUTPUT=$(wrk -t${THREADS} -c${CONNECTIONS} -d${DURATION} -s wrk_otlp.lua "${TARGET_URL}/v1/logs" 2>&1)

echo "$WRK_OUTPUT"

# Save results to file
echo ""
print_header "Saving Results"

{
    echo "# Glust OTLP Stress Test Results"
    echo "# Generated: ${TIMESTAMP}"
    echo ""
    echo "## System Information"
    echo "- Kernel: ${KERNEL}"
    echo "- CPU: ${CPU_INFO}"
    echo ""
    echo "## Test Configuration"
    echo "- Target: ${TARGET_URL}/v1/logs"
    echo "- Threads: ${THREADS}"
    echo "- Connections: ${CONNECTIONS}"
    echo "- Duration: ${DURATION}"
    echo "- Payload Size: ${PAYLOAD_SIZE} bytes"
    echo ""
    echo "## Raw Output"
    echo '```'
    echo "$WRK_OUTPUT"
    echo '```'
} > "$OUTPUT_FILE"

print_success "Results saved to ${OUTPUT_FILE}"

echo ""
print_header "Test Complete"
