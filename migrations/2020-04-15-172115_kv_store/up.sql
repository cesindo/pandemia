CREATE TABLE kv_store (
  id BIGSERIAL PRIMARY KEY NOT NULL,
  a_key TEXT NOT NULL,
  a_val TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_kv_store_a_key ON kv_store(a_key);

