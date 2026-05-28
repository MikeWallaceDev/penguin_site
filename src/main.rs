#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::sqlite::SqlitePoolOptions;
    use techno_penguin::app::*;
    use techno_penguin::server::state::AppState;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "techno_penguin=debug,tower_http=debug,leptos_axum=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Techno Penguin server...");
    tracing::debug!("Environment variables loaded");

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:techno_penguin.db?mode=rwc".to_string());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Could not connect to database");

    tracing::info!("Database connection established");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Could not run migrations");

    tracing::info!("Migrations applied successfully");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let state = AppState {
        pool,
        leptos_options: leptos_options.clone(),
    };

    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let state = state.clone();
            move || shell(state.leptos_options.clone())
        })
        // .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .fallback(leptos_axum::file_and_error_handler(shell))
        // .with_state(state);
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(feature = "ssr")]
fn shell(options: leptos::prelude::LeptosOptions) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;
    use leptos_meta::MetaTags;
    use techno_penguin::app::App;

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options=options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main; use src/lib.rs for hydration
}
