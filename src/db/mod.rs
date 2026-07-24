use sea_orm::{Database, DatabaseConnection};

pub async fn connect_to_db() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let database_url = "sqlite://main.db?mode=rwc";
    let db: DatabaseConnection = Database::connect(database_url).await?;
    Ok(db)
}