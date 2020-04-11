CREATE INDEX idx_gist_map_markers ON map_markers USING gist (ll_to_earth(latitude, longitude));
