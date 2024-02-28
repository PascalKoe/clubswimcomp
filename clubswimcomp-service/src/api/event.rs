use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    routing::*,
};
use tracing::instrument;

use crate::services::EventRegistrationCardsError;

use super::{ApiError, AppState};

pub fn router() -> axum::Router<super::AppState> {
    Router::new().route("/cards", get(registration_cards))
}

impl From<&EventRegistrationCardsError> for StatusCode {
    fn from(err: &EventRegistrationCardsError) -> Self {
        match err {
            EventRegistrationCardsError::PdfGenerationFailed(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EventRegistrationCardsError::RepositoryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[instrument(skip(state))]
async fn registration_cards(
    State(state): State<AppState>,
) -> Result<(HeaderMap, Vec<u8>), ApiError> {
    let registration_card_service = state.registration_card_service();
    let cards = registration_card_service.all_registration_cards().await?;

    let file_name = format!("event-registration-cards.pdf");

    let mut headers = HeaderMap::new();
    headers.append(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.append(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{file_name}\"")
            .parse()
            .unwrap(),
    );

    Ok((headers, cards))
}
