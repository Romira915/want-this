-- users
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
-- uesrs
-- friends
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
-- friends
-- org
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        owner
    )
VALUES (
        'Aqours',
        'LoveLive！Sunshine!!',
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        owner
    )
VALUES (
        '虹ヶ咲学園スクールアイドル同好会',
        'LoveLive！虹ヶ咲学園スクールアイドル同好会',
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'Liella !',
        'LoveLive！スーパースター ! !',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_0',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_1',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_2',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_3',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_4',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_5',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_6',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_7',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_8',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
INSERT IGNORE INTO organizations (
        organization_name,
        description,
        is_public,
        owner
    )
VALUES (
        'org_9',
        'debug data',
        1,
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        )
    );
-- org
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '1'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        1,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '2'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '3'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '4'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '5'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        1,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '6'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '7'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '8'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO users_organizations (
        user_id,
        organization_id,
        edit_permission,
        join_status
    )
VALUES (
        (
            SELECT user_id
            FROM users
            WHERE google_id = '9'
        ),
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        0,
        'Joined'
    );
INSERT IGNORE INTO want_items (
        having_organization_id,
        url
    )
VALUES (
        (
            SELECT organization_id
            FROM organizations
            WHERE organization_name = 'Aqours'
        ),
        'https://store.steampowered.com/app/1604030/V_Rising/'
    );