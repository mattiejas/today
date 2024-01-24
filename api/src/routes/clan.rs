use axum::{debug_handler, routing::post, Extension, Json, Router};
use lib::{error::AppResult, prisma::clan, services::AppServices};
use serde::Deserialize;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/clan", post(create_clan))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateClanRequest {
    name: String,
    display_name: String,
}

#[debug_handler]
async fn create_clan(
    services: Extension<AppServices>,
    Json(CreateClanRequest { name, display_name }): Json<CreateClanRequest>,
) -> AppResult<Json<clan::Data>> {
    let clan = services.clan.create(&name, &display_name).await?;

    Ok(Json(clan))
}
