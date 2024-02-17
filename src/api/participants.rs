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
        .route(
            "/:participant_id/registrations/available-competitions",
            get(available_competitions),
        )
        .route(
            "/:participant_id/registrations",
            post(register_for_competition),
        )
        .route(
            "/:participant_id/registrations/:registration_id",
            delete(unregister_from_competition),
        )
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
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
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
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            "Participant does not exist".to_string(),
        )),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
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
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
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
) -> ApiResponse<()> {
    let participant_service = state.participant_service();
    match participant_service
        .remove_participant(participant_id, params.cascade.unwrap_or_default())
        .await
    {
        Ok(Some(true)) => Ok((StatusCode::OK, ())),
        Ok(Some(false)) => Err((
            StatusCode::BAD_REQUEST,
            "Participant can't be deleted as there are still registrations.".to_string(),
        )),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            "Participant does not exist".to_string(),
        )),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
        }
    }
}

async fn available_competitions(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> ApiResponse<Json<Vec<model::Competition>>> {
    let participant_service = state.participant_service();
    match participant_service
        .competitions_available_for_registration(participant_id)
        .await
    {
        Ok(Some(c)) => Ok((StatusCode::OK, Json(c))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            "Participant does not exist".to_string(),
        )),
        Err(e) => {
            tracing::error!("{e:#?}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct RegisterForCompetitionBody {
    competition_id: Uuid,
}

async fn register_for_competition(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(body): Json<RegisterForCompetitionBody>,
) -> ApiResponse<Json<model::ParticipantRegistration>> {
    let participant_service = state.participant_service();
    match participant_service
        .register_for_competition(participant_id, body.competition_id)
        .await
    {
        Ok(Some(r)) => Ok((StatusCode::CREATED, Json(r))),
        Ok(None) => Err((
            StatusCode::BAD_REQUEST,
            "Participant or competition does not exist".to_string(),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )),
    }
}

async fn unregister_from_competition(
    Path((_participant_id, registration_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> ApiResponse<()> {
    let participant_service = state.participant_service();
    match participant_service
        .unregister_from_competition(registration_id)
        .await
    {
        Ok(Some(_)) => Ok((StatusCode::OK, ())),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            "The registration does not exist".to_string(),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )),
    }
}
