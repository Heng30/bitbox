use super::pool;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct AddressBook {
    pub uuid: String,
    pub data: String,
}

pub async fn new() -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS address_book (
             id INTEGER PRIMARY KEY,
             uuid TEXT NOT NULL UNIQUE,
             data TEXT NOT NULL)",
    )
    .execute(&pool())
    .await?;

    Ok(())
}

pub async fn delete(uuid: &str) -> Result<()> {
    sqlx::query("DELETE FROM address_book WHERE uuid=?")
        .bind(uuid)
        .execute(&pool())
        .await?;
    Ok(())
}

pub async fn insert(uuid: &str, data: &str) -> Result<()> {
    sqlx::query("INSERT INTO address_book (uuid, data) VALUES (?, ?)")
        .bind(uuid)
        .bind(data)
        .execute(&pool())
        .await?;
    Ok(())
}

pub async fn select_all() -> Result<Vec<AddressBook>> {
    Ok(
        sqlx::query_as::<_, AddressBook>("SELECT * FROM address_book")
            .fetch_all(&pool())
            .await?,
    )
}
