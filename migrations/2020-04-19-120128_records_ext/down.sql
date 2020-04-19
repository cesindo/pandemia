
ALTER TABLE records DROP COLUMN ppdwt;
ALTER TABLE records DROP COLUMN pptb;
ALTER TABLE records DROP COLUMN odp;
ALTER TABLE records DROP COLUMN odpsp;
ALTER TABLE records DROP COLUMN pdp;
ALTER TABLE records DROP COLUMN pdps;
ALTER TABLE records DROP COLUMN pdpm;
ALTER TABLE records DROP COLUMN otg;

ALTER TABLE records DROP COLUMN loc_path;


DROP INDEX idx_records_meta;
DROP INDEX idx_records_loc_path;
