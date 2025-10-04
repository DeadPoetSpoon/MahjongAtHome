-- Your SQL goes here
CREATE TYPE ROLE AS ENUM ('Super', 'Normal', 'Guest');
CREATE TABLE user_role (
  name ROLE PRIMARY KEY,
  allowed TEXT[] NOT NULL,
  excepted TEXT[] NOT NULL
);
INSERT INTO user_role (name, allowed, excepted) VALUES ('Super', ARRAY['*'], ARRAY['NONE']);
INSERT INTO user_role (name, allowed, excepted) VALUES ('Normal', ARRAY['/api/user'], ARRAY['/api/user/signup']);
INSERT INTO user_role (name, allowed, excepted) VALUES ('Guest',  ARRAY['NONE'], ARRAY['*']);
