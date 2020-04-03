CREATE TABLE version (
    id SERIAL PRIMARY KEY,
    package_id BIGINT UNSIGNED NOT NULL,
    version INT UNSIGNED NOT NULL UNIQUE,
    upstream_version VARCHAR(20) NOT NULL,
    changelog TEXT,
    report_url VARCHAR(255),
    distributor VARCHAR(255),
    distributor_url VARCHAR(255),
    maintainer VARCHAR(255),
    maintainer_url VARCHAR(255),
    dependencies VARCHAR(255),
    conf_dependencies VARCHAR(255),
    conflicts VARCHAR(255),
    conf_conflicts VARCHAR(255),
    install_wizard BOOLEAN,
    upgrade_wizard BOOLEAN,
    startable BOOLEAN,
    license TEXT,
    insert_date DATETIME NOT NULL,
    FOREIGN KEY (package_id) REFERENCES package(id)
);
CREATE UNIQUE INDEX version_package_id_index ON version(package_id,version);
