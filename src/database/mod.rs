
pub(crate) mod checks;
pub(crate) mod queries;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tokio::runtime::Runtime;

use crate::{utils::environment, SqlxError};

pub(crate) fn start() -> Result<SqlitePool, SqlxError> {
    let pool = Runtime::new()?.block_on(connect())?;
    Runtime::new()?.block_on(migrate(&pool))?;
    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

async fn connect() -> Result<SqlitePool, SqlxError> {
    let options = connect_options().await;

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options?)
        .await
}

async fn connect_options() -> Result<SqliteConnectOptions, SqlxError> {
    let db_url = if let Ok(url) = environment::database_url() {
        url
    } else {
        format!("sqlite://wakalaka.db")
    };
    let db_filename = db_url.replace("sqlite://", "");

    let connect_options = SqliteConnectOptions::new()
        .filename(db_filename)
        .create_if_missing(true);
    Ok(connect_options)
}
