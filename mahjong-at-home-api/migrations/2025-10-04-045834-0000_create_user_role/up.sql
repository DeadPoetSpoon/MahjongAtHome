-- Your SQL goes here
CREATE TYPE ROLE AS ENUM ('Super', 'Normal', 'Guest');
CREATE TABLE user_role (
  name ROLE PRIMARY KEY,
  allowed TEXT[],
  excepted TEXT[]
);
INSERT INTO user_role (name, allowed, excepted) VALUES ('Super', ARRAY['*'], NULL);
INSERT INTO user_role (name, allowed, excepted) VALUES ('Normal', ARRAY['/api/user'], ARRAY['/api/user/signup']);
INSERT INTO user_role (name, allowed, excepted) VALUES ('Guest', NULL, ARRAY['*']);
