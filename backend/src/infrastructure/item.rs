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
}
