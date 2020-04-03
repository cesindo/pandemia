CREATE TABLE geoloc_cache (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  "name" TEXT NOT NULL,
  latitude DOUBLE PRECISION NOT NULL,
  longitude DOUBLE PRECISION NOT NULL,
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_geoloc_cache_name ON geoloc_cache (
    (lower("name"))
);

