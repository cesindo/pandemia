CREATE TABLE village_data (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  village_id BIGINT NOT NULL REFERENCES villages(id),
  odp INT NOT NULL,
  pdp INT NOT NULL,
  cases INT NOT NULL,
  recovered INT NOT NULL,
  deaths INT NOT NULL,
  last_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_updated_by_id BIGINT NOT NULL REFERENCES users(id),
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_village_data_village_id ON village_data(village_id);
