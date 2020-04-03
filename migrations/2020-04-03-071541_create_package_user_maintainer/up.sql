CREATE TABLE package_user_maintainer (
    package_id BIGINT UNSIGNED,
    user_id BIGINT UNSIGNED,
    FOREIGN KEY (package_id) REFERENCES package(id),
    FOREIGN KEY (user_id) REFERENCES user(id),
    PRIMARY KEY (package_id, user_id)
);
