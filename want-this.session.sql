SELECT *
FROM users
    INNER JOIN users_organizations ON users.user_id = users_organizations.user_id
WHERE join_status = "Pending";