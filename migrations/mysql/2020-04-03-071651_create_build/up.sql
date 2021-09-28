CREATE TABLE build (
    id SERIAL PRIMARY KEY,
    version_id BIGINT UNSIGNED NOT NULL,
    firmware_id BIGINT UNSIGNED NOT NULL,
    publisher_user_id BIGINT UNSIGNED,
    checksum VARCHAR(32),
    extract_size INT,
    path VARCHAR(100) NOT NULL UNIQUE,
    md5 VARCHAR(32),
    insert_date DATETIME NOT NULL,
    active BOOLEAN,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (firmware_id) REFERENCES firmware(id)
);
