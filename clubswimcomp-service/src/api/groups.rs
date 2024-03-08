use axum::{
    extract::*,
    http::{header, HeaderMap, StatusCode},
    routing::*,
    Json,
};
use clubswimcomp_types::{
    api::{AddGroupRequest, AddGroupResponse},
    model,
};
use tracing::instrument;
use uuid::Uuid;

use crate::services::GroupResultError;

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new()
        .route("/", get(list_groups))
        .route("/", post(add_group))
        .route("/:group_id", get(group_details))
}

impl From<&GroupResultError> for StatusCode {
    fn from(err: &GroupResultError) -> Self {
        match err {
            GroupResultError::GroupDoesNotExist => StatusCode::NOT_FOUND,
            GroupResultError::RepositoryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[instrument(skip(state))]
async fn list_groups(State(state): State<AppState>) -> Result<Json<Vec<model::Group>>, ApiError> {
    let group_service = state.group_service();
    let groups = group_service.list_groups().await?;
    Ok(Json(groups))
}

#[instrument(skip(state))]
async fn group_details(
    State(state): State<AppState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<model::GroupDetails>, ApiError> {
    let group_service = state.group_service();
    let group_details = group_service.group_details(group_id).await?;
    Ok(Json(group_details))
}

#[instrument(skip(state))]
async fn add_group(
    State(state): State<AppState>,
    Json(req): Json<AddGroupRequest>,
) -> Result<Json<AddGroupResponse>, ApiError> {
    let group_service = state.group_service();
    let group_id = group_service.add_group(req.name).await?;
    Ok(Json(AddGroupResponse { group_id }))
}
