CREATE TABLE sub_reports (
  id BIGSERIAL NOT NULL PRIMARY KEY,
  creator_id BIGINT NOT NULL DEFAULT 0 REFERENCES users(id) ON DELETE SET DEFAULT,
  creator_name VARCHAR NOT NULL,
  full_name VARCHAR NOT NULL,
  age INT NOT NULL,
  residence_address VARCHAR NOT NULL,
  gender VARCHAR(1) NOT NULL, -- L: Laki-laki, P: Perempuan
  arrival_address VARCHAR NOT NULL,
  arrival_date DATE NOT NULL,
  healthy BOOLEAN NOT NULL, -- True: Sehat, False: Ada Gejala
  "desc" VARCHAR NOT NULL, -- Kerja/Mondok/Kuliah/Lainnya
  "status" INT NOT NULL, -- 1: ODP, 2: PDP, 3: Positive, 4: Recovered
  meta TEXT[] NOT NULL DEFAULT '{}',
  ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);