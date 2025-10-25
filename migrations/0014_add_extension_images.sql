CREATE TABLE "extension_images" (
	"id" serial PRIMARY KEY NOT NULL,
	"extension_id" integer NOT NULL,
	"width" integer NOT NULL,
	"height" integer NOT NULL,
	"size" integer NOT NULL,
	"location" varchar(255) NOT NULL,
	"created" timestamp DEFAULT now() NOT NULL
);

DO $$ BEGIN
 ALTER TABLE "extension_images" ADD CONSTRAINT "extension_images_extension_id_extensions_id_fk" FOREIGN KEY ("extension_id") REFERENCES "public"."extensions"("id") ON DELETE CASCADE ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

CREATE INDEX IF NOT EXISTS "extension_images_extension_id_idx" ON "extension_images" USING btree ("extension_id");
