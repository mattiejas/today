use axum::{extract::State, routing::post, Json, Router};

use lib::{
    cqrs::event::ClanEventCommand,
    error::{AppError, AppResult, SubError},
};

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/event", post(create_event))
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequest {
    name: String,
    clan_id: i32,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventResponse {
    id: String,
}

async fn create_event(
    State(state): State<AppState>,
    Json(body): Json<CreateEventRequest>,
) -> AppResult<Json<CreateEventResponse>> {
    let command = ClanEventCommand::CreateEvent {
        name: body.name,
        clan_id: body.clan_id,
    };

    let id = uuid::Uuid::new_v4().to_string();
    let event = state.cqrs.execute(&id, command).await;

    match event {
        Ok(_) => {}
        Err(e) => {
            return Err(AppError::AnyhowError(SubError::new(format!(
                "Failed to create event: {}",
                e
            ))))
        }
    }

    Ok(Json(CreateEventResponse { id: id.to_string() }))
}
