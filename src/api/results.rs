use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::*,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{ApiResponse, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new().route("/", post(enter_result))
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct EnterResultBody {
    pub registration_id: Uuid,
    pub disqualified: bool,
    pub time_millis: u32,
}

async fn enter_result(
    State(state): State<AppState>,
    Json(b): Json<EnterResultBody>,
) -> ApiResponse<()> {
    let result_service = state.result_service();
    match result_service
        .enter_result_for_registration(b.registration_id, b.disqualified, b.time_millis)
        .await
    {
        Ok(Some(())) => Ok((StatusCode::CREATED, ())),
        Ok(None) => Err((
            StatusCode::BAD_REQUEST,
            "The registration does not exist or already has a result".to_string(),
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
