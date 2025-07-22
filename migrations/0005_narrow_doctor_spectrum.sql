DROP INDEX "extensions_name_idx";
CREATE INDEX "extensions_name_idx" ON "extensions" USING btree ("name");
