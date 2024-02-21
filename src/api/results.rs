use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::*,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::services::{AddRegistrationResultError, RemoveRegistrationResultError};

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new()
        .route("/", post(add_registration_result))
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct EnterResultBody {
    pub registration_id: Uuid,
    pub disqualified: bool,
    pub time_millis: u32,
}

#[instrument(skip(state))]
async fn add_registration_result(
    State(state): State<AppState>,
    Json(b): Json<EnterResultBody>,
) -> Result<(), ApiError> {
    let result_service = state.result_service();
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
    let registration_service = state.result_service();
    registration_service
        .remove_registration_result(registration_id)
        .await
        .map_err(ApiError::from)
}
