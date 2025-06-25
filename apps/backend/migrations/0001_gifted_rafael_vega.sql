TRUNCATE "telemetry_data", "telemetry_panels" CASCADE;
ALTER TABLE "telemetry_data" ALTER COLUMN "panel_id" SET DATA TYPE uuid;
ALTER TABLE "telemetry_data" ALTER COLUMN "data" SET DATA TYPE jsonb;
ALTER TABLE "telemetry_panels" ALTER COLUMN "id" SET DATA TYPE uuid;
ALTER TABLE "telemetry_data" ADD COLUMN "telemetry_version" smallint NOT NULL;
ALTER TABLE "telemetry_panels" ADD COLUMN "last_update" timestamp DEFAULT now() NOT NULL;
ALTER TABLE "telemetry_panels" DROP COLUMN IF EXISTS "version";
