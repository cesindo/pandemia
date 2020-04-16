ALTER TABLE sub_reports RENAME "desc" TO notes;
ALTER TABLE sub_reports ADD COLUMN village_id BIGINT NOT NULL REFERENCES villages(id);
