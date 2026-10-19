#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use std::time::Duration;
    use techno_penguin::app::*;
    use techno_penguin::components::App;
    use techno_penguin::database;
    use tower_governor::{
        governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor, GovernorLayer,
    };
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Techno Penguin server...");

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    tracing::debug!("Environment variables loaded");

    // Init the pool into static
    database::init_db()
        .await
        .expect("problem during initialization of the database");

    // Rate limiting: 5 submissions per IP every 1 minute
    // We limit server function endpoints under /api
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .period(Duration::from_secs(1 * 60))
            .burst_size(5)
            .key_extractor(PeerIpKeyExtractor)
            .finish()
            .unwrap(),
    );
    let governor_layer = GovernorLayer::new(governor_conf);

    let leptos_routes = generate_route_list(App);

    // 1. Public route for healthcheck that should NOT be rate limited
    let health_router = Router::new().route("/health", get(|| async { "OK" }));

    // 2. App routes that NEED rate limiting
    let app_router = Router::new()
        .leptos_routes(&leptos_options, leptos_routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .layer(governor_layer); // layer only applies to this router

    // build our application with a route
    let app = Router::new()
        .merge(health_router)
        .merge(app_router)
        .fallback(leptos_axum::file_and_error_handler(shell))
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

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main; use src/lib.rs for hydration
}
