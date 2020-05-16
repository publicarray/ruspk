CREATE TABLE version_service_dependency (
    version_id BIGINT UNSIGNED,
    package_id BIGINT UNSIGNED,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (package_id) REFERENCES package(id),
    PRIMARY KEY (version_id, package_id)

);
