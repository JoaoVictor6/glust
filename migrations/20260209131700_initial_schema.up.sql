-- Create Table
CREATE TABLE IF NOT EXISTS logs (
    timestamp           TIMESTAMPTZ NOT NULL,
    trace_id            TEXT, -- Hex string
    span_id             TEXT, -- Hex string
    trace_flags         SMALLINT,
    severity_text       TEXT,
    severity_number     INTEGER,
    body                JSONB,
    resource_attributes JSONB,
    scope_attributes    JSONB,
    log_attributes      JSONB,
    service_name        TEXT NOT NULL -- Extracted from resource_attributes for faster filtering
);

-- Indexes for common query patterns
-- 1. Filter by Service + Time range (Dashboard view)
CREATE INDEX IF NOT EXISTS idx_logs_service_time ON logs (service_name, timestamp DESC);

-- 2. Trace correlation (Find all logs for a trace)
CREATE INDEX IF NOT EXISTS idx_logs_trace_id ON logs (trace_id);
CREATE INDEX IF NOT EXISTS idx_logs_span_id ON logs (span_id);

-- 3. Global tail (Latest logs across all services)
CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs (timestamp DESC);
