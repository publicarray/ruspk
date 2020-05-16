CREATE TABLE service (
  id SERIAL PRIMARY KEY,
  code VARCHAR(30) NOT NULL UNIQUE
);

INSERT INTO service (code) VALUES
    ('apache-web'),
    ('mysql'),
    ('php_disable_safe_exec_dir'),
    ('ssh');
