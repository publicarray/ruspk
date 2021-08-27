CREATE TABLE description (
    version_id BIGINT UNSIGNED NOT NULL,
    language_id BIGINT UNSIGNED NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    PRIMARY KEY (version_id, language_id)
);
