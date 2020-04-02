CREATE TABLE IF NOT EXISTS user_connect (
  device_id TEXT NOT NULL PRIMARY KEY,
  user_id BIGINT  NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  provider_name VARCHAR(50) NOT NULL, -- eg: android, ios
  app_id TEXT NOT NULL,
  enable_push_notif BOOLEAN NOT NULL DEFAULT TRUE,
  latest_location TEXT NOT NULL DEFAULT '{}'
);
