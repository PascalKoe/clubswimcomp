use anyhow::{Context, Result};
use chrono::NaiveDate;
use thiserror::Error;
use tracing::instrument;
use uuid::Uuid;

use crate::{db, model};

use super::ServiceRepositoryError;

pub struct ParticipantService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
}

#[derive(Debug, Error)]
pub enum ParticipantDetailsError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RemoveParticipantError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("Participant can't be deleted while still registered to competitions")]
    ParticipantHasRegistrations,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum AvailableCompetitionsForRegistrationError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum RegisterForCompetitionsError {
    #[error("The participant does not exist")]
    ParticipantDoesNotExist,

    #[error("The competition does not exist")]
    CompetitionDoesNotExist,

    #[error("Participant is already registered for the competition")]
    AlreadyRegistered,

    #[error("Participant is not eligible to register for the competition")]
    NotEligible,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum UnregisterFromCompetitionError {
    #[error("The registration does not exist")]
    RegistrationDoesNotExist,

    #[error("The repository ran into an error: {0:#?}")]
    RepositoryError(#[from] anyhow::Error),
}

impl ParticipantService {
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
    pub async fn list_participants(
        &self,
    ) -> Result<Vec<model::Participant>, ServiceRepositoryError> {
        let participants = self
            .participant_repo
            .list_participants()
            .await
            .context("Failed to load participants from repository")?
            .into_iter()
            .map(model::Participant::from)
            .collect();

        Ok(participants)
    }

    #[instrument(skip(self))]
    pub async fn add_participant(
        &self,
        first_name: &str,
        last_name: &str,
        gender: model::Gender,
        birthday: NaiveDate,
    ) -> Result<Uuid, ServiceRepositoryError> {
        self.participant_repo
            .create_participant(first_name, last_name, gender.into(), birthday)
            .await
            .context("Failed to add participant to repository")
            .map_err(ServiceRepositoryError::from)
    }

    /// Fetch the details of the given participant.
    ///
    /// # Parameters:
    /// - `participant_id` - The id of the participant
    #[instrument(skip(self))]
    pub async fn participant_details(
        &self,
        participant_id: Uuid,
    ) -> Result<model::ParticipantDetails, ParticipantDetailsError> {
        tracing::debug!("Ensuring the participant actually exists");
        let participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to load participant from repository")?
            .map(model::Participant::from)
            .ok_or(ParticipantDetailsError::ParticipantDoesNotExist)?;

        tracing::debug!("Loading all registrations for participant");
        let db_registrations = self
            .registration_repo
            .registrations_of_participant(participant_id)
            .await
            .context("Failed to load registrations for participant from repository")?;

        tracing::debug!("Loading the registration details");
        let mut registrations = Vec::with_capacity(db_registrations.len());
        for db_registration in db_registrations.into_iter() {
            tracing::debug!(
                competition_id = ?db_registration.competition_id,
                "Loading competition for registration"
            );
            let competition = self
                .competition_repo
                .competition_by_id(db_registration.competition_id)
                .await
                .context("Failed to load competition for registration from repository")?
                .map(model::Competition::from)
                .context("Competition is referenced in registration but could not be found in repository")?;

            tracing::debug!(
                registration_id = ?db_registration.id,
                "Loading registration result for registration"
            );
            let result = self
                .registration_repo
                .result_for_registration(db_registration.id)
                .await
                .context("Failed to load registration result from repository")?
                .map(model::RegistrationResult::from);

            let registration = model::ParticipantRegistration {
                id: db_registration.id,
                competition,
                result,
            };
            registrations.push(registration);
        }

        Ok(model::ParticipantDetails {
            participant,
            registrations,
        })
    }

    /// Remove a participant from the store.
    ///
    /// Before a participant can be deleted, all registrations of the
    /// participant must be removed. As an alternative they can be removed
    /// automatically by setting `force_delete` to `true`.
    ///
    /// # Parameters
    /// - `participant_id` - The id of the participant that shall be removed.
    /// - `force_delete` - If set to `true`, all registrations for the participant are
    ///   also removed
    #[instrument(skip(self))]
    pub async fn remove_participant(
        &self,
        participant_id: Uuid,
        force_delete: bool,
    ) -> Result<(), RemoveParticipantError> {
        tracing::debug!("Ensuring participant actually exists");
        let _participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to fetch participant from repository")?
            .ok_or(RemoveParticipantError::ParticipantDoesNotExist)?;

        tracing::debug!("Loading registrations for the participant");
        let registrations = self
            .registration_repo
            .registrations_of_participant(participant_id)
            .await
            .context("Failed to fetch registrations for participant from repository")?;

        if !registrations.is_empty() && !force_delete {
            tracing::debug!("There are still registrations left but no force delete is requested");
            return Err(RemoveParticipantError::ParticipantHasRegistrations);
        } else if !registrations.is_empty() && force_delete {
            tracing::debug!("Deleting the remaining registrations for the participant");
            for registration in registrations {
                tracing::debug!(registration_id = ?registration.id, "Deleting result for registration from repository");
                self.registration_repo
                    .delete_result_for_registration(registration.id)
                    .await
                    .context("Failed to delete result for registration from repository")?;

                tracing::debug!(registration_id = ?registration.id, "Deleting registration from repository");
                self.registration_repo
                    .delete_registration(registration.id)
                    .await
                    .context("Failed to delete registration from repository")?;
            }
        }

        tracing::debug!("Deleting participant from repository");
        self.participant_repo
            .delete_participant(participant_id)
            .await
            .context("Failed to delete participant from repository")?;

        Ok(())
    }

    /// Get a list of competitions for which registrations are still available.
    ///
    /// # Parameters:
    /// - `participant_id` - The id of the participant
    #[instrument(skip(self))]
    pub async fn available_competitions_for_registration(
        &self,
        participant_id: Uuid,
    ) -> Result<Vec<model::Competition>, AvailableCompetitionsForRegistrationError> {
        tracing::debug!("Ensuring participant actually exists");
        let participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to load participant from repository")?
            .ok_or(AvailableCompetitionsForRegistrationError::ParticipantDoesNotExist)?;

        tracing::debug!("Loading existing registrations for participant");
        let registrations = self
            .registration_repo
            .registrations_of_participant(participant_id)
            .await
            .context("Failed to load registrations from repository")?;

        tracing::debug!("Loading applicable competitions for the participant");
        let applicable_competitions = self
            .competition_repo
            .search_competition(Some(participant.gender), None, None)
            .await
            .context("Failed to load competitions from repository")?;

        let not_registered_yet = applicable_competitions
            .into_iter()
            .filter(|c| !registrations.iter().any(|r| r.competition_id == c.id))
            .map(model::Competition::from)
            .collect();

        Ok(not_registered_yet)
    }

    /// Register participant for a competition.
    ///
    /// # Parameters:
    /// - `participant_id` - The id of the participant
    /// - `competition_id` - The id of the competition
    #[instrument(skip(self))]
    pub async fn register_for_competition(
        &self,
        participant_id: Uuid,
        competition_id: Uuid,
    ) -> Result<Uuid, RegisterForCompetitionsError> {
        tracing::debug!("Ensuring participant actually exists");
        let participant = self
            .participant_repo
            .participant_by_id(participant_id)
            .await
            .context("Failed to fetch participant from repository")?
            .ok_or(RegisterForCompetitionsError::ParticipantDoesNotExist)?;

        tracing::debug!("Ensuring competition actually exists");
        let competition = self
            .competition_repo
            .competition_by_id(competition_id)
            .await
            .context("Failed to fetch competition from repository")?
            .ok_or(RegisterForCompetitionsError::CompetitionDoesNotExist)?;

        tracing::debug!("Ensuring participant is eligible for the competition");
        if participant.gender != competition.gender {
            tracing::debug!(
                participant_gender = ?participant.gender,
                competition_gender = ?competition.gender,
                "Tried to register for a competition with different gender than participant"
            );
            return Err(RegisterForCompetitionsError::NotEligible);
        }

        tracing::debug!("Loading existing registrations for participant");
        let registrations = self
            .registration_repo
            .registrations_of_participant(participant_id)
            .await
            .context("Failed to fetch registrations for participant from repository")?;

        let already_registered = registrations
            .iter()
            .any(|r| r.competition_id == competition_id);

        if already_registered {
            tracing::debug!("Participant is already registered for the competition");
            return Err(RegisterForCompetitionsError::AlreadyRegistered);
        }

        let registration_id = self
            .registration_repo
            .create_registration(participant_id, competition_id)
            .await
            .context("Failed to create registration in repository")?;

        Ok(registration_id)
    }

    /// Unregister from a competition.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration
    pub async fn unregister_from_competition(
        &self,
        registration_id: Uuid,
    ) -> Result<(), UnregisterFromCompetitionError> {
        tracing::debug!("Deleting registration result from repository");
        self.registration_repo
            .delete_result_for_registration(registration_id)
            .await
            .context("Failed to delete registration result from repository")?;

        tracing::debug!("Deleting registration from repository");
        self.registration_repo
            .delete_registration(registration_id)
            .await
            .context("Failed to delete registration from repository")?
            .ok_or(UnregisterFromCompetitionError::RegistrationDoesNotExist)?;

        Ok(())
    }
}
