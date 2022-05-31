use anyhow::Context;
use sqlx::MySqlConnection;

pub(crate) mod organization;
pub(crate) mod user;

pub(crate) async fn create_uuid_short(conn: &mut MySqlConnection) -> anyhow::Result<u64> {
    let uuid = sqlx::query!("SELECT uuid_short() as uuid;")
        .fetch_one(conn)
        .await
        .context("Failed to uuid_short()")?
        .uuid;

    Ok(uuid)
}
