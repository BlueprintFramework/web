ALTER TABLE "users" ADD COLUMN "pronouns" varchar(22);
ALTER TABLE "users" ADD COLUMN "suspended" boolean DEFAULT false NOT NULL;

UPDATE extensions SET banner = REPLACE(banner, 'https://s3.blueprint.zip/extensions/', '');
