use axum::{extract::*, http::StatusCode, routing::*, Router};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{model, services::AddCompetitionError};

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new().route("/", post(add_competition))
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
