DROP INDEX IF EXISTS "telemetry_data_data_idx";
CREATE INDEX IF NOT EXISTS "telemetry_data_data_idx" ON "telemetry_data" USING gin ("data");
