CREATE TABLE IF NOT EXISTS user_connect (
  device_id TEXT NOT NULL PRIMARY KEY,
  user_id BIGINT  NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider_name VARCHAR(50) NOT NULL, -- eg: android, ios
  app_id TEXT NOT NULL,
  enable_push_notif BOOLEAN NOT NULL DEFAULT TRUE,
  latest_loc TEXT NOT NULL DEFAULT '',
  latest_loc_full TEXT NOT NULL DEFAULT '',
  latest_loc_long double precision NOT NULL DEFAULT '0.0',
  latest_loc_lat double precision NOT NULL DEFAULT '0.0'
);

CREATE INDEX idx_user_connect_latest_loc_lower ON user_connect (
    (lower(latest_loc))
);

CREATE INDEX idx_user_connect_latest_loc_full_lower ON user_connect (
    (lower(latest_loc_full))
);

