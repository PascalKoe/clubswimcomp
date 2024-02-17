use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::*,
    Json, Router,
};
use uuid::Uuid;

use crate::model;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_participants))
        .route("/:participant_id", get(participant_details))
}

async fn list_participants(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<model::Participant>>), StatusCode> {
    let participant_service = state.participant_service();
    match participant_service
        .list_participants()
        .await
        .context("Failed to handle list participants request")
    {
        Ok(p) => Ok((StatusCode::OK, Json(p))),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn participant_details(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<model::ParticipantDetails>), StatusCode> {
    let participant_service = state.participant_service();
    match participant_service
        .participant_details(participant_id)
        .await
        .context("Failed to handle participant details request")
    {
        Ok(Some(p)) => Ok((StatusCode::OK, Json(p))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
