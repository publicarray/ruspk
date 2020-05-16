CREATE TABLE role (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL UNIQUE,
  description VARCHAR(255) NOT NULL
);

INSERT INTO role (name, description) VALUES
    ('admin','Administrator'),
    ('package_admin','Package Administrator'),
    ('developer','Developer');
