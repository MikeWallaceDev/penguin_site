#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use techno_penguin::app::*;
    use techno_penguin::server::state::AppState;
    use techno_penguin::server::*;
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

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    // Init the pool into static
    init_db()
        .await
        .expect("problem during initialization of the database");

    let routes = generate_route_list(app::App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
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
    use techno_penguin::app::app::App;

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
