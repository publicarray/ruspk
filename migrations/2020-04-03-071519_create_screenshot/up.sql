CREATE TABLE screenshot (
    id SERIAL PRIMARY KEY,
    package_id BIGINT UNSIGNED NOT NULL,
    path VARCHAR(100) NOT NULL,
    FOREIGN KEY (package_id) REFERENCES package(id)
);
