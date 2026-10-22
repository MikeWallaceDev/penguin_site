#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
   use axum::extract::DefaultBodyLimit;
   use axum::http::header;
   use axum::{Router, routing::get};
   use dotenv::EnvLoader;
   use leptos::prelude::*;
   use leptos_axum::{LeptosRoutes, generate_route_list};
   use std::time::Duration;
   use techno_penguin::app::*;
   use techno_penguin::components::App;
   use tower_governor::{
      GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
   };
   use tower_http::set_header::SetResponseHeaderLayer;
   use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

   tracing_subscriber::registry()
      .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
      .with(tracing_subscriber::fmt::layer())
      .init();

   tracing::info!("Starting Techno Penguin server...");

   // Load development environtment variables from .env file
   tracing::debug!("Loading environment variables");
   EnvLoader::new().load().ok();

   // This reads the Leptos configuration from Cargo.toml
   tracing::debug!("Loading Leptos environment variables");
   let conf = get_configuration(None).unwrap();
   let addr = conf.leptos_options.site_addr;
   let leptos_options = conf.leptos_options;

   let db_pool = prepare_database().await;

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
      .leptos_routes_with_context(
         &leptos_options,
         leptos_routes,
         {
            let db_pool = db_pool.clone();
            move || provide_context(db_pool.clone()) // Inject SQLx pool into Leptos context
         },
         {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
         },
      )
      .layer(governor_layer); // layer only applies to this router

   // build our application with a route
   let app = Router::new()
      .merge(health_router)
      .merge(app_router)
      .fallback(leptos_axum::file_and_error_handler(shell))
      .layer(
         tower_http::trace::TraceLayer::new_for_http()
            .make_span_with(tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::DEBUG))
            .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::DEBUG))
            .on_response(tower_http::trace::DefaultOnResponse::new().level(tracing::Level::DEBUG))
            .on_failure(tower_http::trace::DefaultOnFailure::new().level(tracing::Level::DEBUG)),
      )
      // Prevent site from being embedded in iframes (Clickjacking)
      .layer(SetResponseHeaderLayer::overriding(
         header::X_FRAME_OPTIONS,
         header::HeaderValue::from_static("DENY"),
      ))
      // Prevent browsers from MIME-sniffing away from the declared content-type
      .layer(SetResponseHeaderLayer::overriding(
         header::X_CONTENT_TYPE_OPTIONS,
         header::HeaderValue::from_static("nosniff"),
      ))
      // Control referrer data sent in outgoing links
      .layer(SetResponseHeaderLayer::overriding(
         header::REFERRER_POLICY,
         header::HeaderValue::from_static("strict-origin-when-cross-origin"),
      ))
      .layer(SetResponseHeaderLayer::overriding(
         header::HeaderName::from_static("permissions-policy"),
         header::HeaderValue::from_static("geolocation=(), microphone=(), camera=()"),
      ))
      // Limit maximum request body size (e.g., 256 KB is more than enough for contact forms)
      .layer(DefaultBodyLimit::max(256 * 1024))
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

#[tracing::instrument]
#[cfg(feature = "ssr")]
async fn prepare_database() -> sqlx::SqlitePool {
   use sqlx::sqlite::SqliteConnectOptions;
   use sqlx::sqlite::SqlitePoolOptions;
   use std::str::FromStr;

   tracing::info!("Creating Database Pool");

   let db_url = std::env::var("DATABASE_URL").expect("No database url found in ENV vars");

   let connection_options = SqliteConnectOptions::from_str(&db_url)
      .expect("Invalid database URL")
      .create_if_missing(true);

   let pool = SqlitePoolOptions::new()
      .max_connections(5)
      .connect_with(connection_options)
      .await
      .expect("Could not connect to database");

   tracing::debug!("Database connection established");

   sqlx::migrate!("./migrations")
      .run(&pool)
      .await
      .expect("Could not run migrations");

   tracing::debug!("Migrations applied successfully");

   pool
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
   // no client-side main; use src/lib.rs for hydration
}
