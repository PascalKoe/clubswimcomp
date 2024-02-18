use anyhow::{Context, Result};
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{db, model};

use super::ServiceRepositoryError;

pub struct CompetitionService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
}

#[derive(Debug, Error)]
pub enum AddCompetitionError {
    #[error("Distance must be multiple of 25 meters")]
    InvalidDistance,

    #[error("There is already the same competition")]
    SameCompetitionExists,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

impl CompetitionService {
    pub fn new(
        participant_repo: db::participants::Repository,
        registration_repo: db::registrations::Repository,
        competition_repo: db::competitions::Repository,
    ) -> Self {
        Self {
            participant_repo,
            registration_repo,
            competition_repo,
        }
    }

    #[instrument(skip(self))]
    pub async fn list_competitions(
        &self,
    ) -> Result<Vec<model::Competition>, ServiceRepositoryError> {
        tracing::debug!("Loading competitions from repository");
        let competitions = self
            .competition_repo
            .all_competitions()
            .await
            .context("Failed to fetch competitions from repository")?
            .into_iter()
            .map(model::Competition::from)
            .collect();

        Ok(competitions)
    }

    #[instrument(skip(self))]
    pub async fn add_competition(
        &self,
        distance: u32,
        gender: model::Gender,
        stroke: model::Stroke,
    ) -> Result<Uuid, AddCompetitionError> {
        tracing::debug!("Checking if the provided distance is valid");
        if distance % 25 != 0 {
            tracing::debug!("The provided distance is not a multiple of 25 meters");
            return Err(AddCompetitionError::InvalidDistance);
        }

        tracing::debug!("Checking if there already exists the same competition");
        let already_exists = !self
            .competition_repo
            .search_competition(
                Some(gender.into()),
                Some(stroke.into()),
                Some(distance as _),
            )
            .await
            .context("Failed to fetch competitions from repository")?
            .is_empty();

        if already_exists {
            tracing::debug!("The same competition already exists");
            return Err(AddCompetitionError::SameCompetitionExists);
        }

        tracing::debug!("Creating the competition in the repository");
        let competition_id = self
            .competition_repo
            .create_competition(gender.into(), stroke.into(), distance as _)
            .await
            .context("Failed to create competition in repository")?;

        Ok(competition_id)
    }
}
