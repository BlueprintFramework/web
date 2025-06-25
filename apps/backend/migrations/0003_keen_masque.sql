ALTER TABLE "extensions" ADD COLUMN "keywords" varchar(255)[] DEFAULT '{}' NOT NULL;
CREATE INDEX IF NOT EXISTS "extensions_pending_idx" ON "extensions" USING btree ("pending");
CREATE INDEX IF NOT EXISTS "extensions_hidden_idx" ON "extensions" USING btree ("hidden");
CREATE INDEX IF NOT EXISTS "extensions_created_idx" ON "extensions" USING btree ("created");
CREATE INDEX IF NOT EXISTS "extensions_author_id_idx" ON "extensions" USING btree ("author_id");
CREATE INDEX IF NOT EXISTS "extensions_type_idx" ON "extensions" USING btree ("type");
CREATE INDEX IF NOT EXISTS "extensions_keywords_idx" ON "extensions" USING gin ("keywords");
