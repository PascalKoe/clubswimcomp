use axum::{extract::*, http::StatusCode, routing::*};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    model,
    services::{AddCompetitionError, DeleteCompetitionError},
};

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new()
        .route("/", get(list_competitions))
        .route("/", post(add_competition))
        .route("/:competition_id", delete(delete_competition))
}

impl From<&AddCompetitionError> for StatusCode {
    fn from(err: &AddCompetitionError) -> Self {
        match err {
            AddCompetitionError::InvalidDistance => Self::BAD_REQUEST,
            AddCompetitionError::SameCompetitionExists => Self::BAD_REQUEST,
            AddCompetitionError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<&DeleteCompetitionError> for StatusCode {
    fn from(err: &DeleteCompetitionError) -> Self {
        match err {
            DeleteCompetitionError::CompetitionDoesNotExist => Self::NOT_FOUND,
            DeleteCompetitionError::CompetitionHasRegistrations => Self::BAD_REQUEST,
            DeleteCompetitionError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

#[instrument(skip(state))]
async fn list_competitions(
    State(state): State<AppState>,
) -> Result<Json<Vec<model::Competition>>, ApiError> {
    let competition_service = state.competition_service();
    let competitions = competition_service.list_competitions().await?;
    Ok(Json(competitions))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddCompetitionRequest {
    pub gender: model::Gender,
    pub stroke: model::Stroke,
    pub distance: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct AddCompetitionResponse {
    competition_id: Uuid,
}

#[instrument(skip(state))]
async fn add_competition(
    State(state): State<AppState>,
    Json(b): Json<AddCompetitionRequest>,
) -> Result<Json<AddCompetitionResponse>, ApiError> {
    let competition_service = state.competition_service();
    let competition_id = competition_service
        .add_competition(b.distance, b.gender, b.stroke)
        .await?;

    Ok(Json(AddCompetitionResponse { competition_id }))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DeleteCompetitionParams {
    force_delete: Option<bool>,
}

#[instrument(skip(state))]
async fn delete_competition(
    State(state): State<AppState>,
    Path(competition_id): Path<Uuid>,
    Query(params): Query<DeleteCompetitionParams>,
) -> Result<(), ApiError> {
    let competition_service = state.competition_service();
    competition_service
        .delete_competition(competition_id, params.force_delete.unwrap_or_default())
        .await?;

    Ok(())
}
