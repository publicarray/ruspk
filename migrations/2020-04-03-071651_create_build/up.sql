CREATE TABLE build (
    id SERIAL PRIMARY KEY,
    package_id BIGINT UNSIGNED NOT NULL,
    firmware_id BIGINT UNSIGNED NOT NULL,
    publisher_user_id BIGINT UNSIGNED,
    checksum VARCHAR(32),
    extract_size INT NOT NULL,
    path VARCHAR(100) NOT NULL,
    md5 VARCHAR(32) NOT NULL,
    insert_date DATETIME NOT NULL,
    active BOOLEAN,
    FOREIGN KEY (package_id) REFERENCES package(id),
    FOREIGN KEY (firmware_id) REFERENCES firmware(id)
);
