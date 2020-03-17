CREATE TABLE admins (
    id BIGSERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    phone_num VARCHAR NOT NULL,
    labels TEXT[] NOT NULL DEFAULT '{}',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    register_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX admins_email ON admins (
    (lower(email))
);

-- create nobody admin
INSERT INTO admins (id, "name", email, phone_num, active)
VALUES
(0, 'nobody', 'nobody@pandemia.net', '+62123', TRUE),
(1, 'Admin', 'admin@pandemia.net', '+62456', TRUE)
;

SELECT nextval('admins_id_seq');

-- Berisi koleksi passhash dari akun
-- dibuat one-to-many agar ada history-nya setiap user merubah password.
CREATE TABLE admin_passhash (
    id BIGSERIAL PRIMARY KEY,
    admin_id BIGINT NOT NULL REFERENCES admins(id) ON DELETE CASCADE,
    passhash VARCHAR NOT NULL,
    deprecated BOOLEAN NOT NULL,
    ver INT NOT NULL, -- passhash versioning, dibutuhkan apabila ingin merubah algo passhash ketika sudah jalan.
    created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO admin_passhash (admin_id, passhash, deprecated, ver)
VALUES
(1, '$2y$05$mw56Wls35HoufQH7QipJnOzqzVmZuwcVUojcqQxKZ5hcG8aBdZRo.', FALSE, 1)
;

CREATE TABLE admin_access_tokens (
    token TEXT PRIMARY KEY,
    admin_id BIGINT NOT NULL REFERENCES admins (id) ON DELETE CASCADE,
    created TIMESTAMP NOT NULL,
    valid_thru TIMESTAMP NOT NULL
);

CREATE INDEX idx_access_tokens_admin_id ON admin_access_tokens (
    (admin_id)
);


CREATE TABLE reset_password_admins (
  admin_id BIGINT NOT NULL REFERENCES admins(id) ON DELETE CASCADE,
  token VARCHAR(100) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT (now()),
  expiration TIMESTAMP DEFAULT NULL,
  PRIMARY KEY (admin_id)
);


