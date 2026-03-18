use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> sqlx::Result<()> {
    let database_url = "sqlite:chump-chassis.db";
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(())
}