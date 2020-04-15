CREATE TABLE report_notes (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  notes TEXT NOT NULL,
  creator_id BIGINT NOT NULL REFERENCES users(id),
  creator_name TEXT NOT NULL DEFAULT '',
  city_id BIGINT NOT NULL REFERENCES cities(id),
  approved BOOL NOT NULL DEFAULT FALSE,
  meta TEXT[] NOT NULL DEFAULT '{}',
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_reort_notes_id ON report_notes(city_id);
