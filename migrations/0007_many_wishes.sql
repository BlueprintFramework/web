DROP INDEX IF EXISTS "telemetry_data_created_idx";
CREATE INDEX IF NOT EXISTS "telemetry_data_created_idx" ON "telemetry_data" USING brin ("created");
