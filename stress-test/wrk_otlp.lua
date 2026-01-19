-- wrk lua script for OTLP stress testing
-- Usage: wrk -t4 -c100 -d30s -s wrk_otlp.lua http://localhost:3000/v1/logs

local payload = nil

function init(args)
    local file = io.open("payload.bin", "rb")
    if file then
        payload = file:read("*all")
        file:close()
        print("Loaded OTLP payload: " .. #payload .. " bytes")
    else
        print("ERROR: Could not load payload.bin - run generate-payload first")
        os.exit(1)
    end
end

function request()
    return wrk.format("POST", "/v1/logs", {
        ["Content-Type"] = "application/x-protobuf"
    }, payload)
end

function response(status, headers, body)
    if status ~= 200 then
        print("Error response: " .. status .. " - " .. body)
    end
end

function done(summary, latency, requests)
    print("\n========================================")
    print("STRESS TEST RESULTS")
    print("========================================")

    local duration_s = summary.duration / 1000000
    local requests_per_sec = summary.requests / duration_s
    local bytes_per_sec = summary.bytes / duration_s

    print(string.format("\nDuration: %.2f seconds", duration_s))
    print(string.format("Total Requests: %d", summary.requests))
    print(string.format("Total Errors: %d", summary.errors.status + summary.errors.connect + summary.errors.read + summary.errors.write + summary.errors.timeout))

    print("\n--- Throughput ---")
    print(string.format("Requests/sec: %.2f", requests_per_sec))
    print(string.format("Transfer/sec: %.2f KB", bytes_per_sec / 1024))

    print("\n--- Latency Distribution ---")
    print(string.format("Mean:    %.2f ms", latency.mean / 1000))
    print(string.format("Stdev:   %.2f ms", latency.stdev / 1000))
    print(string.format("Max:     %.2f ms", latency.max / 1000))
    print(string.format("p50:     %.2f ms", latency:percentile(50) / 1000))
    print(string.format("p90:     %.2f ms", latency:percentile(90) / 1000))
    print(string.format("p99:     %.2f ms", latency:percentile(99) / 1000))
    print(string.format("p99.9:   %.2f ms", latency:percentile(99.9) / 1000))

    print("\n--- Error Breakdown ---")
    print(string.format("Connect errors: %d", summary.errors.connect))
    print(string.format("Read errors:    %d", summary.errors.read))
    print(string.format("Write errors:   %d", summary.errors.write))
    print(string.format("Timeout errors: %d", summary.errors.timeout))
    print(string.format("Status errors:  %d", summary.errors.status))
    print("========================================\n")
end
