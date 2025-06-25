CREATE TYPE "public"."extension_type" AS ENUM('THEME', 'EXTENSION');
CREATE TABLE IF NOT EXISTS "advent_calendar" (
	"extension_id" integer NOT NULL,
	"day" integer NOT NULL,
	"year" integer NOT NULL,
	"message" text NOT NULL,
	CONSTRAINT "advent_calendar_pk" PRIMARY KEY("day","year")
);

CREATE TABLE IF NOT EXISTS "authors" (
	"id" serial PRIMARY KEY NOT NULL,
	"name" varchar(255) NOT NULL,
	"website" varchar(63),
	"key" char(32) DEFAULT md5(random()::text) NOT NULL,
	"created" timestamp DEFAULT now() NOT NULL
);

CREATE TABLE IF NOT EXISTS "extensions" (
	"id" serial PRIMARY KEY NOT NULL,
	"author_id" integer NOT NULL,
	"type" "extension_type" NOT NULL,
	"hidden" boolean DEFAULT false NOT NULL,
	"pending" boolean DEFAULT true NOT NULL,
	"name" varchar(255) NOT NULL,
	"identifier" varchar(63) NOT NULL,
	"summary" varchar(255) NOT NULL,
	"platforms" jsonb NOT NULL,
	"banner" varchar(255) NOT NULL,
	"created" timestamp DEFAULT now() NOT NULL
);

CREATE TABLE IF NOT EXISTS "telemetry_data" (
	"id" serial PRIMARY KEY NOT NULL,
	"panel_id" char(23) NOT NULL,
	"data" varchar(255) NOT NULL,
	"ip" char(64) NOT NULL,
	"continent" char(2),
	"country" char(2),
	"created" timestamp DEFAULT now() NOT NULL
);

CREATE TABLE IF NOT EXISTS "telemetry_panels" (
	"id" char(23) PRIMARY KEY NOT NULL,
	"version" varchar(31) NOT NULL,
	"created" timestamp DEFAULT now() NOT NULL
);

DO $$ BEGIN
 ALTER TABLE "advent_calendar" ADD CONSTRAINT "advent_calendar_extension_id_extensions_id_fk" FOREIGN KEY ("extension_id") REFERENCES "public"."extensions"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
 ALTER TABLE "extensions" ADD CONSTRAINT "extensions_author_id_authors_id_fk" FOREIGN KEY ("author_id") REFERENCES "public"."authors"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
 ALTER TABLE "telemetry_data" ADD CONSTRAINT "telemetry_data_panel_id_telemetry_panels_id_fk" FOREIGN KEY ("panel_id") REFERENCES "public"."telemetry_panels"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

CREATE UNIQUE INDEX IF NOT EXISTS "authors_name_idx" ON "authors" USING btree ("name");
CREATE UNIQUE INDEX IF NOT EXISTS "authors_key_idx" ON "authors" USING btree ("key");
CREATE UNIQUE INDEX IF NOT EXISTS "extensions_name_idx" ON "extensions" USING btree ("name");
CREATE UNIQUE INDEX IF NOT EXISTS "extensions_identifier_idx" ON "extensions" USING btree ("identifier");
CREATE INDEX IF NOT EXISTS "telemetry_data_panel_id_idx" ON "telemetry_data" USING btree ("panel_id");
CREATE INDEX IF NOT EXISTS "telemetry_data_data_idx" ON "telemetry_data" USING btree ("data");
CREATE INDEX IF NOT EXISTS "telemetry_data_ip_idx" ON "telemetry_data" USING btree ("ip");
CREATE INDEX IF NOT EXISTS "telemetry_data_continent_idx" ON "telemetry_data" USING btree ("continent");
CREATE INDEX IF NOT EXISTS "telemetry_data_country_idx" ON "telemetry_data" USING btree ("country");
CREATE INDEX IF NOT EXISTS "telemetry_data_created_idx" ON "telemetry_data" USING btree ("created");
