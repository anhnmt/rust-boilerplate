use crate::config::database::Database;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::path::Path;

pub async fn database_connection(db: Database) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .connect(db.url.as_str())
        .await?;
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Read migrations from a local folder: ./migrations
    let m = Migrator::new(Path::new("./migrations")).await?;
    // Run the migrations
    m.run(pool).await?;
    Ok(())
}