use leptos::prelude::*;
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

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::prelude::*;
    use sqlx::SqlitePool;

    pub fn pool() -> Result<SqlitePool, ServerFnError> {
        // use leptos_axum::extract;
        // In a real app, we'd use a Layer to provide the pool.
        // For simplicity here, we'll try to get it from the Axum state or a global.
        // Leptos 0.7 often uses `expect_context` or similar.
        use crate::server::state::AppState;
        let state = expect_context::<AppState>();
        Ok(state.pool.clone())
    }
}

// #[server]
// #[cfg(feature = "ssr")]
// pub async fn save_contact(
pub async fn SaveContact(
    name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    let pool = ssr::pool()?;

    tracing::info!("Saving contact submission from: {}", email);

    sqlx::query!(
        "INSERT INTO contact_submissions (name, email, message) VALUES (?, ?, ?)",
        name,
        email,
        message
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}
