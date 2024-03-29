CREATE TABLE IF NOT EXISTS users (
    user_id BIGINT UNSIGNED PRIMARY KEY NOT NULL DEFAULT (uuid_short()),
    google_id VARCHAR(39) UNIQUE NOT NULL,
    user_name VARCHAR(100),
    profile_icon_path TEXT,
    is_public BOOLEAN NOT NULL DEFAULT (0),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_google_id(google_id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE IF NOT EXISTS friends_relationship (
    source BIGINT UNSIGNED NOT NULL,
    destination BIGINT UNSIGNED NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (source, destination),
    CONSTRAINT fk_src FOREIGN KEY (source) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_dist FOREIGN KEY (destination) REFERENCES users(user_id) ON DELETE CASCADE,
    INDEX idx_source(source),
    INDEX idx_destination(destination)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE IF NOT EXISTS organizations (
    organization_id BIGINT UNSIGNED PRIMARY KEY NOT NULL DEFAULT (uuid_short()),
    organization_name VARCHAR(100) NOT NULL,
    description VARCHAR(255),
    is_public BOOLEAN NOT NULL DEFAULT (0),
    owner BIGINT UNSIGNED NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_owner FOREIGN KEY (owner) REFERENCES users(user_id),
    INDEX idx_org_name(organization_name)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE IF NOT EXISTS users_organizations (
    user_id BIGINT UNSIGNED NOT NULL,
    organization_id BIGINT UNSIGNED NOT NULL,
    edit_permission BOOLEAN NOT NULL DEFAULT (0),
    join_status VARCHAR(10) NOT NULL,
    PRIMARY KEY (user_id, organization_id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_org_id FOREIGN KEY (organization_id) REFERENCES organizations(organization_id) ON DELETE CASCADE,
    INDEX idx_status(join_status)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE IF NOT EXISTS want_items (
    item_id BIGINT UNSIGNED PRIMARY KEY NOT NULL DEFAULT (uuid_short()),
    having_organization_id BIGINT UNSIGNED NOT NULL,
    url TEXT,
    title VARCHAR(100),
    memo VARCHAR(500),
    created_by BIGINT UNSIGNED,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_having_org_id FOREIGN KEY (having_organization_id) REFERENCES organizations(organization_id) ON DELETE CASCADE,
    CONSTRAINT fk_created_by FOREIGN KEY (created_by) REFERENCES users(user_id) ON DELETE
    SET NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE IF NOT EXISTS good_users_items (
    user_id BIGINT UNSIGNED NOT NULL,
    item_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (user_id, item_id),
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    CONSTRAINT fk_good_user_id FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT fk_item_id FOREIGN KEY (item_id) REFERENCES want_items(item_id) ON DELETE CASCADE
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;