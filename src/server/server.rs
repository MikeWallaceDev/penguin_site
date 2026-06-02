#[cfg(feature = "ssr")]
pub mod state {
    //    use axum::extract::FromRef;
    use leptos::prelude::LeptosOptions;
    use sqlx::SqlitePool;

    // #[derive(FromRef, Debug, Clone)]
    #[derive(Debug, Clone)]
    pub struct AppState {
        pub pool: SqlitePool,
        pub leptos_options: LeptosOptions,
    }
}
