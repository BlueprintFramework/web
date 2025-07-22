CREATE TABLE "user_sessions" (
	"id" serial PRIMARY KEY NOT NULL,
	"user_id" integer NOT NULL,
	"key_id" char(16) NOT NULL,
    "key" bytea NOT NULL,
	"ip" inet NOT NULL,
	"user_agent" varchar(255) NOT NULL,
	"last_used" timestamp DEFAULT now() NOT NULL,
	"created" timestamp DEFAULT now() NOT NULL
);

CREATE INDEX "user_sessions_user_id_idx" ON "user_sessions" USING btree ("user_id");
CREATE UNIQUE INDEX "user_sessions_key_id_idx" ON "user_sessions" USING btree ("key_id");

ALTER TABLE "extensions" DROP CONSTRAINT "extensions_author_id_authors_id_fk";

ALTER TABLE "authors" RENAME TO "users";

ALTER TABLE "users" ADD COLUMN "email" varchar(255) DEFAULT gen_random_uuid() || '@example.com' NOT NULL;
ALTER TABLE "users" ADD COLUMN "email_pending" varchar(255);
ALTER TABLE "users" ADD COLUMN "email_verification" char(16);
ALTER TABLE "users" ADD COLUMN "admin" boolean DEFAULT false NOT NULL;
ALTER TABLE "users" ADD COLUMN "password" bytea;
ALTER TABLE "users" ADD COLUMN "github_id" bigint;

ALTER TABLE "users" DROP COLUMN "key";
ALTER TABLE "users" DROP COLUMN "website";

CREATE UNIQUE INDEX "users_email_idx" ON "users" USING btree ("email");

DO $$ BEGIN
 ALTER TABLE "user_sessions" ADD CONSTRAINT "user_sessions_user_id_users_id_fk" FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE cascade ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
 ALTER TABLE "extensions" ADD CONSTRAINT "extensions_author_id_users_id_fk" FOREIGN KEY ("author_id") REFERENCES "public"."users"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

CREATE TYPE "public"."extension_status" AS ENUM('APPROVED', 'READY', 'PENDING');

ALTER TABLE "extensions" DROP COLUMN "pending";
ALTER TABLE "extensions" RENAME COLUMN "hidden" TO "unlisted";
ALTER TABLE "extensions" ADD COLUMN "status" "extension_status" NOT NULL DEFAULT 'APPROVED';
