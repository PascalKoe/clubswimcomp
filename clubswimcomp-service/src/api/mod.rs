use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;

use crate::db;
use crate::services::{
    CompetitionService, ParticipantService, RegistrationCardService, RegistrationService,
    ServiceRepositoryError,
};

mod competitions;
mod event;
mod participants;
mod results;

struct ApiError {
    status_code: StatusCode,
    message: String,
    internal_message: String,
}

impl ApiError {
    pub fn with_message(status_code: StatusCode, message: String) -> Self {
        Self {
            status_code,
            message,
            internal_message: String::new(),
        }
    }

    pub fn with_internal_message(status_code: StatusCode, internal_message: String) -> Self {
        Self {
            status_code,
            message: String::new(),
            internal_message,
        }
    }
}

impl<T> From<T> for ApiError
where
    T: std::error::Error,
    for<'a> &'a T: Into<StatusCode>,
{
    fn from(err: T) -> Self {
        let status_code: StatusCode = (&err).into();
        if status_code.is_server_error() {
            ApiError::with_internal_message(status_code, err.to_string())
        } else {
            ApiError::with_message(status_code, err.to_string())
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        if !self.internal_message.is_empty() {
            tracing::error!(self.internal_message, "API has internal error message");
        }
        (self.status_code, self.message).into_response()
    }
}

impl From<&ServiceRepositoryError> for StatusCode {
    fn from(err: &ServiceRepositoryError) -> Self {
        match err {
            ServiceRepositoryError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
}

impl AppState {
    pub fn new(pool: db::DatabasePool) -> Self {
        Self {
            participant_repo: db::participants::Repository::new(pool.clone()),
            registration_repo: db::registrations::Repository::new(pool.clone()),
            competition_repo: db::competitions::Repository::new(pool.clone()),
        }
    }

    pub fn participant_service(&self) -> ParticipantService {
        ParticipantService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
        )
    }

    pub fn registration_card_service(&self) -> RegistrationCardService {
        RegistrationCardService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
        )
    }

    pub fn registration_service(&self) -> RegistrationService {
        RegistrationService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
        )
    }

    pub fn competition_service(&self) -> CompetitionService {
        CompetitionService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
        )
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/participants", participants::router())
        .nest("/results", results::router())
        .nest("/competitions", competitions::router())
        .nest("/event", event::router())
}
