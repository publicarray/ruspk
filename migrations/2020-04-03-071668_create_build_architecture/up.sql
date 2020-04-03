CREATE TABLE build_architecture (
    build_id BIGINT UNSIGNED,
    architecture_id BIGINT UNSIGNED,
    FOREIGN KEY (build_id) REFERENCES build(id),
    FOREIGN KEY (architecture_id) REFERENCES architecture(id),
    PRIMARY KEY (build_id, architecture_id)
);
