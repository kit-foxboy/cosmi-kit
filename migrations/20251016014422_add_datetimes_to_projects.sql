-- Add migration script here
ALTER TABLE projects
ADD COLUMN updated_at INTEGER NOT NULL DEFAULT 0;

ALTER TABLE features
ADD COLUMN updated_at INTEGER NOT NULL DEFAULT 0;

ALTER TABLE tags
ADD COLUMN created_at INTEGER NOT NULL DEFAULT 0;
ALTER TABLE tags
ADD COLUMN updated_at INTEGER NOT NULL DEFAULT 0;

-- backfill existing rows with current epoch seconds
UPDATE projects
SET updated_at = CAST(strftime('%s','now') AS INTEGER)
WHERE updated_at = 0;

UPDATE features
SET updated_at = CAST(strftime('%s','now') AS INTEGER)
WHERE updated_at = 0;

UPDATE tags
SET created_at = CAST(strftime('%s','now') AS INTEGER)
WHERE created_at = 0;

UPDATE tags
SET updated_at = CAST(strftime('%s','now') AS INTEGER)
WHERE updated_at = 0;
