use anyhow::Result;
use tracing::instrument;
use uuid::Uuid;

use crate::db;

pub struct ResultService {
    participant_repo: db::participants::Repository,
    registration_repo: db::registrations::Repository,
    competition_repo: db::competitions::Repository,
}

impl ResultService {
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
    ///
    /// # Returns:
    /// - `Ok(Some(())` - if the result was entered
    /// - `Ok(None)` - if the registration does not exist
    /// - `Err(e)` - in case of an error
    #[instrument(skip(self))]
    pub async fn enter_result_for_registration(
        &self,
        registration_id: Uuid,
        disqualified: bool,
        time_millis: u32,
    ) -> Result<Option<()>> {
        if self
            .registration_repo
            .result_for_registration(registration_id)
            .await?
            .is_some()
        {
            tracing::info!("Tried to add result for registration, that already has a result");
            return Ok(None);
        }

        if self
            .registration_repo
            .registration_by_id(registration_id)
            .await?
            .is_none()
        {
            tracing::info!("Tried to add result for registration, that does not exist");
            return Ok(None);
        }

        self.registration_repo
            .create_registration_result(registration_id, time_millis as _, disqualified)
            .await?;

        Ok(Some(()))
    }

    /// Delete a result for a registration.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration
    ///
    /// # Returns:
    /// - `Ok(Some(())` - if the result was deleted
    /// - `Ok(None)` - if the result does not exist
    /// - `Err(e)` - in case of an error
    #[instrument(skip(self))]
    pub async fn delete_result(&self, registration_id: Uuid) -> Result<Option<()>> {
        self.registration_repo
            .delete_result_for_registration(registration_id)
            .await
    }
}
