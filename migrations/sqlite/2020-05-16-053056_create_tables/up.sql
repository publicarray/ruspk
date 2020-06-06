CREATE TABLE user (
  id INTEGER NOT NULL PRIMARY KEY,
  username VARCHAR(50) NOT NULL,
  email VARCHAR(254) NOT NULL,
  password VARCHAR(255) NOT NULL,
  api_key VARCHAR(64) UNIQUE,
  github_access_token VARCHAR(255) UNIQUE,
  active BOOLEAN NOT NULL DEFAULT false,
  confirmed_at DATETIME
);
CREATE TABLE role (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR(50) NOT NULL UNIQUE,
  description VARCHAR(255) NOT NULL
);

INSERT INTO role (name, description) VALUES
    ('admin','Administrator'),
    ('package_admin','Package Administrator'),
    ('developer','Developer');
CREATE TABLE architecture (
  id INTEGER NOT NULL PRIMARY KEY,
  code VARCHAR(20) NOT NULL UNIQUE
);

INSERT INTO architecture (code) VALUES
    ('noarch'),
    ('ppc824x'),
    ('ppc854x'),
    ('ppc853x'),
    ('88f628x'),
    ('x86'),
    ('bromolow'),
    ('cedarview'),
    ('qoriq'),
    ('armada370'),
    ('armadaxp'),
    ('evansport'),
    ('comcerto2k'),
    ('avoton'),
    ('armada375'),
    ('alpine'),
    ('powerpc'),
    ('alpine4k'),
    ('monaco'),
    ('braswell'),
    ('armada38x'),
    ('broadwell'),
    ('x86_64'),
    ('grantley'),
    ('apollolake'),
    ('rtd1296'),
    ('denverton'),
    ('dockerx64'),
    ('broadwellnk'),
    ('kvmx64'),
    ('ipq806x'),
    ('northstarplus'),
    ('hi3535'),
    ('armada37xx'),
    ('aarch64'),
    ('dakota'),
    ('purley');

CREATE TABLE language (
  id INTEGER NOT NULL PRIMARY KEY,
  code VARCHAR(3) NOT NULL UNIQUE,
  name VARCHAR(50) NOT NULL
);

INSERT INTO language (code, name) VALUES
    ('enu', 'English'),
    ('cht', 'Traditional Chinese'),
    ('chs', 'Simplified Chinese'),
    ('krn', 'Korean'),
    ('ger', 'German'),
    ('fre', 'French'),
    ('ita', 'Italian'),
    ('spn', 'Spanish'),
    ('jpn', 'Japanese'),
    ('dan', 'Danish'),
    ('nor', 'Norwegian'),
    ('sve', 'Swedish'),
    ('nld', 'Dutch'),
    ('rus', 'Russian'),
    ('plk', 'Polish'),
    ('ptb', 'Brazilian Portuguese'),
    ('ptg', 'European Portuguese'),
    ('hun', 'Hungarian'),
    ('trk', 'Turkish'),
    ('csy', 'Czech');

CREATE TABLE firmware (
  id INTEGER NOT NULL PRIMARY KEY,
  version VARCHAR(3) NOT NULL UNIQUE,
  build INT NOT NULL
);

INSERT INTO firmware (version, build) VALUES
    ('2.0', 731),
    ('2.1', 844),
    ('2.2', 942),
    ('2.3', 1139),
    ('3.0', 1337),
    ('3.1', 1594),
    ('3.2', 1922),
    ('4.0', 2198),
    ('4.1', 2636),
    ('4.2', 3202),
    ('4.3', 3776),
    ('5.0', 4458),
    ('5.1', 5004),
    ('5.2', 5644),
    ('6.0', 7321),
    ('6.1', 15047),
    ('6.2', 22259),
    ('1.1', 6931),
    ('6.2', 23739),
    ('1.2', 1757),
    ('1.2', 7742);

CREATE TABLE service (
  id INTEGER NOT NULL PRIMARY KEY,
  code VARCHAR(30) NOT NULL UNIQUE
);

