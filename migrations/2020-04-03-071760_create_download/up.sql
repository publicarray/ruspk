CREATE TABLE download (
    id SERIAL PRIMARY KEY,
    build_id BIGINT UNSIGNED NOT NULL,
    architecture_id BIGINT UNSIGNED NOT NULL,
    firmware_build INT NOT NULL,
    ip_address VARCHAR(46) NOT NULL,
    user_agent VARCHAR(255),
    date DATETIME NOT NULL,
    FOREIGN KEY (build_id) REFERENCES build(id),
    FOREIGN KEY (architecture_id) REFERENCES architecture(id)
);
