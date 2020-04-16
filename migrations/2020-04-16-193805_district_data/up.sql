CREATE TABLE district_data (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  district_id BIGINT NOT NULL REFERENCES districts(id),
  odp INT NOT NULL,
  pdp INT NOT NULL,
  cases INT NOT NULL,
  recovered INT NOT NULL,
  deaths INT NOT NULL,
  last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_updated_by_id BIGINT NOT NULL DEFAULT 0,
  city_id BIGINT NOT NULL,
  meta TEXT[] NOT NULL DEFAULT '{}',
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_district_data_city_id_district_id ON district_data(city_id, district_id);