INSERT INTO service (code) VALUES
    ('apache-web'),
    ('mysql'),
    ('php_disable_safe_exec_dir'),
    ('ssh'),
    ('Docker');
CREATE TABLE package (
    id INTEGER NOT NULL PRIMARY KEY,
    author_user_id BIGINT UNSIGNED DEFAULT NULL,
    name VARCHAR(50) NOT NULL UNIQUE,
    insert_date DATETIME,
    FOREIGN KEY (author_user_id) REFERENCES user(id) ON DELETE SET NULL
);
CREATE TABLE user_role (
    user_id BIGINT UNSIGNED,
    role_id BIGINT UNSIGNED,
    FOREIGN KEY (user_id) REFERENCES role(id),
    FOREIGN KEY (role_id) REFERENCES user(id),
    PRIMARY KEY (user_id, role_id)
);
CREATE TABLE screenshot (
    id INTEGER NOT NULL PRIMARY KEY,
    package_id BIGINT UNSIGNED NOT NULL,
    path VARCHAR(100) NOT NULL,
    FOREIGN KEY (package_id) REFERENCES package(id)
);
CREATE TABLE version (
    id INTEGER NOT NULL PRIMARY KEY,
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
CREATE TABLE package_user_maintainer (
    package_id BIGINT UNSIGNED,
    user_id BIGINT UNSIGNED,
    FOREIGN KEY (package_id) REFERENCES package(id),
    FOREIGN KEY (user_id) REFERENCES user(id),
    PRIMARY KEY (package_id, user_id)
);
CREATE TABLE version_service_dependency (
    version_id BIGINT UNSIGNED,
    package_id BIGINT UNSIGNED,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (package_id) REFERENCES package(id),
    PRIMARY KEY (version_id, package_id)

);
CREATE TABLE build (
    id INTEGER NOT NULL PRIMARY KEY,
    version_id BIGINT UNSIGNED NOT NULL,
    firmware_id BIGINT UNSIGNED NOT NULL,
    publisher_user_id BIGINT UNSIGNED,
    checksum VARCHAR(32),
    extract_size INT NOT NULL,
    path VARCHAR(100) NOT NULL,
    md5 VARCHAR(32) NOT NULL,
    insert_date DATETIME NOT NULL,
    active BOOLEAN,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (firmware_id) REFERENCES firmware(id)
);
CREATE TABLE icon (
    id INTEGER NOT NULL PRIMARY KEY,
    version_id BIGINT UNSIGNED NOT NULL,
    size INT NOT NULL,
    path VARCHAR(100) NOT NULL,
    FOREIGN KEY (version_id) REFERENCES version(id)
);
CREATE TABLE description (
    version_id BIGINT UNSIGNED NOT NULL,
    language_id BIGINT UNSIGNED NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    PRIMARY KEY (version_id, language_id)
);
CREATE TABLE displayname (
    version_id BIGINT UNSIGNED NOT NULL,
    language_id BIGINT UNSIGNED NOT NULL,
    displayname VARCHAR(50) NOT NULL,
    FOREIGN KEY (version_id) REFERENCES version(id),
    FOREIGN KEY (language_id) REFERENCES language(id),
    PRIMARY KEY (version_id, language_id)
);
CREATE TABLE build_architecture (
    build_id BIGINT UNSIGNED,
    architecture_id BIGINT UNSIGNED,
    FOREIGN KEY (build_id) REFERENCES build(id),
    FOREIGN KEY (architecture_id) REFERENCES architecture(id),
    PRIMARY KEY (build_id, architecture_id)
);
CREATE TABLE download (
    id INTEGER NOT NULL PRIMARY KEY,
    build_id BIGINT UNSIGNED NOT NULL,
    architecture_id BIGINT UNSIGNED NOT NULL,
    firmware_build INT NOT NULL,
    ip_address VARCHAR(46) NOT NULL,
    user_agent VARCHAR(255),
    date DATETIME NOT NULL,
    FOREIGN KEY (build_id) REFERENCES build(id),
    FOREIGN KEY (architecture_id) REFERENCES architecture(id)
);
