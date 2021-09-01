CREATE TABLE user_role (
    user_id BIGINT UNSIGNED,
    role_id BIGINT UNSIGNED,
    FOREIGN KEY (user_id) REFERENCES role(id),
    FOREIGN KEY (role_id) REFERENCES user(id),
    PRIMARY KEY (user_id, role_id)
);
