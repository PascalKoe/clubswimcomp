use anyhow::{Context, Result};
use clubswimcomp_types::model;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::db;

#[derive(Debug, Error)]
pub enum AddRegistrationResultError {
    #[error("The registration already has a result")]
    ResultAlreadyExists,

    #[error("The registration does not exist")]
    RegistrationDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RemoveRegistrationResultError {
    #[error("The registration does not exist")]
    RegistrationDoesNotExist,

    #[error("The registration does not have any result")]
    RegistrationHasNoResult,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RegistrationDetailsError {
    #[error("The registration does not exist")]
    RegistrationDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

pub struct RegistrationService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
}

impl RegistrationService {
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

    /// Enter results for a registration.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration
    /// - `disqualified` - `true` is the participant is disqualified, `false`
    ///   otherwise.
    /// - `time_millis` - The result time of the participant in milliseconds.
    #[instrument(skip(self))]
    pub async fn add_result_for_registration(
        &self,
        registration_id: Uuid,
        disqualified: bool,
        time_millis: u32,
    ) -> Result<(), AddRegistrationResultError> {
        tracing::debug!("Ensuring the registration actually exists");
        self.registration_repo
            .registration_by_id(registration_id)
            .await
            .context("Failed to search registration by id in repository")?
            .ok_or(AddRegistrationResultError::RegistrationDoesNotExist)?;

        tracing::debug!("Ensuring no result already exists for registration");
        if self
            .registration_repo
            .result_for_registration(registration_id)
            .await
            .context("Failed to search for registration result in repository")?
            .is_some()
        {
            return Err(AddRegistrationResultError::ResultAlreadyExists);
        }

        tracing::debug!("Creating registration result in repository");
        self.registration_repo
            .create_registration_result(registration_id, time_millis as _, disqualified)
            .await
            .context("Failed to create result for registration in repository")
            .map_err(AddRegistrationResultError::from)
    }

    /// Remove a result for a registration.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration
    #[instrument(skip(self))]
    pub async fn remove_registration_result(
        &self,
        registration_id: Uuid,
    ) -> Result<(), RemoveRegistrationResultError> {
        tracing::debug!("Ensuring the registration actually exists");
        self.registration_repo
            .registration_by_id(registration_id)
            .await
            .context("Failed to search registration by id in repository")?
            .ok_or(RemoveRegistrationResultError::RegistrationDoesNotExist)?;

        tracing::debug!("Trying to delete the registration result in the repository");
        self.registration_repo
            .delete_result_for_registration(registration_id)
            .await
            .context("Failed to delete registration result in repository")?
            .ok_or(RemoveRegistrationResultError::RegistrationHasNoResult)
    }

    /// Get the details of a registration.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration
    #[instrument(skip(self))]
    pub async fn registration_details(
        &self,
        registration_id: Uuid,
    ) -> Result<model::RegistrationDetails, RegistrationDetailsError> {
        tracing::debug!("Fetching the registration from the repository");
        let registration = self
            .registration_repo
            .registration_by_id(registration_id)
            .await
            .context("Failed to search registration by id in repository")?
            .ok_or(RegistrationDetailsError::RegistrationDoesNotExist)?;

        tracing::debug!("Fetching participant for the registration from the repository");
        let participant = self
            .participant_repo
            .participant_by_id(registration.participant_id)
            .await
            .context("Failed to fetch participant for registration from repository")?
            .ok_or(anyhow::anyhow!(
                "Repository has reference to participant but participant could not be found"
            ))
            .map(model::Participant::from)?;

        tracing::debug!("Fetching competition for the registration from the repository");
        let competition = self
            .competition_repo
            .competition_by_id(registration.competition_id)
            .await
            .context("Failed to fetch competition for registration from repository")?
            .ok_or(anyhow::anyhow!(
                "Repository has reference to competition but competition could not be found"
            ))
            .map(model::Competition::from)?;

        tracing::debug!("Fetching result for registration from the repository");
        let result = self
            .registration_repo
            .result_for_registration(registration_id)
            .await
            .context("Failed to fetch result for registration from repository")?
            .map(model::RegistrationResult::from);

        Ok(model::RegistrationDetails {
            id: registration_id,
            participant,
            competition,
            result,
        })
    }
}
