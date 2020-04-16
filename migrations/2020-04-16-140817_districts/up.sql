CREATE TABLE districts (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  "name" TEXT NOT NULL,
  city_id BIGINT NOT NULL REFERENCES cities(id),
  meta TEXT[] NOT NULL DEFAULT '{}'
);

CREATE UNIQUE INDEX idx_districts_name ON districts(city_id,lower(name));

INSERT INTO districts ("name", city_id)
VALUES
  ('Garung', 1),
  ('Kalibawang', 1),
  ('Kalikajar', 1),
  ('Kaliwiro', 1),
  ('Kejajar', 1),
  ('Kepil', 1),
  ('Kertek', 1),
  ('Leksono', 1),
  ('Mojotengah', 1),
  ('Sapuran', 1),
  ('Selomerto', 1),
  ('Sukoharjo', 1),
  ('Wadaslintang', 1),
  ('Watumalang', 1),
  ('Wonosobo', 1)
;


ALTER TABLE villages RENAME COLUMN "sub_district" TO "district_name";
