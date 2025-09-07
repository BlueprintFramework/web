DROP INDEX IF EXISTS "authors_name_idx";
CREATE UNIQUE INDEX IF NOT EXISTS "users_name_idx" ON "users" USING btree (LOWER("name"));

ALTER TABLE "users" ADD COLUMN "totp_enabled" boolean DEFAULT false NOT NULL;
ALTER TABLE "users" ADD COLUMN "totp_secret" char(32);

CREATE TABLE "user_recovery_codes" (
	"id" serial PRIMARY KEY NOT NULL,
	"user_id" integer NOT NULL,
	"code" text NOT NULL,
	"created" timestamp DEFAULT now() NOT NULL
);

ALTER TABLE "user_recovery_codes" ADD CONSTRAINT "user_recovery_codes_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "users"("id") ON DELETE cascade ON UPDATE no action;
CREATE INDEX "user_recovery_codes_user_id_idx" ON "user_recovery_codes" USING btree ("user_id");
CREATE UNIQUE INDEX "user_recovery_codes_user_id_code_idx" ON "user_recovery_codes" USING btree ("user_id","code");
