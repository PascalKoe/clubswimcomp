use axum::{extract::*, http::StatusCode, routing::*};
use clubswimcomp_types::{api, model};
use tracing::instrument;
use uuid::Uuid;

use crate::services::{
    AddCompetitionError, CompetitionDetailsError, CompetitionScoreboardError,
    DeleteCompetitionError,
};

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new()
        .route("/", get(list_competitions))
        .route("/", post(add_competition))
        .route("/:competition_id", get(competition_details))
        .route("/:competition_id", delete(delete_competition))
        .route("/:competition_id/scoreboard", get(competition_scoreboard))
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

impl From<&CompetitionDetailsError> for StatusCode {
    fn from(err: &CompetitionDetailsError) -> Self {
        match err {
            CompetitionDetailsError::CompetitionDoesNotExist => Self::NOT_FOUND,
            CompetitionDetailsError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
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

impl From<&CompetitionScoreboardError> for StatusCode {
    fn from(err: &CompetitionScoreboardError) -> Self {
        match err {
            CompetitionScoreboardError::CompetitionDoesNotExist => Self::NOT_FOUND,
            CompetitionScoreboardError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
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

#[instrument(skip(state))]
async fn add_competition(
    State(state): State<AppState>,
    Json(b): Json<api::AddCompetitionRequest>,
) -> Result<Json<api::AddCompetitionResponse>, ApiError> {
    let competition_service = state.competition_service();
    let competition_id = competition_service
        .add_competition(b.distance, b.gender, b.stroke, b.target_time)
        .await?;

    Ok(Json(api::AddCompetitionResponse { competition_id }))
}

#[instrument(skip(state))]
async fn competition_details(
    State(state): State<AppState>,
    Path(competition_id): Path<Uuid>,
) -> Result<Json<model::CompetitionDetails>, ApiError> {
    let competition_service = state.competition_service();
    let competition_details = competition_service
        .competition_details(competition_id)
        .await?;

    Ok(Json(competition_details))
}

#[instrument(skip(state))]
async fn delete_competition(
    State(state): State<AppState>,
    Path(competition_id): Path<Uuid>,
    Query(params): Query<api::DeleteCompetitionParams>,
) -> Result<(), ApiError> {
    let competition_service = state.competition_service();
    competition_service
        .delete_competition(competition_id, params.force_delete.unwrap_or_default())
        .await?;

    Ok(())
}

#[instrument(skip(state))]
async fn competition_scoreboard(
    State(state): State<AppState>,
    Path(competition_id): Path<Uuid>,
) -> Result<Json<model::CompetitionScoreboard>, ApiError> {
    let competition_service = state.competition_service();
    let scoreboard = competition_service
        .competition_scoreboard(competition_id)
        .await?;

    Ok(Json(scoreboard))
}
