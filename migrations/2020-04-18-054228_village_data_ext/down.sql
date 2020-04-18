ALTER TABLE village_data DROP COLUMN ppdwt;
ALTER TABLE village_data DROP COLUMN pptb;
ALTER TABLE village_data DROP COLUMN odpsp;
ALTER TABLE village_data DROP COLUMN pdps;
ALTER TABLE village_data DROP COLUMN pdpm;
ALTER TABLE village_data DROP COLUMN otg;
ALTER TABLE village_data DROP COLUMN district_id;

ALTER TABLE district_data DROP COLUMN ppdwt;
ALTER TABLE district_data DROP COLUMN pptb;
ALTER TABLE district_data DROP COLUMN odpsp;
ALTER TABLE district_data DROP COLUMN pdps;
ALTER TABLE district_data DROP COLUMN pdpm;
ALTER TABLE district_data DROP COLUMN otg;


DROP trigger auto_update_district_data ON village_data;
DROP FUNCTION calc_district_data() cascade;
