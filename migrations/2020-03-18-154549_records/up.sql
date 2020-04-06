-- This model structure modeled after data from https://www.worldometers.info/coronavirus/
CREATE TABLE records (
  id BIGSERIAL PRIMARY KEY,
  loc TEXT NOT NULL,
  loc_kind SMALLINT NOT NULL DEFAULT 1, -- 0: Global, 1: Continent, 2: Country, 3: Province, 4: City
  total_cases INT NOT NULL DEFAULT 0,
  total_deaths INT NOT NULL DEFAULT 0,
  total_recovered INT NOT NULL DEFAULT 0,
  active_cases INT NOT NULL DEFAULT 0,
  critical_cases INT NOT NULL DEFAULT 0,
  latest BOOLEAN NOT NULL DEFAULT FALSE,
  meta TEXT[] NOT NULL DEFAULT '{}',
  last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_records_loc_name_lower ON records (
    (lower(loc))
);


