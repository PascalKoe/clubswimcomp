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

#[derive(Debug, Error)]
pub enum DeleteCompetitionError {
    #[error("The competition does not exist")]
    CompetitionDoesNotExist,

    #[error("The competition can not be deleted while there are still registrations for it")]
    CompetitionHasRegistrations,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum CompetitionDetailsError {
    #[error("The competition does not exist")]
    CompetitionDoesNotExist,

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

    #[instrument(skip(self))]
    pub async fn delete_competition(
        &self,
        competition_id: Uuid,
        force_delete: bool,
    ) -> Result<(), DeleteCompetitionError> {
        tracing::debug!("Ensuring the competition actually exists");
        let _competition = self
            .competition_repo
            .competition_by_id(competition_id)
            .await
            .context("Failed to fetch competition from repository")?
            .ok_or(DeleteCompetitionError::CompetitionDoesNotExist)?;

        tracing::debug!("Fetching registrations for competition from repository");
        let registrations = self
            .registration_repo
            .registrations_for_competition(competition_id)
            .await
            .context("Failed to fetch registrations for competition from repository")?;
        if !registrations.is_empty() && !force_delete {
            tracing::debug!(
                "Tried to delete competition with registrations and without force delete flag"
            );
            return Err(DeleteCompetitionError::CompetitionHasRegistrations);
        } else if !registrations.is_empty() {
            tracing::debug!("Force deleting the registrations of the competition");
            for registration in registrations.iter() {
                tracing::debug!(registration_id = ?registration.id, "Deleting result for registration from respository");
                self.registration_repo
                    .delete_result_for_registration(registration.id)
                    .await
                    .context("Failed to delete registration result in repository")?;

                tracing::debug!("Deleting registration from respository");
                self.registration_repo
                    .delete_registration(registration.id)
                    .await
                    .context("Failed to delete registration in repository")?;
            }
            tracing::debug!("Deleted all registrations for competition from repository");
        }

        tracing::debug!("Deleting competition from repository");
        self.competition_repo
            .delete_competition(competition_id)
            .await
            .context("Failed to delete competition in repository")?;

        Ok(())
    }

    pub async fn competition_details(
        &self,
        competition_id: Uuid,
    ) -> Result<model::CompetitionDetails, CompetitionDetailsError> {
        tracing::debug!("Fetching competition from repository");
        let competition = self
            .competition_repo
            .competition_by_id(competition_id)
            .await
            .context("Failed to fetch competition from repository")?
            .map(model::Competition::from)
            .ok_or(CompetitionDetailsError::CompetitionDoesNotExist)?;

        tracing::debug!("Fetching registrations for competition from repository");
        let db_registrations = self
            .registration_repo
            .registrations_for_competition(competition_id)
            .await
            .context("Failed to fetch registrations for competition from repository")?;

        let mut registrations = Vec::with_capacity(db_registrations.len());
        for registration in db_registrations.into_iter() {
            tracing::debug!(registration_id = ?registration.id, "Fetching result for registration from repository");
            let result = self
                .registration_repo
                .result_for_registration(registration.id)
                .await
                .context("Failed to fetch result for registration from repository")?
                .map(model::RegistrationResult::from);

            tracing::debug!(participant_id = ?registration.participant_id, "Fetching participant for registration");
            let participant = self
                .participant_repo
                .participant_by_id(registration.participant_id)
                .await
                .context("Failed to fetch participant from repository")?
                .map(model::Participant::from)
                .ok_or(anyhow::anyhow!("Participant is reference in the registration but does not exist in the repository"))?;

            registrations.push(model::CompetitionRegistration {
                id: registration.id,
                participant,
                result,
            });
        }

        let results_pending = registrations.iter().any(|r| r.result.is_none());

        Ok(model::CompetitionDetails {
            competition,
            results_pending,
            registrations,
        })
    }
}
