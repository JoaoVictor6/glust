# Glust OTLP Stress Testing Guide

This directory contains tools for stress testing the Glust OTLP log ingestion endpoint.

## Overview

The stress testing pipeline consists of:

1. **Payload Generator** (`generate-payload`) - Creates a fixed OTLP protobuf payload
2. **wrk Lua Script** (`wrk_otlp.lua`) - Loads the payload and executes HTTP stress tests
3. **Runner Script** (`run_stress_test.sh`) - Orchestrates the test and collects metrics

## Prerequisites

### Required Tools

- **Rust toolchain** - For building the payload generator
- **wrk** - HTTP benchmarking tool ([github.com/wg/wrk](https://github.com/wg/wrk))

### Installation

```bash
# Ubuntu/Debian
sudo apt install wrk

# macOS
brew install wrk

# Arch Linux
sudo pacman -S wrk
```

## Quick Start

```bash
cd stress-test

# Run with defaults (localhost:3000, 4 threads, 100 connections, 30s)
./run_stress_test.sh

# Custom configuration
./run_stress_test.sh --host localhost --port 3000 --threads 8 --connections 200 --duration 60s
```

## Manual Execution

### 1. Generate OTLP Payload

```bash
cargo build --release -p stress-test
../target/release/generate-payload
# Creates: payload.bin
```

### 2. Run wrk Directly

```bash
wrk -t4 -c100 -d30s -s wrk_otlp.lua http://localhost:3000/v1/logs
```

## Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `-t, --threads` | 4 | Number of wrk threads |
| `-c, --connections` | 100 | Number of concurrent connections |
| `-d, --duration` | 30s | Test duration |
| `-h, --host` | localhost | Target host |
| `-p, --port` | 3000 | Target port |
| `-o, --output` | results.txt | Output file for results |

## Metrics Collected

### Throughput Metrics
- **Requests/sec** - Number of requests completed per second
- **Transfer/sec** - Data transfer rate

### Latency Distribution
- **Mean** - Average latency
- **Stdev** - Standard deviation
- **Max** - Maximum observed latency
- **p50** - 50th percentile (median)
- **p90** - 90th percentile
- **p99** - 99th percentile
- **p99.9** - 99.9th percentile

### Error Tracking
- Connect errors
- Read errors
- Write errors
- Timeout errors
- HTTP status errors (non-200)

## Interpreting Results

### Sample Output

```
========================================
STRESS TEST RESULTS
========================================

Duration: 30.00 seconds
Total Requests: 150000
Total Errors: 0

--- Throughput ---
Requests/sec: 5000.00
Transfer/sec: 512.00 KB

--- Latency Distribution ---
Mean:    2.50 ms
Stdev:   1.20 ms
Max:     45.00 ms
p50:     2.00 ms
p90:     4.00 ms
p99:     8.00 ms
p99.9:   15.00 ms
```

### Key Indicators

| Metric | Good | Concerning |
|--------|------|------------|
| p99 latency | < 10ms | > 50ms |
| Error rate | 0% | > 0.1% |
| Throughput variance | < 10% | > 20% |

## Test Scenarios

### Baseline Test
```bash
./run_stress_test.sh -t2 -c50 -d30s
```

### High Concurrency
```bash
./run_stress_test.sh -t8 -c500 -d60s
```

### Sustained Load
```bash
./run_stress_test.sh -t4 -c100 -d300s
```

### Spike Test
```bash
# Run multiple tests with increasing load
for c in 50 100 200 400 800; do
    echo "Testing with $c connections..."
    ./run_stress_test.sh -c$c -d10s -o "results_${c}.txt"
    sleep 5
done
```

## Results Documentation Template

After running tests, document results in the following format:

```markdown
## Stress Test Results - [DATE]

### Environment
- Server: [specs]
- Client: [specs]
- Network: [local/remote]

### Test Configuration
- Duration: 30s
- Threads: 4
- Connections: 100

### Results
| Metric | Value |
|--------|-------|
| Requests/sec | X |
| p50 | X ms |
| p99 | X ms |
| Error rate | X% |

### Observations
- [Notes about performance]
- [Bottlenecks identified]
- [Recommendations]
```

## Troubleshooting

### "Server not reachable"
Ensure the Glust server is running:
```bash
cargo run --release
```

### "payload.bin not found"
Generate the payload first:
```bash
cargo build --release -p stress-test
../target/release/generate-payload
```

### Low throughput
- Check server resource utilization (CPU, memory)
- Verify network isn't a bottleneck
- Consider increasing server worker threads

### High error rates
- Check server logs for errors
- Verify connection limits aren't being hit
- Monitor for resource exhaustion
