use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::*,
    Json,
};
use clubswimcomp_types::{api, model};
use tracing::instrument;
use uuid::Uuid;

use crate::services::{
    AddRegistrationResultError, RegistrationDetailsError, RemoveRegistrationResultError,
};

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new()
        .route("/", post(add_registration_result))
        .route("/:registration_id", get(registration_details))
        .route("/:registration_id", delete(delete_result))
}

impl From<&AddRegistrationResultError> for StatusCode {
    fn from(err: &AddRegistrationResultError) -> Self {
        match err {
            AddRegistrationResultError::ResultAlreadyExists => StatusCode::BAD_REQUEST,
            AddRegistrationResultError::RegistrationDoesNotExist => StatusCode::NOT_FOUND,
            AddRegistrationResultError::RepositoryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&RemoveRegistrationResultError> for StatusCode {
    fn from(err: &RemoveRegistrationResultError) -> Self {
        match err {
            RemoveRegistrationResultError::RegistrationDoesNotExist => StatusCode::NOT_FOUND,
            RemoveRegistrationResultError::RegistrationHasNoResult => StatusCode::NOT_FOUND,
            RemoveRegistrationResultError::RepositoryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&RegistrationDetailsError> for StatusCode {
    fn from(err: &RegistrationDetailsError) -> Self {
        match err {
            RegistrationDetailsError::RegistrationDoesNotExist => StatusCode::NOT_FOUND,
            RegistrationDetailsError::RepositoryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[instrument(skip(state))]
async fn registration_details(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
) -> Result<Json<model::RegistrationDetails>, ApiError> {
    let registration_service = state.registration_service();
    let registration_details = registration_service
        .registration_details(registration_id)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(registration_details))
}

#[instrument(skip(state))]
async fn add_registration_result(
    State(state): State<AppState>,
    Json(b): Json<api::EnterResultBody>,
) -> Result<(), ApiError> {
    let result_service = state.registration_service();
    result_service
        .add_result_for_registration(b.registration_id, b.disqualified, b.time_millis)
        .await
        .map_err(ApiError::from)
}

#[instrument(skip(state))]
async fn delete_result(
    State(state): State<AppState>,
    Path(registration_id): Path<Uuid>,
) -> Result<(), ApiError> {
    let registration_service = state.registration_service();
    registration_service
        .remove_registration_result(registration_id)
        .await
        .map_err(ApiError::from)
}
