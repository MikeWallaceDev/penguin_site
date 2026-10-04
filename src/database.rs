use sqlx::sqlite::SqlitePoolOptions;

static DB: std::sync::OnceLock<sqlx::SqlitePool> = std::sync::OnceLock::new();

async fn create_pool() -> sqlx::SqlitePool {
    let db_url = std::env::var("DATABASE_URL").expect("no database url found in ENV vars");

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    tracing::info!("Database connection established");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Could not run migrations");

    tracing::info!("Migrations applied successfully");

    pool
}

pub async fn init_db() -> Result<(), ()> {
    DB.set(create_pool().await).map_err(|_| ())
}

pub fn get_db() -> &'static sqlx::SqlitePool {
    DB.get().expect("database initialized")
}
