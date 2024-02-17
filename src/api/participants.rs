use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
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
        .route("/:participant_id", delete(remove_participant))
}

async fn list_participants(
    State(state): State<AppState>,
) -> ApiResponse<Json<Vec<model::Participant>>> {
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
) -> ApiResponse<Json<model::ParticipantDetails>> {
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
) -> ApiResponse<Json<model::Participant>> {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveParticipantParameters {
    cascade: Option<bool>,
}

async fn remove_participant(
    Path(participant_id): Path<Uuid>,
    Query(params): Query<RemoveParticipantParameters>,
    State(state): State<AppState>,
) -> ApiResponse<&'static str> {
    let participant_service = state.participant_service();
    match participant_service
        .remove_participant(participant_id, params.cascade.unwrap_or_default())
        .await
    {
        Ok(Some(true)) => Ok((StatusCode::OK, "")),
        Ok(Some(false)) => Ok((
            StatusCode::BAD_REQUEST,
            "Participant can't be deleted as there are still registrations.",
        )),
        Ok(None) => Ok((StatusCode::NOT_FOUND, "Participant does not exist")),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
