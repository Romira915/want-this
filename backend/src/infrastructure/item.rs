use anyhow::Context;
use sqlx::MySqlConnection;

use crate::domain::entity::item::NewItem;

use super::take_n_str;

const MAX_LEN_ITEM_TITLE: usize = 100;
const MAX_LEN_ITEM_MEMO: usize = 500;
pub(crate) struct InternalItemRepository;

impl InternalItemRepository {
    pub(crate) async fn add_new_item(
        conn: &mut MySqlConnection,
        new_item: &NewItem,
    ) -> anyhow::Result<u64> {
        let title = if let Some(title) = new_item.title.as_ref() {
            Some(take_n_str(title, MAX_LEN_ITEM_TITLE))
        } else {
            None
        };
        let memo = if let Some(memo) = new_item.memo.as_ref() {
            Some(take_n_str(memo, MAX_LEN_ITEM_MEMO))
        } else {
            None
        };

        let id = sqlx::query!(
            "INSERT INTO want_items (having_organization_id, url, title, memo)
            VALUES (?, ?, ?, ?);",
            new_item.having_organization_id,
            new_item.url,
            title,
            memo
        )
        .execute(conn)
        .await
        .context("Failed to add_new_item")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_url(
        conn: &mut MySqlConnection,
        org_id: u64,
        url: &str,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "UPDATE want_items SET url = ? WHERE having_organization_id = ?;",
            url,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_url")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_title(
        conn: &mut MySqlConnection,
        org_id: u64,
        title: &str,
    ) -> anyhow::Result<u64> {
        let title = take_n_str(title, MAX_LEN_ITEM_TITLE);
        let id = sqlx::query!(
            "UPDATE want_items SET title = ? WHERE having_organization_id = ?;",
            title,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_title")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn update_memo(
        conn: &mut MySqlConnection,
        org_id: u64,
        memo: &str,
    ) -> anyhow::Result<u64> {
        let memo = take_n_str(memo, MAX_LEN_ITEM_TITLE);
        let id = sqlx::query!(
            "UPDATE want_items SET memo = ? WHERE having_organization_id = ?;",
            memo,
            org_id
        )
        .execute(conn)
        .await
        .context("Failed to update_memo")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn entry_good(
        conn: &mut MySqlConnection,
        user_id: u64,
        item_id: u64,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "INSERT INTO good_users_items (user_id, item_id)
            VALUES (?, ?);",
            user_id,
            item_id
        )
        .execute(conn)
        .await
        .context("Failed to entry_good")?
        .last_insert_id();

        Ok(id)
    }

    pub(crate) async fn cancel_good(
        conn: &mut MySqlConnection,
        user_id: u64,
        item_id: u64,
    ) -> anyhow::Result<u64> {
        let id = sqlx::query!(
            "DELETE FROM good_users_items WHERE user_id = ? AND item_id = ?;",
            user_id,
            item_id
        )
        .execute(conn)
        .await
        .context("Failed to cancel_good")?
        .last_insert_id();

        Ok(id)
    }
}
