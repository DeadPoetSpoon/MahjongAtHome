-- Your SQL goes here

CREATE TABLE mahjong_user (
  id Serial PRIMARY KEY,
  email Text NOT NULL UNIQUE,
  psd Text NOT NULL,
  role ROLE NOT NULL DEFAULT 'Normal'
);
INSERT INTO mahjong_user (email, psd, role) VALUES ('admin@math.com', '$argon2id$v=19$m=19456,t=2,p=1$7nCfBBMpmCnPFUnnigj2bg$GKn8RZwpvGLMVwbulQP+JFCS0rZt99Gt/2NADQqabZw', 'Super');
