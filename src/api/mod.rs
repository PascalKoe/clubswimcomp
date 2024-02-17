use axum::http::StatusCode;
use axum::Router;

use crate::db;
use crate::services::ParticipantService;

mod participants;

type ApiResponse<T> = Result<(StatusCode, T), (StatusCode, String)>;

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
}

pub fn routes() -> Router<AppState> {
    Router::new().nest("/participants", participants::router())
}
