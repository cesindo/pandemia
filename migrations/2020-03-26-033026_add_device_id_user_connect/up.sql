ALTER TABLE user_connect DROP CONSTRAINT user_connect_user_id_fkey;
ALTER TABLE user_connect RENAME COLUMN user_id TO device_id;
ALTER TABLE user_connect ALTER COLUMN device_id TYPE VARCHAR(50);