pub(crate) mod checks;
pub(crate) mod queries;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

use crate::{utils::environment, SqlxThrowable, Throwable};

pub(crate) async fn start() -> Throwable<SqlitePool> {
    //no runtime
    let pool = connect().await?;

    migrate(&pool).await?;

    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> Throwable<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

async fn connect() -> SqlxThrowable<SqlitePool> {
    let options = connect_options().await;

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options?)
        .await
}

async fn connect_options() -> SqlxThrowable<SqliteConnectOptions> {
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
