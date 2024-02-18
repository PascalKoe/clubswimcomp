use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::*,
    Json, Router,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model,
    services::{
        AvailableCompetitionsForRegistrationError, ParticipantDetailsError,
        RegisterForCompetitionsError, RemoveParticipantError, UnregisterFromCompetitionError,
    },
};

use super::{ApiError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_participants))
        .route("/", post(add_participant))
        .route("/:participant_id", get(participant_details))
        .route("/:participant_id", delete(remove_participant))
        .route(
            "/:participant_id/registrations/available-competitions",
            get(available_competitions_for_registration),
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

impl From<&ParticipantDetailsError> for StatusCode {
    fn from(err: &ParticipantDetailsError) -> Self {
        match err {
            ParticipantDetailsError::ParticipantDoesNotExist => Self::NOT_FOUND,
            ParticipantDetailsError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&RemoveParticipantError> for StatusCode {
    fn from(err: &RemoveParticipantError) -> Self {
        match err {
            RemoveParticipantError::ParticipantDoesNotExist => Self::NOT_FOUND,
            RemoveParticipantError::ParticipantHasRegistrations => Self::BAD_REQUEST,
            RemoveParticipantError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&AvailableCompetitionsForRegistrationError> for StatusCode {
    fn from(err: &AvailableCompetitionsForRegistrationError) -> Self {
        match err {
            AvailableCompetitionsForRegistrationError::ParticipantDoesNotExist => Self::NOT_FOUND,
            AvailableCompetitionsForRegistrationError::RepositoryError(_) => {
                Self::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<&RegisterForCompetitionsError> for StatusCode {
    fn from(err: &RegisterForCompetitionsError) -> Self {
        match err {
            RegisterForCompetitionsError::ParticipantDoesNotExist => Self::NOT_FOUND,
            RegisterForCompetitionsError::CompetitionDoesNotExist => Self::NOT_FOUND,
            RegisterForCompetitionsError::AlreadyRegistered => Self::BAD_REQUEST,
            RegisterForCompetitionsError::NotEligible => Self::BAD_REQUEST,
            RegisterForCompetitionsError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&UnregisterFromCompetitionError> for StatusCode {
    fn from(err: &UnregisterFromCompetitionError) -> Self {
        match err {
            UnregisterFromCompetitionError::RegistrationDoesNotExist => Self::NOT_FOUND,
            UnregisterFromCompetitionError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

#[instrument(skip(state))]
async fn list_participants(
    State(state): State<AppState>,
) -> Result<Json<Vec<model::Participant>>, ApiError> {
    let participant_service = state.participant_service();
    let participants = participant_service.list_participants().await?;
    Ok(Json(participants))
}

#[instrument(skip(state))]
async fn participant_details(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<model::ParticipantDetails>, ApiError> {
    let participant_service = state.participant_service();
    let participant_details = participant_service
        .participant_details(participant_id)
        .await?;

    Ok(Json(participant_details))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct AddParticipantBody {
    first_name: String,
    last_name: String,
    gender: model::Gender,
    birthday: NaiveDate,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct AddParticipantResponse {
    participant_id: Uuid,
}

#[instrument(skip(state))]
async fn add_participant(
    State(state): State<AppState>,
    Json(p): Json<AddParticipantBody>,
) -> Result<Json<AddParticipantResponse>, ApiError> {
    let participant_service = state.participant_service();
    let participant_id = participant_service
        .add_participant(&p.first_name, &p.last_name, p.gender, p.birthday)
        .await?;

    Ok(Json(AddParticipantResponse { participant_id }))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RemoveParticipantParameters {
    force_delete: Option<bool>,
}

#[instrument(skip(state))]
async fn remove_participant(
    Path(participant_id): Path<Uuid>,
    Query(p): Query<RemoveParticipantParameters>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    let participant_service = state.participant_service();
    participant_service
        .remove_participant(participant_id, p.force_delete.unwrap_or_default())
        .await
        .map_err(ApiError::from)
}

#[instrument(skip(state))]
async fn available_competitions_for_registration(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<model::Competition>>, ApiError> {
    let participant_service = state.participant_service();
    let competitions = participant_service
        .available_competitions_for_registration(participant_id)
        .await?;

    Ok(Json(competitions))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct RegisterForCompetitionBody {
    competition_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct RegisterForCompetitionResponse {
    registration_id: Uuid,
}

#[instrument(skip(state))]
async fn register_for_competition(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(b): Json<RegisterForCompetitionBody>,
) -> Result<Json<RegisterForCompetitionResponse>, ApiError> {
    let participant_service = state.participant_service();
    let registration_id = participant_service
        .register_for_competition(participant_id, b.competition_id)
        .await?;

    Ok(Json(RegisterForCompetitionResponse { registration_id }))
}

#[instrument(skip(state))]
async fn unregister_from_competition(
    Path((_participant_id, registration_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    let participant_service = state.participant_service();
    participant_service
        .unregister_from_competition(registration_id)
        .await
        .map_err(ApiError::from)
}
