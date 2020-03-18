-- This model structure modeled after data from https://www.worldometers.info/coronavirus/
CREATE TABLE records (
  id BIGSERIAL PRIMARY KEY,
  loc TEXT NOT NULL,
  loc_kind SMALLINT NOT NULL DEFAULT 1, -- 0: Continent, 1: Country, 2: Province, 3: City
  total_cases INT NOT NULL DEFAULT 0,
  total_deaths INT NOT NULL DEFAULT 0,
  total_recovered INT NOT NULL DEFAULT 0,
  active_cases INT NOT NULL DEFAULT 0,
  critical_cases INT NOT NULL DEFAULT 0,
  cases_to_pop DOUBLE PRECISION NOT NULL DEFAULT 0.0,
  meta TEXT[] NOT NULL DEFAULT '{}',
  last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
