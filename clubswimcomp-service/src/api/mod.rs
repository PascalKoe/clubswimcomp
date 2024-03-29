use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;

use crate::infra::typst_compiler::TypstCompiler;
use crate::services::{
    CompetitionService, GroupService, ParticipantService, RegistrationCardService,
    RegistrationService, ScoreService, ServiceRepositoryError,
};
use crate::{db, infra, Config};

mod competitions;
mod event;
mod groups;
mod participants;
mod registrations;

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
    group_repo: db::groups::Repository,

    typst_compiler: infra::typst_compiler::TypstCompiler,
}

impl AppState {
    pub fn new(config: Config, pool: db::DatabasePool) -> Self {
        let typst_compiler = TypstCompiler::new(config.typst_bin, config.typst_assets);

        Self {
            participant_repo: db::participants::Repository::new(pool.clone()),
            registration_repo: db::registrations::Repository::new(pool.clone()),
            competition_repo: db::competitions::Repository::new(pool.clone()),
            group_repo: db::groups::Repository::new(pool.clone()),

            typst_compiler,
        }
    }

    pub fn participant_service(&self) -> ParticipantService {
        ParticipantService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.group_repo.clone(),
        )
    }

    pub fn registration_card_service(&self) -> RegistrationCardService {
        RegistrationCardService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.typst_compiler.clone(),
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

    pub fn group_service(&self) -> GroupService {
        GroupService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.group_repo.clone(),
            self.typst_compiler.clone(),
        )
    }

    pub fn score_service(&self) -> ScoreService {
        ScoreService::new(
            self.participant_repo.clone(),
            self.registration_repo.clone(),
            self.competition_repo.clone(),
            self.group_repo.clone(),
            self.typst_compiler.clone(),
        )
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/participants", participants::router())
        .nest("/registrations", registrations::router())
        .nest("/competitions", competitions::router())
        .nest("/event", event::router())
        .nest("/groups", groups::router())
}
