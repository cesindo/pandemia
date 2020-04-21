DROP TRIGGER IF EXISTS auto_update_district_data ON village_data;
DROP FUNCTION calc_district_data();


CREATE OR REPLACE FUNCTION calc_district_data() 
RETURNS trigger AS $$
DECLARE
a_odp BIGINT;
a_pdp BIGINT;
a_cases BIGINT;
a_recovered BIGINT;
a_deaths BIGINT;
a_ppdwt BIGINT;
a_pptb BIGINT;
a_odpsp BIGINT;
a_pdps BIGINT;
a_pdpm BIGINT;
a_otg BIGINT;
V_REC village_data%ROWTYPE;
BEGIN
  if tg_op = 'UPDATE' then
    V_REC = NEW;
  elsif tg_op = 'INSERT' then
    V_REC = NEW;
  elsif tg_op = 'DELETE' then
    V_REC = OLD;
  end if;
    SELECT INTO a_odp, a_pdp, a_cases, a_recovered, a_deaths, a_ppdwt, a_pptb, a_odpsp, a_pdps, a_pdpm, a_otg  
      COALESCE(SUM(odp), 0), COALESCE(SUM(pdp), 0), COALESCE(SUM(cases),0), COALESCE(SUM(recovered),0), COALESCE(SUM(deaths),0), COALESCE(SUM(ppdwt),0), COALESCE(SUM(pptb),0), COALESCE(SUM(odpsp),0), COALESCE(SUM(pdps),0), COALESCE(SUM(pdpm),0), COALESCE(SUM(otg),0)
        FROM village_data WHERE district_id = V_REC.district_id;
    INSERT INTO district_data 
      (district_id, last_updated, city_id, odp, pdp, cases, recovered, deaths, ppdwt, pptb, odpsp, pdps, pdpm, otg)
      VALUES 
      (V_REC.district_id, CURRENT_TIMESTAMP, V_REC.city_id, a_odp, a_pdp, a_cases, a_recovered, a_deaths, a_ppdwt, a_pptb, a_odpsp, a_pdps, a_pdpm, a_otg)
      ON CONFLICT (district_id)
      DO UPDATE
        SET (odp, pdp, cases, recovered, deaths, ppdwt, pptb, odpsp, pdps, pdpm, otg)
        = (a_odp, a_pdp, a_cases, a_recovered, a_deaths, a_ppdwt, a_pptb, a_odpsp, a_pdps, a_pdpm, a_otg);
        -- WHERE id = OLD.district_id;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER auto_update_district_data AFTER UPDATE OR INSERT OR DELETE
ON village_data
FOR EACH ROW EXECUTE PROCEDURE calc_district_data();



