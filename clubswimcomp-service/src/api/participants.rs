use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    routing::*,
    Json,
};
use clubswimcomp_types::{api, model};
use tracing::instrument;
use uuid::Uuid;

use crate::services::{
    AvailableCompetitionsForRegistrationError, ParticipantCertificateError,
    ParticipantDetailsError, ParticipantRegistrationCardsError, ParticipantScoreboardError,
    RegisterForCompetitionsError, RemoveParticipantError, UnregisterFromCompetitionError,
};

use super::{ApiError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_participants))
        .route("/", post(add_participant))
        .route("/:participant_id", get(participant_details))
        .route("/:participant_id", delete(remove_participant))
        .route("/:participant_id/scoreboard", get(participant_scoreboard))
        .route("/:participant_id/certificate", get(participant_certificate))
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
        .route(
            "/:participant_id/registrations/cards",
            get(registration_cards),
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

impl From<&ParticipantRegistrationCardsError> for StatusCode {
    fn from(err: &ParticipantRegistrationCardsError) -> Self {
        match err {
            ParticipantRegistrationCardsError::ParticipantDoesNotExist => Self::NOT_FOUND,
            ParticipantRegistrationCardsError::PdfGenerationFailed(_) => {
                Self::INTERNAL_SERVER_ERROR
            }
            ParticipantRegistrationCardsError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&ParticipantScoreboardError> for StatusCode {
    fn from(err: &ParticipantScoreboardError) -> Self {
        match err {
            ParticipantScoreboardError::ParticipantDoesNotExist => Self::NOT_FOUND,
            ParticipantScoreboardError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&ParticipantCertificateError> for StatusCode {
    fn from(err: &ParticipantCertificateError) -> Self {
        match err {
            ParticipantCertificateError::ParticipantDoesNotExist => Self::NOT_FOUND,
            ParticipantCertificateError::PdfGenerationFailed(_) => Self::INTERNAL_SERVER_ERROR,
            ParticipantCertificateError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
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

#[instrument(skip(state))]
async fn add_participant(
    State(state): State<AppState>,
    Json(p): Json<api::AddParticipantBody>,
) -> Result<Json<api::AddParticipantResponse>, ApiError> {
    let participant_service = state.participant_service();
    let participant_id = participant_service
        .add_participant(
            &p.first_name,
            &p.last_name,
            p.gender,
            p.birthday,
            p.group_id,
        )
        .await?;

    Ok(Json(api::AddParticipantResponse { participant_id }))
}

#[instrument(skip(state))]
async fn remove_participant(
    Path(participant_id): Path<Uuid>,
    Query(p): Query<api::RemoveParticipantParameters>,
    State(state): State<AppState>,
) -> Result<(), ApiError> {
    let participant_service = state.participant_service();
    participant_service
        .remove_participant(participant_id, p.force_delete.unwrap_or_default())
        .await
        .map_err(ApiError::from)
}

#[instrument(skip(state))]
async fn participant_scoreboard(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<model::ParticipantScoreboard>, ApiError> {
    let score_service = state.score_service();
    let scoreboard = score_service.participant_scoreboard(participant_id).await?;

    Ok(Json(scoreboard))
}

#[instrument(skip(state))]
async fn participant_certificate(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(HeaderMap, Vec<u8>), ApiError> {
    let score_service = state.score_service();
    let certificate = score_service
        .participant_certificate(participant_id)
        .await?;

    let file_name = format!("{participant_id}-certificate.pdf");

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.append(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{file_name}\"")
            .parse()
            .unwrap(),
    );

    Ok((headers, certificate))
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

#[instrument(skip(state))]
async fn register_for_competition(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(b): Json<api::RegisterForCompetitionBody>,
) -> Result<Json<api::RegisterForCompetitionResponse>, ApiError> {
    let participant_service = state.participant_service();
    let registration_id = participant_service
        .register_for_competition(participant_id, b.competition_id)
        .await?;

    Ok(Json(api::RegisterForCompetitionResponse {
        registration_id,
    }))
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

#[instrument(skip(state))]
async fn registration_cards(
    Path(participant_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(HeaderMap, Vec<u8>), ApiError> {
    let registration_card_service = state.registration_card_service();
    let registration_cards = registration_card_service
        .participants_registration_cards(participant_id)
        .await
        .map_err(ApiError::from)?;

    let file_name = format!("{participant_id}-cards.pdf");

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.append(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{file_name}\"")
            .parse()
            .unwrap(),
    );

    Ok((headers, registration_cards))
}
