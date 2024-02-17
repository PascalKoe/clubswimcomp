use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::*,
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model;

use super::{ApiResponse, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_participants))
        .route("/", post(add_participant))
        .route("/:participant_id", get(participant_details))
}

async fn list_participants(State(state): State<AppState>) -> ApiResponse<Vec<model::Participant>> {
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

#[axum::debug_handler]
async fn participant_details(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> ApiResponse<model::ParticipantDetails> {
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct AddParticipantBody {
    first_name: String,
    last_name: String,
    gender: model::Gender,
    birthday: NaiveDate,
}

async fn add_participant(
    State(state): State<AppState>,
    Json(p): Json<AddParticipantBody>,
) -> ApiResponse<model::Participant> {
    let participant_service = state.participant_service();
    match participant_service
        .add_participant(&p.first_name, &p.last_name, p.gender, p.birthday)
        .await
        .context("Failed to handle add participant request")
    {
        Ok(p) => Ok((StatusCode::CREATED, Json(p))),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
