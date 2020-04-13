CREATE TABLE villages (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  "name" TEXT NOT NULL,
  sub_district TEXT NOT NULL,
  city TEXT NOT NULL, -- kota atau kabupaten
  province TEXT NOT NULL,
  latitude DOUBLE PRECISION NOT NULL,
  longitude DOUBLE PRECISION NOT NULL,
  meta TEXT[] NOT NULL DEFAULT '{}',
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_villages_subdistrict_city_province_name ON villages(province, city, sub_district, "name");
CREATE INDEX idx_villages_subdistrict_city_name ON villages(city, sub_district, "name");

