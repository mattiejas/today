use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract,
    response::{Html, IntoResponse},
};
use lib::graphql::AppSchema;
use tracing::{Instrument, Level};

pub(crate) async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/ws"),
    ))
}

pub(crate) async fn graphql_handler(
    schema: extract::Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let span = tracing::span!(Level::INFO, "graphql_execution");

    log::info!("Processing GraphQL request");

    let response = async move { schema.execute(req.into_inner()).await }
        .instrument(span.clone())
        .await;

    log::info!("Processing GraphQL request finished");

    response.into()
}
