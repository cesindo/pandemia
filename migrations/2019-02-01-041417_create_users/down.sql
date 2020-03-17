DROP INDEX register_users_email;
DROP INDEX register_users_phone_num;

DELETE FROM users WHERE id = 0;

DROP TABLE user_keys;
DROP TABLE user_passhash;
DROP TABLE register_users;
DROP TABLE addresses;
DROP TABLE users;



