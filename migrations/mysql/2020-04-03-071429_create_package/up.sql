CREATE TABLE package (
    id SERIAL PRIMARY KEY,
    author_user_id BIGINT UNSIGNED DEFAULT NULL,
    name VARCHAR(50) NOT NULL UNIQUE,
    insert_date DATETIME,
    FOREIGN KEY (author_user_id) REFERENCES user(id) ON DELETE SET NULL
);
