CREATE TABLE icon (
    id SERIAL PRIMARY KEY,
    version_id BIGINT UNSIGNED NOT NULL,
    size ENUM('72', '120', '256') NOT NULL,
    path VARCHAR(100) NOT NULL,
    FOREIGN KEY (version_id) REFERENCES version(id)
);
