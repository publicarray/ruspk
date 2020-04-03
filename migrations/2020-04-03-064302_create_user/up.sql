CREATE TABLE user (
  id SERIAL PRIMARY KEY,
  username VARCHAR(50) NOT NULL,
  email VARCHAR(254) NOT NULL,
  password VARCHAR(255) NOT NULL,
  api_key VARCHAR(64) UNIQUE,
  github_access_token VARCHAR(255) UNIQUE,
  active BOOLEAN NOT NULL DEFAULT false,
  confirmed_at DATETIME
);
