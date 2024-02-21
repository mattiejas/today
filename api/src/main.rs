use async_graphql::Schema;
use axum::{routing::get, Extension, Router};
use clap::{command, Parser};
use lib::{
    config::Config,
    db::{self},
    services::{self},
    state::AppState,
};

use tower::ServiceBuilder;
use tower_http::{trace::TraceLayer, ServiceBuilderExt};
use tower_http::cors::{Any, CorsLayer};

mod routes;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long, default_value = "localhost")]
    host: String,

    #[arg(short, long, default_value = "3001")]
    port: u16,

    #[arg(long, default_value = "debug")]
    log_level: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // setup logging
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(format!("api={},tower_http=debug", args.log_level))
        .init();

    // setup database
    let db_pool = db::new_pool().await.expect("Failed to connect to database");

    let config = Config::new().expect("Failed to read config");

    // register services
    let services = services::AppServices::new(config.clone()).await;

    let state = AppState {
        db_pool: db_pool.clone(),
        config: config.clone(),
    };

    // setup graphql
    let schema = Schema::build(
        lib::graphql::QueryRoot::default(),
        lib::graphql::MutationRoot::default(),
        async_graphql::EmptySubscription,
    )
    .data(state.clone())
    .data(services.clone())
    .finish();

    // setup middleware
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any);

    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .layer(Extension(schema))
        .layer(Extension(services))
        .compression();

    // setup routes
    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route(
            "/graphql",
            get(routes::graphql::graphql_playground).post(routes::graphql::graphql_handler),
        )
        .merge(routes::auth::router())
        .layer(middleware)
        .with_state(state);

    // get port from args
    let url = format!("{}:{}", args.host, args.port);

    let listener = tokio::net::TcpListener::bind(&url)
        .await
        .expect(format!("Failed to bind to {}:{}", args.host, args.port).as_str());

    log::info!(
        "Listening on http://{}",
        listener.local_addr().expect("Failed to get local address")
    );

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server")
}
