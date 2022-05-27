INSERT IGNORE INTO users (google_id, user_name)
VALUES ('1', '高海千歌');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('2', '桜内梨子');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('3', '松浦果南');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('4', '黒澤ダイヤ');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('5', '渡辺曜');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('6', '津島善子');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('7', '国木田花丸');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('8', '小原鞠莉');
INSERT IGNORE INTO users (google_id, user_name)
VALUES ('9', '黒澤ルビィ');
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '2'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '3'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '4'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '5'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '6'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '7'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '8'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '9'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '2'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO friends_relationship (source, destination)
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '5'
        ),
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
select google_id,
    user_id,
    user_name
FROM users
    INNER JOIN (
        SELECT follow AS user_id
        FROM (
                SELECT source AS follower
                FROM friends_relationship
                WHERE destination = 99799836211019858
            ) AS follower
            INNER JOIN (
                SELECT destination AS follow
                FROM friends_relationship
                WHERE source = 99799836211019858
            ) AS follow ON follower.follower = follow.follow
    ) AS friend_list USING(user_id);