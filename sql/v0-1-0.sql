CREATE TABLE IF NOT EXISTS users
(
    user_id     VARCHAR(30) PRIMARY KEY NOT NULL,
    user_name   VARCHAR(100),
    friend_id   UUID NOT NULL,
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS friend_relationship
(
    friend_relationship_id INTEGER PRIMARY KEY NOT NULL AUTO_INCREMENT,
    source      UUID NOT NULL,
    destination UUID NOT NULL,

    FOREIGN KEY (source) REFERENCES users(friend_id) ON UPDATE CASCADE ON DELETE RESTRICT
);
