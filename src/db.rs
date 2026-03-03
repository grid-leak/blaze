use sea_orm::{Database, DatabaseConnection};
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init(url: &str) -> Result<(), sea_orm::DbErr> {
    let db = Database::connect(url).await?;
    DB.set(db).expect("DB already initialized");
    Ok(())
}

pub fn db() -> &'static DatabaseConnection {
    DB.get().expect("DB not initialized")
}
