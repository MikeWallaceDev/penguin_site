#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use techno_penguin::app::*;
    use techno_penguin::components::App;
    use techno_penguin::database;

    let tracing_sub = tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        // .init()
        .finish();

    tracing::subscriber::set_global_default(tracing_sub).expect("Failed to set subscriber");

    tracing::info!("Starting Techno Penguin server...");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    tracing::debug!("Environment variables loaded");

    // Init the pool into static
    database::init_db()
        .await
        .expect("problem during initialization of the database");

    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        // .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .fallback(leptos_axum::file_and_error_handler(shell))
        // .with_state(state);
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::DEBUG),
                )
                .on_failure(
                    tower_http::trace::DefaultOnFailure::new().level(tracing::Level::DEBUG),
                ),
        )
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on http://{}", &addr);
    log!("listening on http://{}", &addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main; use src/lib.rs for hydration
}
