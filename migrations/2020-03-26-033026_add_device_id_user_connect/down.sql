ALTER TABLE user_connect RENAME COLUMN device_id TO user_id;
ALTER TABLE user_connect ALTER COLUMN user_id TYPE BIGINT;
ALTER TABLE user_connect ADD CONSTRAINT user_connect_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;