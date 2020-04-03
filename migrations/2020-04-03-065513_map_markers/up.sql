CREATE TABLE map_markers (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  "name" TEXT NOT NULL,
  info TEXT NOT NULL DEFAULT '',
  latitude DOUBLE PRECISION NOT NULL,
  longitude DOUBLE PRECISION NOT NULL,
  kind SMALLINT NOT NULL, -- Untuk jenis-jenisnya lihat [[MapMarkerKind]]
  meta TEXT[] NOT NULL DEFAULT '{}',
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_map_markers_name ON map_markers (
    (lower("name"))
);
